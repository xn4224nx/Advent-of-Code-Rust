#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_8;
use day_8::{Command, Comp, Computer};
use std::collections::HashMap;

#[test]
fn start_computer_instructions() {
    let test = Computer::new("./data/example_01.txt");
    assert_eq!(
        test.instrucs,
        vec![
            Command {
                change_reg: String::from("b"),
                change_val: 5,
                test_reg: String::from("a"),
                comp_opp: Comp::MoreThan,
                test_val: 1,
            },
            Command {
                change_reg: String::from("a"),
                change_val: 1,
                test_reg: String::from("b"),
                comp_opp: Comp::LessThan,
                test_val: 5,
            },
            Command {
                change_reg: String::from("c"),
                change_val: 10,
                test_reg: String::from("a"),
                comp_opp: Comp::MoreThanOrEqual,
                test_val: 1,
            },
            Command {
                change_reg: String::from("c"),
                change_val: -20,
                test_reg: String::from("c"),
                comp_opp: Comp::Equal,
                test_val: 10,
            },
        ]
    );
}

#[test]
fn start_computer_register() {
    let test = Computer::new("./data/example_01.txt");
    assert_eq!(
        test.register,
        HashMap::from([
            (String::from("a"), 0),
            (String::from("b"), 0),
            (String::from("c"), 0),
        ])
    );
}

#[test]
fn find_largest_register_values() {
    assert_eq!(Computer::new("./data/example_01.txt").largest_value(), 1);
}
