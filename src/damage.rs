use macroquad::rand::{gen_range, RandomRange};


pub struct DmgRange<T: RandomRange + Copy> {
    low: T,
    high: T,
}

impl<T: RandomRange + Copy> DmgRange<T> {
    pub fn new(low: T, high: T) -> Self {
        Self { low, high }
    }

    pub fn generate(&self) -> T {
    gen_range(self.low, self.high)
    }
}