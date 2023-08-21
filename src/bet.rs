use crate::gambler::Gambler;
use crate::round::Round;
use rand::Rng;

pub enum BetType {
    Bicho,
    Quina,
    Milhar(u16),
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

    // pub fn validate_bicho(draws: Vec<u16>) {}
    // pub fn validate_quina(draws: Vec<u16>) {}
    fn validate_milhar(value: u16, draws: Vec<u16>) -> bool {
        for draw in &draws {
           if *draw == value {
               return true
           }
        }
        false
    }

    pub fn validate(&self, draws: Vec<u16>) -> bool {
        match self.bet_type {
            BetType::Bicho => panic!("not implemented yet"),
            BetType::Quina => panic!("not implemented yet"),
            BetType::Milhar(value) => Bet::validate_milhar(value, draws),            
        }
    }
}
