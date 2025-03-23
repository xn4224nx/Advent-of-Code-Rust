#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_8;
use day_8::{Command, Comp, Computer};
use std::collections::HashMap;

#[test]
fn start_computer() {
    let test = Computer::new("./data/example_01.txt");

    assert_eq!(
        test.register,
        HashMap::from([
            (String::from("a"), 0),
            (String::from("b"), 0),
            (String::from("c"), 0),
        ])
    );

    assert_eq!(
        test.instrucs,
        vec![
            Command::Inc(String::from("b"), 5, String::from("a"), Comp::MoreThan, 1),
            Command::Inc(String::from("a"), 1, String::from("b"), Comp::LessThan, 5),
            Command::Dec(
                String::from("c"),
                -10,
                String::from("a"),
                Comp::MoreThanOrEqual,
                1
            ),
            Command::Inc(String::from("c"), -20, String::from("c"), Comp::Equal, 10),
        ]
    );
}

#[test]
fn find_largest_register_values() {
    assert_eq!(Computer::new("./data/example_01.txt").largest_value(), 1);
}
