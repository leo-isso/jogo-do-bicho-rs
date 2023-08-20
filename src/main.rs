pub mod bet;
pub mod gambler;
pub mod round;

use crate::bet::{BetType};
use crate::gambler::Gambler;
use crate::round::Round;

fn main() {
    let round = Round::new();

    let gambler_1 = Gambler::new(String::from("Jhon"));

    let gambler_2 = Gambler::new(String::from("Jack"));

    let bet_1 = gambler_1.place_bet(1000, &round, BetType::Bicho);

    let bet_2 = gambler_2.place_bet(1000, &round, BetType::Bicho);
}
