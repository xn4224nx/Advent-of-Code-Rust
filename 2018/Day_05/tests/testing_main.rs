#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod main;
use crate::main::Polymer;
use std::collections::HashMap;

#[test]
fn read_polymer_exp0() {
    assert_eq!(Polymer::new("./data/example_0.txt").units, vec!['a', 'A']);
}

#[test]
fn read_polymer_exp1() {
    assert_eq!(
        Polymer::new("./data/example_1.txt").units,
        vec!['a', 'b', 'B', 'A']
    );
}

#[test]
fn read_polymer_exp2() {
    assert_eq!(
        Polymer::new("./data/example_2.txt").units,
        vec!['a', 'b', 'A', 'B']
    );
}

#[test]
fn read_polymer_exp3() {
    assert_eq!(
        Polymer::new("./data/example_3.txt").units,
        vec!['a', 'a', 'b', 'A', 'A', 'B']
    );
}

#[test]
fn read_polymer_exp4() {
    assert_eq!(
        Polymer::new("./data/example_4.txt").units,
        vec![
            'd', 'a', 'b', 'A', 'c', 'C', 'a', 'C', 'B', 'A', 'c', 'C', 'c', 'a', 'D', 'A'
        ]
    );
}

#[test]
fn compress_polymer_len_exp0() {
    assert_eq!(Polymer::new("./data/example_0.txt").compressed_len(), 0);
}

#[test]
fn compress_polymer_len_exp1() {
    assert_eq!(Polymer::new("./data/example_1.txt").compressed_len(), 0);
}

#[test]
fn compress_polymer_len_exp2() {
    assert_eq!(Polymer::new("./data/example_2.txt").compressed_len(), 4);
}

#[test]
fn compress_polymer_len_exp3() {
    assert_eq!(Polymer::new("./data/example_3.txt").compressed_len(), 6);
}

#[test]
fn compress_polymer_len_exp4() {
    assert_eq!(Polymer::new("./data/example_4.txt").compressed_len(), 10);
}
