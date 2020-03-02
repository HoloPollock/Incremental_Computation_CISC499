use rand::{
    distributions::{Distribution, Standard},
    prelude::*,
    Rng,
};
use std::fmt;

use crate::error::ParseOperationError;

pub trait Calculable {
    fn calc(&mut self) -> isize;
}

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

#[derive(Debug)]
pub struct Node {
    pub operation: Operation,
    pub value: Option<isize>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new_val(val: isize) -> Self {
        Node {
            operation: Operation::Val,
            value: Some(val),
            children: vec![],
        }
    }
    pub fn is_val(&self) -> bool {
        return if self.children.len() == 0 {
            true
        } else {
            false
        };
    }
    // TODO encaplsulate in not stupid and public function return something used in the recursion
    pub fn random_modify_and_calc(&mut self) -> bool {
        if self.is_val() {
            let mut rng = thread_rng();
            let new_val = rng.gen_range(0, 101);
            self.value = Some(new_val);
            true
        } else {
            match Choice::rand_choice() {
                Choice::Left => {
                    //Modify and calc but move left if calc is the same return
                    let is_change = self.children[0].random_modify_and_calc();
                    match is_change {
                        true => {
                            dbg!(self.value);
                            let prev_val = self.value.unwrap(); //May be none deal with
                            self.value = None;
                            let new_val = self.calc();
                            if new_val == prev_val {
                                dbg!("No Change");
                                return false; // dont need to set as calc resets
                            } else {
                                dbg!(self.value);
                                return true;
                            }
                        }
                        false => {
                            return false;
                        }
                    }
                }
                Choice::Right => {
                    let is_change = self.children[1].random_modify_and_calc();
                    match is_change {
                        true => {
                            dbg!(self.value);
                            let prev_val = self.value.unwrap(); //May be none deal with
                            self.value = None;
                            let new_val = self.calc();
                            if new_val == prev_val {
                                dbg!("No Change");
                                return false; // dont need to set as calc resets
                            } else {
                                dbg!(self.value);
                                return true;
                            }
                        }
                        false => {
                            return false;
                        }
                    }
                }
                Choice::Op => {
                    let new_op = Operation::rand_op();
                    self.operation = new_op;
                    dbg!(&self.value);
                    let prev_val = self.value.unwrap();
                    self.value = None;
                    let new_val = self.calc();
                    if prev_val == new_val {
                        dbg!("No Change");
                        return false;
                    } else {
                        dbg!(self.value);
                        return true;
                    }
                }
            }
        } else {
            //if it is a val
            let mut rng = thread_rng();
            let new_val = rng.gen_range(0, 101);
            self.value = Some(new_val);
            true
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
                            "(<{} {}> {} {})",
                            self.operation, val, self.children[0], self.children[1]
                        ),
                        None => write!(
                            f,
                            "(<{}> {} {})",
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
                    // dbg!("resuinging Val", val);
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
                    // dbg!("resuinging Val", val);
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
                    // dbg!("resuinging Val", val);
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
                    // dbg!("resuinging Val", val);
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
}
