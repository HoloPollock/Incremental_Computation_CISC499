use rand::{
    distributions::{Distribution, Standard},
    prelude::*,
    Rng,
};
use std::{fmt, str::FromStr};

use crate::error::ParseOperationError;

#[derive(Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Val,
}

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Sub),
            "*" => Ok(Operation::Mul),
            "/" => Ok(Operation::Div),
            _ => Err(ParseOperationError {}),
        }
    }
}

impl Operation {
    pub fn rand_op() -> Self {
        let mut rng = thread_rng();
        return rng.gen();
    }
}

impl Distribution<Operation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Operation {
        match rng.gen_range(0, 4) {
            0 => Operation::Add,
            1 => Operation::Sub,
            2 => Operation::Mul,
            _ => Operation::Div,
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Sub => write!(f, "-"),
            Operation::Mul => write!(f, "*"),
            Operation::Div => write!(f, "/"),
            Operation::Val => write!(f, "v"),
        }
    }
}
