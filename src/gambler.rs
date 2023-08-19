use rand::Rng;

pub struct Gambler {
    pub id: String,
    pub name: String,
}

impl Gambler {
    pub fn new(name: String) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: rng.gen::<u32>().to_string(),
            name,
        }
    }
}
