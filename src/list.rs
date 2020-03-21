
use crate::node::{Node, Calculable};
use rbtree::RBTree;
#[derive(Debug)]
pub struct NodeList {
    pub list: Vec<Node>,
    pub tree: RBTree<Node, ()> // Cant Use BtreeSet as doesnt accept duplicate values dont use and get style functions
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
    pub fn modify_first_element(){
        unimplemented!()
    }
    pub fn deifned_modify_first_element(){
        unimplemented!();
    }
    fn modify_and_sort() {
        unimplemented!()
    }

    pub fn sort(&mut self) {
        for i in self.list.iter_mut() {
            if i.value == None {
                i.calc();
            }
            self.tree.insert(i.clone(),());
        }
        
    }
    pub fn gen_random(size: isize) -> Self {
        let mut list: Self = Default::default();
        for _i in 0..size {
            list.list.push(Node::gen_op());
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