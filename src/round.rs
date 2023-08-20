use rand::Rng;

pub struct Round {
    pub draws: Option<Vec<u16>>,
    pub id: String,
    pub ready: bool,
} // generate_draws, get_prize, get_winners

impl Round {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: rng.gen::<u32>().to_string(),
            draws: None,
            ready: false,
        }
    }

    fn panic_if_not_ready(&self) {
        if !self.ready {
            panic!("Round is not ready yet!");
        }
    }
}
