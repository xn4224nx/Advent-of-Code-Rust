#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_07;
use day_07::{final_signals, read_booklet};
use std::collections::HashMap;

#[test]
fn signals_exp_01() {
    let data = read_booklet("./data/example_01.txt");
    assert_eq!(
        final_signals(&data),
        HashMap::from([
            ("y".to_string(), 456),
            ("d".to_string(), 72),
            ("i".to_string(), 65079),
            ("x".to_string(), 123),
            ("h".to_string(), 65412),
            ("f".to_string(), 492),
            ("g".to_string(), 114),
            ("e".to_string(), 507)
        ])
    )
}
