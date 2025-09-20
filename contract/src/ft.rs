use crate::*;
pub const ZERO: u128 = 0;

#[derive(serde::Deserialize, Debug)]
pub struct SpinFT {
    spins: Vec<Vec<roulette::Bet>>,
    callback_tgas: u8,
}

#[allow(dead_code)]
#[ext_contract(ft_contract)]
trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[allow(dead_code)]
#[ext_contract(ft_receiver)]
trait FungibleTokenReceiver {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[near]
impl FungibleTokenReceiver for Contract {
    // Callback on receiving tokens by this contract.
    // `msg` format is either "" for deposit or `TokenReceiverMessage`.
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let ft_account_id = env::predecessor_account_id();

        let args = serde_json::from_str::<SpinFT>(&msg).expect("WRONG_MSG_FORMAT");

        self.spin(
            args.spins,
            sender_id,
            amount.0,
            ft_account_id,
            args.callback_tgas,
        );

        PromiseOrValue::Value(U128(0))
    }
}
