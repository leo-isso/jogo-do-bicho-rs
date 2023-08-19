use crate::gambler::Gambler;
use crate::round::Round;
use rand::Rng;

pub enum BetType {
    Bicho,
    Quina,
    Milhar,
} // Default, Quina, Milhar + validacoes

pub struct Bet<'round, 'gambler> {
    pub id: String,
    pub value: u32,
    pub round: &'round Round,
    pub gambler: &'gambler Gambler,
    pub bet_type: BetType,
}

impl<'roundl, 'gamblerl> Bet<'roundl, 'gamblerl> {
    pub fn new(
        value: u32,
        round: &'roundl Round,
        gambler: &'gamblerl Gambler,
        bet_type: BetType,
    ) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: rng.gen::<u32>().to_string(),
            value,
            round,
            gambler,
            bet_type,
        }
    }
}
