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
    pub(crate) fn rand_choice() -> Self {
        let mut rng = thread_rng();
        rng.gen()
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
#[derive(Debug)]
pub enum OpChoice {
    Op,
    Val,
}

impl OpChoice {
    pub(crate) fn rand() -> Self {
        let mut rng = thread_rng();
        rng.gen()
    }
}

impl Distribution<OpChoice> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> OpChoice {
        match rng.gen_range(0, 2) {
            0 => OpChoice::Op,
            _ => OpChoice::Val,
        }
    }
}
