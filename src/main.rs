pub mod bet;
pub mod gambler;
pub mod round;
use crate::bet::{Bet, BetType};
use crate::gambler::Gambler;
use crate::round::Round;

fn main() {
    let round = Round::new();

    let gambler_1 = Gambler::new(String::from("Jhon"));

    let gambler_2 = Gambler::new(String::from("Jack"));

    let bet_1 = Bet::new(1000, &round, &gambler_1, BetType::Bicho);

    let bet_2 = Bet::new(1000, &round, &gambler_2, BetType::Bicho);
}
