use rand::Rng;

pub struct Round {
    id: String,
    draws: Vec<u16>,
} // generate_draws, get_prize, get_winners

impl Round {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: rng.gen::<u32>().to_string(),
            draws: Vec::new(),
        }
    }
}
