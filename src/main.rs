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

struct Bet<'round, 'gambler> {
    id: String,
    value: u32,
    round: &'round Round,
    gambler: &'gambler Gambler,
    bet_type: BetType,
}

enum BetType {
    Bicho,
    Quina,
    Milhar,
} // Default, Quina, Milhar + validacoes

fn main() {
    let round = Round::new();

    let gambler_1 = Gambler {
        id: String::from("1"),
        name: String::from("Jhon"),
    };

    let gambler_2 = Gambler {
        id: String::from("2"),
        name: String::from("Max"),
    };

    let bet_1 = Bet {
        id: String::from("1"),
        value: 1000,
        round: &round,
        gambler: &gambler_1,
        bet_type: BetType::Bicho,
    };

    let bet_2 = Bet {
        id: String::from("2"),
        value: 1000,
        round: &round,
        gambler: &gambler_2,
        bet_type: BetType::Bicho,
    };
}
