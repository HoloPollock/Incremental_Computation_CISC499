use crate::{
    choice::Choice,
    node::{Calculable, Node},
    rbtree::RBTree,
};
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
        let mut first_node = self.tree.pop_first();
        first_node.random_modify_and_calc();
        self.tree.insert(first_node, ());
    }
    pub fn defined_modify_first_element(&mut self, path: Vec<Choice>, modification: &str) {
        let mut first_node = self.tree.pop_first();
        first_node.define_modify_and_calc(path, modification);
        self.tree.insert(first_node, ());
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
        list
    }
    pub fn gen_random_of_depth(size: usize, n: usize) -> Self {
        let mut list: Self = Default::default();
        for _i in 0..size {
            list.list.push(Node::gen_node_of_depth(n))
        }
        list
    }
}
