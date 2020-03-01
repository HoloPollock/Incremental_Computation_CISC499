use incremntal_tree::operations::{Calculable, Node, Operation};

fn main() {
    let mut tree = Node {
        operation: Operation::Add,
        value: None,
        children: vec![
            Node::new_val(12),
            Node {
                operation: Operation::Mul,
                value: None,
                children: vec![Node::new_val(3), Node::new_val(4)],
            },
        ],
    };

    println!("{}", tree);

    dbg!(tree.calc());

    println!("{}", tree);

    dbg!(tree.calc());
}
