
#[derive(Debug)]
pub struct Guess {
    value: usize,
}

impl Guess {
    pub fn new(value: usize) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}
