#[allow(dead_code)]
pub struct Rng {
    pub state: u64,
}

impl Rng {
    pub fn new(seed: u32) -> Self {
        Self { state: seed as u64 }
    }

    pub fn random_u32(&mut self) -> u32 {
        // xorshift rng: https://en.wikipedia.org/wiki/Xorshift#xorshift.2A
        // doing & 0xFFFFFFFFFFFFFFFF is the same as cast to uint64 in C
        // doing & 0xFFFFFFFF is the same as cast to uint32 in C
        self.state ^= (self.state >> 12) & 0xFFFFFFFFFFFFFFFF;
        self.state ^= (self.state << 25) & 0xFFFFFFFFFFFFFFFF;
        self.state ^= (self.state >> 27) & 0xFFFFFFFFFFFFFFFF;
        return (((self.state.wrapping_mul(0x2545F4914F6CDD1D)) >> 32) & 0xFFFFFFFF) as u32;
    }

    pub fn random(&mut self) -> f32 {
        return (self.random_u32() >> 8) as f32 / 16777216.0;
    }

    pub fn uniform(&mut self, a: Option<f32>, b: Option<f32>) -> f32 {
        let a = a.unwrap_or(0.0);
        let b = b.unwrap_or(1.0);
        return a + (b - a) * self.random();
    }
}
