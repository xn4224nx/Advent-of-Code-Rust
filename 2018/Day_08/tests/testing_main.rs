#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod main;
use crate::main::FlatTree;

#[test]
fn new_instance_exp0() {
    assert_eq!(
        FlatTree::new("./data/example_0.txt").vals,
        vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]
    )
}

#[test]
fn metadata_sum_exp0() {
    assert_eq!(
        FlatTree::new("./data/example_0.txt").statistics(),
        (138, 66)
    );
}
