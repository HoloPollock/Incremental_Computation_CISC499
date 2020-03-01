use std::fmt;
use rand::{
    prelude::*,
    distributions::{Distribution, Standard},
    Rng,
};
pub trait Calculable {
    fn calc(&mut self) -> isize;
    fn ignore_memo_calc(&mut self) -> isize;
}
#[derive(Debug)]
enum Choice {
    Left,
    Right,
    Op,
}
#[derive(Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Val,
}

#[derive(Debug)]
pub struct Node {
    pub operation: Operation,
    pub value: Option<isize>,
    pub children: Vec<Node>,
}

impl Distribution<Operation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Operation {
        match rng.gen_range(0,4) {
            0 => Operation::Add,
            1 => Operation::Sub,
            2 => Operation::Mul,
            _ => Operation::Div,
        }
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


impl Node {
    pub fn new_val(val: isize) -> Self {
        Node {
            operation: Operation::Val,
            value: Some(val),
            children: vec![],
        }
    }
    fn modify_and_calc(&mut self) {
        let mut rng = thread_rng();
        let change: Choice = rng.gen();
        match change {

        }        
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.operation {
            Operation::Val => write!(f, "{}", self.value.unwrap()),
            _ => {
                if self.children.len() == 2 {
                    match self.value {
                        Some(val) => write!(
                            f,
                            "<{} {}>([{}] [{}])",
                            self.operation, val, self.children[0], self.children[1]
                        ),
                        None => write!(
                            f,
                            "<{}>([{}] [{}])",
                            self.operation, self.children[0], self.children[1]
                        ),
                    }
                } else {
                    panic!("Wrong Number of Children")
                }
            }
        }
    }
}

impl Calculable for Node {
    // This is kinda whack because it modified but doesnt sound like it should
    fn calc(&mut self) -> isize {
        match self.operation {
            Operation::Add => match self.value {
                Some(val) => {
                    return val;
                }
                None => {
                    if self.children.len() == 2 {
                        let result = self.children[0].calc() + self.children[1].calc();
                        self.value = Some(result);
                        return result;
                    } else {
                        panic!("Incorrect number of children");
                    }
                }
            },
            Operation::Sub => match self.value {
                Some(val) => {
                    return val;
                }
                None => {
                    if self.children.len() == 2 {
                        let result = self.children[0].calc() - self.children[1].calc();
                        self.value = Some(result);
                        return result;
                    } else {
                        panic!("Incorrect number of children");
                    }
                }
            },
            Operation::Mul => match self.value {
                Some(val) => {
                    return val;
                }
                None => {
                    if self.children.len() == 2 {
                        let result = self.children[0].calc() * self.children[1].calc();
                        self.value = Some(result);
                        return result;
                    } else {
                        panic!("Incorrect number of children");
                    }
                }
            },
            Operation::Div => match self.value {
                Some(val) => {
                    return val;
                }
                None => {
                    if self.children.len() == 2 {
                        let result = self.children[0].calc() / self.children[1].calc();
                        self.value = Some(result);
                        return result;
                    } else {
                        panic!("Incorrect numver of children");
                    }
                }
            },
            Operation::Val => match self.value {
                Some(val) => return val,
                None => panic!("Val Op has no val set"),
            },
        }
    }
    fn ignore_memo_calc(&mut self) -> isize {
        unimplemented!();
        // match self.operation {

        // }
    }
}
