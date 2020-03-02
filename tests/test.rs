use incremntal_tree::operations::*;

#[test]
fn test_no_op() {
    let mut tree = Node {
        operation: Operation::Add,
        value: None,
        children: vec![
            Node::new_val(3),
            Node {
                operation: Operation::Sub,
                value: None,
                children: vec![
                    Node::new_val(1),
                    Node::new_val(0)
                ]
            }
        ]
    };

    let vec_changes = vec![Choice::Op, Choice::Right];
    let modi = "+";
    tree.calc();
    println!("{}", tree);
    let did_change = tree.define_modify_and_calc(vec_changes, modi);
    println!("{}", tree);
    dbg!(did_change);
    assert!(!did_change);
    
}
