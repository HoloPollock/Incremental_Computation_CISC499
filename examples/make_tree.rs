use incremental_tree::{
    self,
    node::{Calculable, Node},
    operations::Operation,
    list::NodeList,
    choice::Choice
};
use std::time::{Instant};
use rand::thread_rng;
use rand::prelude::*;



fn main() {
    dbg!("genning");
    let mut tree_test = NodeList::gen_random_of_depth(50000,8);
    let now = Instant::now();
    tree_test.sort();
    let done = Instant::now();
    dbg!(done.duration_since(now));
    // let mut tree = Node::gen_node_of_depth(3);
    // println!("{}", tree);
    // tree.define_modify(vec![Choice::Right, Choice::Left], "10");
    // println!("{}", tree);
    // let mut tree = Node::gen_node_of_depth(8);
    // println!("{:?}", tree_test.list);
    // dbg!("running");
    // let now = Instant::now();
    // tree_test.sort();
    // let full_sort = Instant::now();
    // let mut rng = thread_rng();
    // dbg!("Shuffling");
    // tree_test.list.shuffle(&mut rng);
    // dbg!("re_sort");
    // let pre_re = Instant::now();
    // tree_test.sort();
    // let re_sort = Instant::now();

    // dbg!(full_sort.duration_since(now));
    // dbg!(re_sort.duration_since(pre_re));



        // let mut tree = Node {
    //     operation: Operation::Add,
    //     value: None,
    //     children: vec![
    //         Node {
    //             operation: Operation::Add,
    //             value: None,
    //             children: vec![
    //                 Node::new_val(6),
    //                 Node {
    //                     operation: Operation::Div,
    //                     value: None,
    //                     children: vec![Node::new_val(25), Node::new_val(12)],
    //                 },
    //             ],
    //         },
    //         Node {
    //             operation: Operation::Mul,
    //             value: None,
    //             children: vec![Node::new_val(3), Node::new_val(4)],
    //         },
    //     ],
    // };

    // println!("{}", tree);

    // dbg!(tree.calc());

    // println!("{}", tree);

    // dbg!(tree.calc());

    // tree.random_modify_and_calc();

    // println!("{}", tree);

    // let mut test_tree = Node::gen_op();
    // println!("{}", test_tree);
    // dbg!(test_tree.calc());
    // println!("{}", test_tree);
    // test_tree.random_modify_and_calc();
    // println!("{}", test_tree);

    // tree_test.list.iter().for_each(|x| {
    //     println!("{}\n", x);
    // })
}
