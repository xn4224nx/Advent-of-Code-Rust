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
fn first_letters_exp0() {
    assert_eq!(
        WordMaker::new("./data/example_0.txt").first_letters(),
        vec!['C']
    )
}

#[test]
fn single_worker_word_exp0() {
    assert_eq!(
        WordMaker::new("./data/example_0.txt").single_worker(),
        String::from("CABDFE")
    )
}
