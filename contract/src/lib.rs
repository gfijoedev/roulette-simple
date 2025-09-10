use std::str::FromStr;

use hex::encode;
use near_sdk::{
    env::{self},
    ext_contract,
    json_types::U128,
    log, near, require, serde, serde_json,
    store::LookupMap,
    AccountId, Gas, NearToken, Promise, PromiseError, PromiseOrValue,
};
use omni_transaction::signer::types::SignatureResponse;

mod chain_signature;
mod ft;
pub mod roulette;

// TODO make enum for inside/outside/call bet types
// see notes/european-roulette-bets.txt

#[allow(dead_code)]
#[ext_contract(my_contract)]
trait MyContract {
    fn mpc_callback(
        &mut self,
        sender_id: AccountId,
        spins: Vec<Vec<roulette::Bet>>,
        token_id: AccountId,
    );
}

#[near(contract_state)]
pub struct Contract {
    spins: u128,
    bets: u128,
    house: u128,
    payout: u128,
    // fts
    balances: LookupMap<String, LookupMap<AccountId, u128>>,
}

impl Default for Contract {
    fn default() -> Self {
        let mut this = Self {
            spins: 0,
            bets: 0,
            house: 100_000_000_000_000_000_000_000_000,
            payout: 0,
            balances: LookupMap::new(b"a"),
        };

        this.balances
            .set("usdc.fakes.testnet".to_owned(), Some(LookupMap::new(b"b")));

        this
    }
}

#[near]
impl Contract {
    pub fn stats(&self) -> (U128, U128, U128, U128) {
        (
            U128(self.spins),
            U128(self.bets),
            U128(self.house),
            U128(self.payout),
        )
    }

    #[payable]
    pub fn spin_with_near(&mut self, spins: Vec<Vec<roulette::Bet>>, callback_tgas: u8) -> Promise {
        let amount = env::attached_deposit();
        let sender_id = env::predecessor_account_id();
        self.spin(
            spins,
            sender_id,
            amount.as_yoctonear(),
            AccountId::from_str("near").unwrap(),
            callback_tgas,
        )
    }

    #[private]
    pub fn mpc_callback(
        &mut self,
        #[callback_result] call_result: Result<SignatureResponse, PromiseError>,
        sender_id: AccountId,
        spins: Vec<Vec<roulette::Bet>>,
        token_id: AccountId, // payout token
    ) -> Vec<Vec<(bool, u8, bool, u8)>> {
        let mut results: Vec<Vec<(bool, u8, bool, u8)>> = vec![];
        match call_result {
            Ok(signature_response) => {
                // get bytes from signature
                let mut r_bytes =
                    hex::decode(signature_response.big_r.affine_point).expect("r_bytes failed");
                // first r_byte is compression flag
                r_bytes.remove(0);
                // all byte of s_bytes should be random for signature s is scalar value
                let mut s_bytes = hex::decode(signature_response.s.scalar).expect("s_bytes failed");
                s_bytes.extend(r_bytes);

                let mut payout: u128 = 0;

                for (i, bets) in spins.iter().enumerate() {
                    let mut spin_result = vec![];
                    for bet in bets {
                        let (win, number, red, multiple) = roulette::bet_eval(s_bytes[i], bet);

                        let amount = bet.amount.as_yoctonear();
                        if multiple > 0 {
                            payout = payout
                                .checked_add(
                                    amount
                                        .checked_mul((multiple + 1) as u128)
                                        .expect("payout overflow"),
                                )
                                .expect("payout overflow");
                        }

                        spin_result.push((win, number, red, multiple));
                    }
                    results.push(spin_result);
                }

                self.house = self.house.checked_sub(payout).expect("house empty");
                self.payout = self.payout.checked_add(payout).expect("paid overflow");

                match token_id.as_str() {
                    "near" => Promise::new(sender_id).transfer(NearToken::from_yoctonear(payout)),
                    _ => ft::ft_contract::ext(token_id)
                        .with_static_gas(Gas::from_tgas(50))
                        .with_attached_deposit(NearToken::from_yoctonear(1))
                        .ft_transfer(sender_id, U128(payout), None),
                };

                results
            }
            Err(error) => {
                env::log_str(&format!("mpc callback failed with error: {:?}", error));
                results.push(vec![(false, 0, false, 0)]);
                results
            }
        }
    }
}

// internal

impl Contract {
    pub fn spin(
        &mut self,
        spins: Vec<Vec<roulette::Bet>>,
        sender_id: AccountId,
        amount: u128,
        token_id: AccountId,
        callback_tgas: u8,
    ) -> Promise {
        require!(spins.len() < 64, "too many spins");

        // required_amount is all bet amounts
        let mut required_amount: u128 = 0;
        for bets in &spins {
            for bet in bets {
                required_amount = required_amount
                    .checked_add(bet.amount.as_yoctonear())
                    .expect("bet.amount overflow");

                // is bet legal
                require!(roulette::bet_legal(&bet), "illegal bet");

                self.bets += 1;
            }
            self.spins += 1;
        }

        require!(amount == required_amount, "deposit != bet amount");

        self.house = self
            .house
            .checked_add(required_amount)
            .expect("house overflow");

        // get chain signature
        let account_id = env::predecessor_account_id();
        let random_seed = env::random_seed_array();

        chain_signature::internal_request_signature(
            account_id.to_string(),
            encode(random_seed),
            "Ecdsa".to_owned(),
        )
        .then(
            my_contract::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(callback_tgas as u64))
                .mpc_callback(sender_id, spins, token_id),
        )
    }
}
