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
    fn calc(&mut self) -> i128;
}
#[derive(Debug, Clone)]
pub struct Node {
    pub operation: Operation,
    pub value: Option<i128>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new_val(val: i128) -> Self {
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
        self.children.is_empty()
    }
    pub fn gen_node() -> Self {
        let op = Operation::rand_op();
        Node {
            operation: op,
            value: None,
            children: vec![
                match OpChoice::rand() {
                    OpChoice::Op => Self::gen_node(),
                    OpChoice::Val => Self::rand_val(),
                },
                match OpChoice::rand() {
                    OpChoice::Op => Self::gen_node(),
                    OpChoice::Val => Self::rand_val(),
                },
            ],
        }
    }
    pub fn gen_node_of_depth(n: usize) -> Self {
        match n {
            1 => Self::rand_val(),
            _ => Node {
                operation: Operation::rand_op(),
                value: None,
                children: vec![
                    Self::gen_node_of_depth(n - 1),
                    Self::gen_node_of_depth(n - 1),
                ],
            },
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
                            //dbg!(self.value);
                            let prev_val = self.value.unwrap(); //May be none deal with
                            self.value = None;
                            let new_val = self.calc();
                            new_val != prev_val
                        }
                        false => false,
                    }
                }
                Choice::Right => {
                    let is_change = self.children[1].random_modify_and_calc();
                    match is_change {
                        true => {
                            //dbg!(self.value);
                            let prev_val = self.value.unwrap(); //May be none deal with
                            self.value = None;
                            let new_val = self.calc();
                            new_val != prev_val
                        }
                        false => false,
                    }
                }
                Choice::Op => {
                    let new_op = Operation::rand_op();
                    self.operation = new_op;
                    //dbg!(&self.value);
                    let prev_val = self.value.unwrap();
                    self.value = None;
                    let new_val = self.calc();
                    prev_val != new_val
                }
            }
        }
    }

    pub fn define_modify_and_calc(&mut self, path: Vec<Choice>, modification: &str){
        self.internal_define_modify_and_calc(path.as_slice(), modification);
    } 
    // TODO encaplsulate in not stupid and public function return something used in the recursion
    pub (crate) fn internal_define_modify_and_calc(&mut self, path: &[Choice], modification: &str) -> bool {
        if self.is_val() {
            let new_val = modification.parse::<i128>().unwrap();
            self.value = Some(new_val);
            true
        } else {
            // let choice = path.pop().unwrap();
            match path {
                [Choice::Left, end @ ..] => {
                    //Modify and calc but move left if calc is the same return
                    let is_change = self.children[0].internal_define_modify_and_calc(end, modification);
                    match is_change {
                        true => {
                            //dbg!(self.value);
                            let prev_val = self.value.unwrap(); //May be none deal with
                            self.value = None;
                            let new_val = self.calc();
                            new_val != prev_val
                        }
                        false => false,
                    }
                }
                [Choice::Right, end @ ..] => {
                    let is_change = self.children[1].internal_define_modify_and_calc(end, modification);
                    match is_change {
                        true => {
                            //dbg!(self.value);
                            let prev_val = self.value.unwrap(); //May be none deal with
                            self.value = None;
                            let new_val = self.calc();
                            new_val != prev_val
                        }
                        false => false,
                    }
                }
                [Choice::Op, ..] => {
                    let new_op = modification.parse::<Operation>().unwrap(); //debatebly good fix error type
                    self.operation = new_op;
                    //dbg!(&self.value);
                    let prev_val = self.value.unwrap();
                    self.value = None;
                    let new_val = self.calc();
                    prev_val != new_val
                }
                [] => {
                    panic!("List Empty and not at val");
                }
            }
        }
    }

    pub fn define_modify(&mut self, path: Vec<Choice>, modification: &str) {
        self.internal_define_modify(path.as_slice(), modification);
    }

    fn internal_define_modify(&mut self, path: &[Choice], modification: &str) {
        if self.is_val() {
            let new_val = modification.parse::<i128>().unwrap();
            self.value = Some(new_val);
        } else {
            match path {
                [Choice::Left, end @ ..] => {
                    //Modify and calc but move left if calc is the same return
                    self.children[0].internal_define_modify(end, modification);
                }
                [Choice::Right, end @ ..] => {
                    self.children[1].internal_define_modify(end, modification);
                }
                [Choice::Op,..] => {
                    let new_op = modification.parse::<Operation>().unwrap(); //debatebly good fix error type
                    self.operation = new_op;
                }
                [] => {
                    panic!("List Empty and not at val");
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
        match self.value.cmp(&other.value) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match self.operation.cmp(&other.operation) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => self.children.cmp(&other.children),
            },
        }
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
    fn calc(&mut self) -> i128 {
        match self.operation {
            Operation::Add => match self.value {
                Some(val) => val,
                None => {
                    if self.children.len() == 2 {
                        let result = self.children[0].calc() + self.children[1].calc();
                        self.value = Some(result);
                        result
                    } else {
                        panic!("Incorrect number of children");
                    }
                }
            },
            Operation::Sub => match self.value {
                Some(val) => val,
                None => {
                    if self.children.len() == 2 {
                        let result = self.children[0].calc() - self.children[1].calc();
                        self.value = Some(result);
                        result
                    } else {
                        panic!("Incorrect number of children");
                    }
                }
            },
            Operation::Mul => match self.value {
                Some(val) => val,
                None => {
                    if self.children.len() == 2 {
                        let result = self.children[0].calc() * self.children[1].calc();
                        self.value = Some(result);
                        result
                    } else {
                        panic!("Incorrect number of children");
                    }
                }
            },
            Operation::Div => match self.value {
                //For convince and not having dealing wiht undefined divinding by zero = zero
                Some(val) => val,
                None => {
                    if self.children.len() == 2 {
                        let denom = self.children[1].calc();
                        let numon = self.children[0].calc();
                        if denom == 0 {
                            self.value = Some(0);
                            0
                        } else {
                            let result = numon / denom;
                            self.value = Some(result);
                            result
                        }
                    } else {
                        panic!("Incorrect numver of children");
                    }
                }
            },
            Operation::Val => match self.value {
                Some(val) => val,
                None => panic!("Val Op has no val set"),
            },
        }
    }
}

fn rand_num() -> i128 {
    let mut rng = thread_rng();
    rng.gen_range(0, 101)
}
