use crate::*;
pub const ZERO: u128 = 0;

#[allow(dead_code)]
#[ext_contract(ft_contract)]
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

        let ft_balances = self
            .balances
            .get_mut(ft_account_id.as_str())
            .expect("token not supported");

        let mut token_balance = *ft_balances.get(&sender_id).unwrap_or(&ZERO);
        token_balance += amount.0;

        ft_balances.insert(sender_id, token_balance);

        PromiseOrValue::Value(U128(0))
    }
}

#[near]
impl Contract {
    pub fn usdc_balance(&self, account_id: AccountId) -> U128 {
        let ft_balances = self
            .balances
            .get("usdc.fakes.testnet")
            .expect("token not supported");

        let token_balance = ft_balances.get(&account_id).unwrap_or(&ZERO);

        U128(*token_balance)
    }
}
