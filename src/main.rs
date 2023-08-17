#[derive(Debug)]
struct Round {
    id: String,
    draws: Vec<u16>,
} // generate_draws, get_prize, get_winners

#[derive(Debug)]
struct Gambler {
    id: String,
    name: String,
}

#[derive(Debug)]
struct Bet<'round, 'gambler> {
    id: String,
    value: u32,
    round: &'round Round,
    gambler: &'gambler Gambler,
    bet_type: BetType,
}

#[derive(Debug)]
enum BetType {
    Bicho,
    Quina,
    Milhar,
} // Default, Quina, Milhar + validacoes

fn main() {
    let round = Round {
        id: String::from("someround"),
        draws: Vec::new(),
    };

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

    println!("{:#?}", bet_1);
    println!("{:#?}", bet_2);
}
