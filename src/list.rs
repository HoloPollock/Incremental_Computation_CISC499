use crate::{
    choice::Choice,
    node::{Calculable, Node},
};
use rbtree::RBTree;
#[derive(Debug, Clone)]
pub struct NodeList {
    pub list: Vec<Node>,
    pub tree: RBTree<Node, ()>, // Cant Use BtreeSet as doesn't accept duplicate values dont use and get style functions
}

impl Default for NodeList {
    fn default() -> Self {
        NodeList {
            list: Vec::new(),
            tree: RBTree::new(),
        }
    }
}

impl NodeList {
    pub fn modify_first_element(&mut self) {
        let first_node = self.tree.pop_first();
        let new_node = match first_node {
            Some(mut node) => {
                node.0.random_modify_and_calc();
                node.0
            }
            None => panic!("empty tree"),
        };
        self.tree.insert(new_node, ());
    }
    pub fn defined_modify_first_element(&mut self, mut path: Vec<Choice>, modification: &str) {
        let first_node = self.tree.pop_first();
        let new_node = match first_node {
            Some(mut node) => {
                node.0.define_modify_and_calc(path, modification);
                node.0
            }
            None => panic!("empty tree"),
        };
        self.tree.insert(new_node, ());
    }

    pub fn modify_and_sort_from_scratch() {
        unimplemented!()
    }

    pub fn sort(&mut self) {
        for i in self.list.iter_mut() {
            if i.value == None {
                i.calc();
            }
            self.tree.insert(i.clone(), ());
        }
    }
    pub fn gen_random(size: isize) -> Self {
        let mut list: Self = Default::default();
        for _i in 0..size {
            list.list.push(Node::gen_node());
        }
        return list;
    }
    pub fn gen_random_of_depth(size: usize, n: usize) -> Self {
        let mut list: Self = Default::default();
        for _i in 0..size {
            list.list.push(Node::gen_node_of_depth(n))
        }
        return list;
    }
}
