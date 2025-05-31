#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_25;
use day_25::{BluePrint, TuringMachine};
use std::collections::{HashMap, HashSet};

#[test]
fn new_turing_machine_exp01() {
    let test = TuringMachine::new("./data/example_01.txt");

    assert_eq!(test.tape, HashSet::new());
    assert_eq!(test.state, 'A');
    assert_eq!(test.cursor, 0);
    assert_eq!(test.diag_steps, 6);
    assert_eq!(
        test.blueprints,
        HashMap::from([
            (
                'A',
                vec![
                    BluePrint {
                        write_val: 1,
                        move_dir: 1,
                        next_state: 'B'
                    },
                    BluePrint {
                        write_val: 0,
                        move_dir: -1,
                        next_state: 'B'
                    }
                ]
            ),
            (
                'B',
                vec![
                    BluePrint {
                        write_val: 1,
                        move_dir: -1,
                        next_state: 'A'
                    },
                    BluePrint {
                        write_val: 1,
                        move_dir: 1,
                        next_state: 'A'
                    }
                ]
            ),
        ])
    );
}

#[test]
fn advance_exp01() {
    let mut test = TuringMachine::new("./data/example_01.txt");
    test.advance();
    assert_eq!(test.tape, HashSet::from([0]));
    assert_eq!(test.state, 'B');
    assert_eq!(test.cursor, 1);
}

#[test]
fn advance_exp02() {
    let mut test = TuringMachine::new("./data/example_01.txt");

    for _ in 0..3 {
        test.advance()
    }

    assert_eq!(test.tape, HashSet::from([0, 1]));
    assert_eq!(test.state, 'A');
    assert_eq!(test.cursor, 0);
}

#[test]
fn advance_exp03() {
    let mut test = TuringMachine::new("./data/example_01.txt");

    for _ in 0..3 {
        test.advance()
    }

    assert_eq!(test.tape, HashSet::from([1]));
    assert_eq!(test.state, 'B');
    assert_eq!(test.cursor, -1);
}

#[test]
fn advance_exp04() {
    let mut test = TuringMachine::new("./data/example_01.txt");
    for _ in 0..4 {
        test.advance()
    }
    assert_eq!(test.tape, HashSet::from([-1, 1]));
    assert_eq!(test.state, 'A');
    assert_eq!(test.cursor, -2);
}

#[test]
fn advance_exp05() {
    let mut test = TuringMachine::new("./data/example_01.txt");
    for _ in 0..5 {
        test.advance()
    }
    assert_eq!(test.tape, HashSet::from([-1, 1, -2]));
    assert_eq!(test.state, 'B');
    assert_eq!(test.cursor, -1);
}

#[test]
fn advance_exp06() {
    let mut test = TuringMachine::new("./data/example_01.txt");
    for _ in 0..6 {
        test.advance()
    }
    assert_eq!(test.tape, HashSet::from([-1, 1, -2]));
    assert_eq!(test.state, 'A');
    assert_eq!(test.cursor, 0);
}

#[test]
fn diagnostics_exp01() {
    assert_eq!(TuringMachine::new("./data/example_01.txt").diagnostics(), 3)
}
