pub mod bet;
pub mod gambler;
pub mod round;

use crate::bet::BetType;
use crate::gambler::Gambler;
use crate::round::Round;

fn main() {
    let mut round = Round::new();

    let gambler_1 = Gambler::new(String::from("Jhon"));

    let gambler_2 = Gambler::new(String::from("Jack"));

    let bet_1 = gambler_1.place_bet(1000, BetType::Milhar(1547));

    let bet_2 = gambler_2.place_bet(1000, BetType::Milhar(1895));
    round.draw_results();
    println!("Results:");
    for i in &round.draws {
        println!("{} are the results", i);
    }
    round.validate_results(vec![bet_1, bet_2]);
}
