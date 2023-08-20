use rand::Rng;

use crate::{bet::{Bet, BetType}, round::Round};

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

    pub fn place_bet<'round_lt, 'gambler_lt>(
        &'gambler_lt self,
        value: u32,
        round: &'round_lt Round,
        bet_type: BetType
    ) -> Bet<'round_lt, 'gambler_lt> {
        Bet::new(value, &round, &self, bet_type)
    }
}
