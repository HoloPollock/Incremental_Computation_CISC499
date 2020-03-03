use rand::prelude::*;
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

use crate::{
    choice::{Choice, OpChoice},
    operations::Operation,
};

pub trait Calculable {
    fn calc(&mut self) -> isize;
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
    pub fn rand_val() -> Self {
        Node {
            operation: Operation::Val,
            value: Some(rand_num()),
            children: vec![],
        }
    }
    fn is_val(&self) -> bool {
        return if self.children.len() == 0 {
            true
        } else {
            false
        };
    }
    pub fn gen_op() -> Self {
        let op = Operation::rand_op();
        Node {
            operation: op,
            value: None,
            children: vec![
                match OpChoice::rand() {
                    OpChoice::Op => Self::gen_op(),
                    OpChoice::Val => Self::rand_val(),
                },
                match OpChoice::rand() {
                    OpChoice::Op => Self::gen_op(),
                    OpChoice::Val => Self::rand_val(),
                },
            ],
        }
    }
    // TODO encaplsulate in not stupid and public function return something used in the recursion
    pub fn random_modify_and_calc(&mut self) -> bool {
        if self.is_val() {
            let new_val = rand_num();
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
        }
    }
    // TODO encaplsulate in not stupid and public function return something used in the recursion
    pub fn define_modify_and_calc(&mut self, mut path: Vec<Choice>, modification: &str) -> bool {
        if self.is_val() {
            let new_val = modification.parse::<isize>().unwrap();
            self.value = Some(new_val);
            true
        } else {
            let choice = path.pop().unwrap();
            match choice {
                Choice::Left => {
                    //Modify and calc but move left if calc is the same return
                    let is_change = self.children[0].define_modify_and_calc(path, modification);
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
                    let is_change = self.children[1].define_modify_and_calc(path, modification);
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
                    let new_op = modification.parse::<Operation>().unwrap(); //debatebly good fix error type
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

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
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

fn rand_num() -> isize {
    let mut rng = thread_rng();
    rng.gen_range(0, 101)
}
