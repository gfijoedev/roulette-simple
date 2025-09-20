use crate::*;

#[derive(Debug, Copy, Clone)]
#[near(serializers = [json, borsh])]
pub struct Stats {
    pub spins: u128,
    pub bets: u128,
    pub house: u128,
    pub payout: u128,
}

impl Contract {
    pub fn internal_set_token(&mut self, token_id: AccountId, stats: Stats) {
        self.tokens.set(token_id, Some(stats));
    }
}
