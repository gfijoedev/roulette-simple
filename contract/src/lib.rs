use hex::encode;
use near_sdk::{
    env::{self},
    ext_contract,
    json_types::U128,
    near, require, AccountId, Gas, NearToken, Promise, PromiseError,
};
use omni_transaction::signer::types::SignatureResponse;

mod chain_signature;
mod roulette;

const CALLBACK_GAS: Gas = Gas::from_tgas(5);

// TODO make enum for inside/outside/call bet types
// see notes/european-roulette-bets.txt

#[allow(dead_code)]
#[ext_contract(my_contract)]
trait MyContract {
    fn mpc_callback(&self, account_id: AccountId, bets: Vec<roulette::Bet>);
}

#[near(contract_state)]
pub struct Contract {
    rounds: u128,
    house: u128,
    payout: u128,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            rounds: 0,
            house: 2_000_000_000_000_000_000_000_000,
            payout: 0,
        }
    }
}

#[near]
impl Contract {
    pub fn stats(&self) -> (U128, U128, U128) {
        (U128(self.rounds), U128(self.house), U128(self.payout))
    }

    #[payable]
    pub fn spin(&mut self, bets: Vec<roulette::Bet>) -> Promise {
        let deposit = env::attached_deposit();

        let bet = &bets[0];
        let amount = bet.amount;
        require!(deposit == amount, "Deposit must be 0.1 NEAR");

        self.house = self
            .house
            .checked_add(amount.as_yoctonear())
            .expect("house overflow");

        self.rounds += 1;

        let account_id = env::predecessor_account_id();

        let random_seed = env::random_seed_array();

        chain_signature::internal_request_signature(
            account_id.to_string(),
            encode(random_seed),
            "Ecdsa".to_owned(),
        )
        .then(
            my_contract::ext(env::current_account_id())
                .with_static_gas(CALLBACK_GAS)
                .mpc_callback(account_id, bets),
        )
    }

    #[private]
    pub fn mpc_callback(
        &mut self,
        #[callback_result] call_result: Result<SignatureResponse, PromiseError>,
        account_id: AccountId,
        bets: Vec<roulette::Bet>,
    ) -> (bool, u8, u8) {
        match call_result {
            Ok(signature_response) => {
                // get bytes from signature
                // all byte of s_bytes should be random for signature s is scalar value
                let s_bytes = hex::decode(signature_response.s.scalar)
                    .expect("failed to decode scalar to bytes");

                let bet = &bets[0];
                let (win, number, multiple) = roulette::bet_eval(s_bytes[0], bet);

                let amount = bet.amount.as_yoctonear();
                if multiple > 0 {
                    let payout = amount
                        .checked_mul(multiple as u128)
                        .expect("payout overflow")
                        .checked_add(amount)
                        .expect("payout overflow");

                    self.house = self.house.checked_sub(payout).expect("house empty");
                    self.payout = self.payout.checked_add(payout).expect("paid overflow");

                    Promise::new(account_id).transfer(NearToken::from_yoctonear(payout));
                }

                (win, number, multiple)
            }
            Err(error) => {
                env::log_str(&format!("mpc callback failed with error: {:?}", error));
                (false, 0, 0)
            }
        }
    }
}
