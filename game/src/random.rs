use std::{cell::RefCell, fs::File, io::Read};

struct Random {
    seed: u64,
    multiplier: u64,
    modulus: u64,
    increment: u64,
}

thread_local! {
    static RNG: RefCell<Random> = RefCell::new(Random::new());
}

pub fn next() -> u64 {
    RNG.with_borrow_mut(|rng| rng.next())
}

pub fn next_in_range_f64(range: std::ops::Range<f64>) -> f64 {
    let value = next() as f64;
    let percentage = value / RNG.with_borrow(|x| x.modulus as f64);
    let span = range.end - range.start;
    range.start + percentage * span
}

impl Random {
    pub fn new() -> Self {
        let mut file = File::open("/dev/urandom").unwrap();
        let mut bytes = [0, 0, 0, 0, 0, 0, 0, 0];
        file.read_exact(&mut bytes).unwrap();
        let modulus = 2u64.pow(31);
        let multiplier = 1103515245;
        let increment = 12345;
        let seed = u64::from_ne_bytes(bytes) % modulus;
        Self {
            seed,
            modulus,
            multiplier,
            increment,
        }
    }

    pub fn next(&mut self) -> u64 {
        self.seed = (self.multiplier * self.seed + self.increment) % self.modulus;
        self.seed
    }
}
