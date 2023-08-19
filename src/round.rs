use rand::Rng;

pub struct Round {
    pub draws: Vec<u16>,
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
}
