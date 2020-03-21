use incremental_tree::{
    self,
    node::{Calculable, Node},
    operations::Operation,
    list::NodeList,
    choice::Choice
};
use std::time::Instant;
fn main()
{
    dbg!("genning");
    let mut tree = Node::gen_node_of_depth(50);
    let mut clean_tree = tree.clone();
    dbg!("done genning");

    let now = Instant::now();
    tree.calc();
    let after_calc = Instant::now();

    println!("{}", tree);
    println!("{}", clean_tree);

    let clean_now = Instant::now();
    clean_tree.define_modify(vec![Choice::Right, Choice::Left], "10");
    clean_tree.calc();
    let clean_after_calc = Instant::now();

    let inc_now = Instant::now();
    tree.define_modify_and_calc(vec![Choice::Right, Choice::Left], "10");
    let inc_after_calc = Instant::now();
    
    println!("From Scratch Calc: {:#?}", after_calc.duration_since(now));
    println!("Modify Calc: {:#?}", clean_after_calc.duration_since(clean_now));
    println!("Inc Calc: {:#?}", inc_after_calc.duration_since(inc_now));
}