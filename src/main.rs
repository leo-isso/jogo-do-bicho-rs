struct Round {
    id: String,
    draws: Vec<u16>,
} // generate_draws, get_prize, get_winners

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
    println!("Hello, world!");
}
