use rand::Rng;

use crate::bet::Bet;

pub struct Round {
    pub draws: Vec<String>,
    pub id: String,
} // generate_draws, get_prize, get_winners

impl Round {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: rng.gen::<u32>().to_string(),
            draws: Vec::new(),
        }
    }

    pub fn draw_results(&mut self) {
        let mut rng = rand::thread_rng();
        let mut draws = Vec::new();
        for _ in 0..4 {
            let number = rng.gen_range(0000..9999);
            draws.push(
                String::from(format!("{:0zbefore$}", number, zbefore=4))
            )
        }
        self.draws = draws;
    }

    pub fn validate_results(&self, bets: Vec<Bet>) {
        let mut winners: Vec<String> = Vec::new();
        
        for bet in &bets {
            if bet.validate(&self.draws) {
                winners.push(String::from(&bet.gambler.name)); 
            } 
        }

        Round::announce_results(&winners);
    }

    fn announce_results(winners: &Vec<String>) {
        if winners.len() < 1 {
            println!("No one won!")
        } else {
            for winner in winners {
                println!("{} was a winner!", winner)
            } 
        }
    }

}
