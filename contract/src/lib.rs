use hex::encode;
use near_sdk::{
    env::{self},
    ext_contract,
    json_types::U128,
    near, require,
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
    fn mpc_callback(&mut self, account_id: AccountId, spins: Vec<Vec<roulette::Bet>>);
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
    pub fn spin(&mut self, spins: Vec<Vec<roulette::Bet>>, callback_gas: u8) -> Promise {
        require!(spins.len() < 64, "too many spins");

        let deposit = env::attached_deposit();

        // required_deposit is all bet amounts
        let mut required_deposit: u128 = 0;
        for bets in &spins {
            for bet in bets {
                required_deposit = required_deposit
                    .checked_add(bet.amount.as_yoctonear())
                    .expect("bet.amount overflow");

                // is bet legal
                require!(roulette::bet_legal(&bet), "illegal bet");

                self.bets += 1;
            }
            self.spins += 1;
        }

        require!(
            deposit.as_yoctonear() == required_deposit,
            "deposit != bet amount"
        );

        self.house = self
            .house
            .checked_add(required_deposit)
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
                .with_static_gas(Gas::from_tgas(callback_gas as u64))
                .mpc_callback(account_id, spins),
        )
    }

    #[private]
    pub fn mpc_callback(
        &mut self,
        #[callback_result] call_result: Result<SignatureResponse, PromiseError>,
        account_id: AccountId,
        spins: Vec<Vec<roulette::Bet>>,
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
                Promise::new(account_id).transfer(NearToken::from_yoctonear(payout));

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
