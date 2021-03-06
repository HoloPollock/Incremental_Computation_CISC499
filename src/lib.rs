#![recursion_limit = "2048"]

pub mod operations;

mod error;

pub mod choice;

pub mod node;

pub mod list;

mod test {
    #[test]
    fn test_no_op() {
        use crate::{
            choice::Choice,
            node::{Calculable, Node},
            operations::Operation,
        };
        let mut tree = Node {
            operation: Operation::Add,
            value: None,
            children: vec![
                Node::new_val(3),
                Node {
                    operation: Operation::Sub,
                    value: None,
                    children: vec![Node::new_val(1), Node::new_val(0)],
                },
            ],
        };
    
        let vec_changes = vec![Choice::Right, Choice::Op];
        let modi = "+";
        tree.calc();
        let did_change = tree.internal_define_modify_and_calc(vec_changes.as_slice(), modi);
        assert!(!did_change);
    }
}