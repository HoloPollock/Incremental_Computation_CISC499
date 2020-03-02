use rand::{
    distributions::{Distribution, Standard},
    prelude::*,
    Rng,
};
#[derive(Debug)]
pub enum Choice {
    Left,
    Right,
    Op,
}

impl Choice {
    pub fn rand_choice() -> Self {
        let mut rng = thread_rng();
        return rng.gen();
    }
}

impl Distribution<Choice> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Choice {
        match rng.gen_range(0, 3) {
            0 => Choice::Left,
            1 => Choice::Right,
            _ => Choice::Op,
        }
    }
}
