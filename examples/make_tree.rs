use incremntal_tree::operations::{Calculable, Node, Operation};

fn main() {
    let mut tree = Node {
        operation: Operation::Add,
        value: None,
        children: vec![
            Node {
                operation: Operation::Add,
                value: None,
                children: vec![
                    Node::new_val(6),
                    Node {
                        operation: Operation::Div,
                        value: None,
                        children: vec![Node::new_val(25), Node::new_val(13)],
                    },
                ],
            },
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

    tree.random_modify_and_calc();

    println!("{}", tree);
}
