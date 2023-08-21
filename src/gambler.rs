use rand::Rng;

use crate::bet::{Bet, BetType};

pub struct Gambler {
    pub id: String,
    pub name: String,
}

impl Gambler {
    pub fn new(name: String) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: rng.gen::<u32>().to_string(),
            name,
        }
    }

    pub fn place_bet<'gambler_lt>(
        &'gambler_lt self,
        value: u32,
        bet_type: BetType
    ) -> Bet<'gambler_lt> {
        Bet::new(value, &self, bet_type)
    }
}
