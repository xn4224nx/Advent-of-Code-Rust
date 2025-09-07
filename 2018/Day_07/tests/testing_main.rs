#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod main;
use crate::main::WordMaker;
use std::collections::{HashMap, HashSet};

#[test]
fn new_instance_exp0() {
    assert_eq!(
        WordMaker::new("./data/example_0.txt").dependencies,
        HashMap::from([
            ('A', HashSet::from(['C'])),
            ('B', HashSet::from(['A'])),
            ('C', HashSet::new()),
            ('D', HashSet::from(['A'])),
            ('E', HashSet::from(['B', 'D', 'F'])),
            ('F', HashSet::from(['C'])),
        ])
    )
}

#[test]
fn single_worker_word_exp0() {
    assert_eq!(
        WordMaker::new("./data/example_0.txt").multi_worker(1, 0).0,
        String::from("CABDFE")
    )
}

#[test]
fn multi_worker_word_exp0() {
    assert_eq!(
        WordMaker::new("./data/example_0.txt").multi_worker(2, 0),
        (String::from("CABFDE"), 15)
    )
}
