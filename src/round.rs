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

    pub fn draw_results(&mut self) {
        let mut rng = rand::thread_rng();
        let mut draws = Vec::new();
        for _ in 0..3 {
            draws.push(rng.gen_range(0000..9999))
        }
        self.draws = Some(draws);
        self.ready = true;
    }
}
