use rand::Rng;

struct Round {
    id: String,
    draws: Vec<u16>,
} // generate_draws, get_prize, get_winners

impl Round {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: rng.gen::<u32>().to_string(),
            draws: Vec::new(),
        }
    }
}

struct Gambler {
    id: String,
    name: String,
}

impl Gambler {
    fn new(name: String) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: rng.gen::<u32>().to_string(),
            name,
        }
    }
}

struct Bet<'round, 'gambler> {
    id: String,
    value: u32,
    round: &'round Round,
    gambler: &'gambler Gambler,
    bet_type: BetType,
}

impl<'roundl, 'gamblerl> Bet<'roundl, 'gamblerl> {
    fn new(
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

enum BetType {
    Bicho,
    Quina,
    Milhar,
} // Default, Quina, Milhar + validacoes

fn main() {
    let round = Round::new();

    let gambler_1 = Gambler::new(String::from("Jhon"));

    let gambler_2 = Gambler::new(String::from("Jack"));

    let bet_1 = Bet::new(1000, &round, &gambler_1, BetType::Bicho);

    let bet_2 = Bet::new(1000, &round, &gambler_2, BetType::Bicho);
}
