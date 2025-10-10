#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod main;
use main::IntCodeProgram;

#[test]
fn new_program_exp0() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_0.txt").memory,
        vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
    );
}

#[test]
fn new_program_exp1() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_1.txt").memory,
        vec![1, 0, 0, 0, 99]
    );
}

#[test]
fn new_program_exp2() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_2.txt").memory,
        vec![2, 3, 0, 3, 99]
    );
}

#[test]
fn new_program_exp3() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_3.txt").memory,
        vec![2, 4, 4, 5, 99, 0]
    );
}

#[test]
fn new_program_exp4() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_4.txt").memory,
        vec![1, 1, 1, 4, 99, 5, 6, 0, 99]
    );
}

#[test]
fn addition_cmd_exp0() {
    let mut test = IntCodeProgram {
        memory: vec![1, 0, 0, 0, 99],
        pntr: 0,
    };
    test.execute_cmd();
    assert_eq!(test.memory, vec![2, 0, 0, 0, 99]);
}

#[test]
fn addition_cmd_exp1() {
    let mut test = IntCodeProgram {
        memory: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
        pntr: 0,
    };
    test.execute_cmd();
    assert_eq!(test.memory, vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    assert_eq!(test.pntr, 4);
}

#[test]
fn multiplication_cmd_exp0() {
    let mut test = IntCodeProgram {
        memory: vec![2, 3, 0, 3, 99],
        pntr: 0,
    };
    test.execute_cmd();
    assert_eq!(test.memory, vec![2, 3, 0, 6, 99]);
}

#[test]
fn multiplication_cmd_exp1() {
    let mut test = IntCodeProgram {
        memory: vec![2, 4, 4, 5, 99, 0],
        pntr: 0,
    };
    test.execute_cmd();
    assert_eq!(test.memory, vec![2, 4, 4, 5, 99, 9801]);
}

#[test]
fn addition_cmd_exp2() {
    let mut test = IntCodeProgram {
        memory: vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        pntr: 4,
    };
    test.execute_cmd();
    assert_eq!(
        test.memory,
        vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
    assert_eq!(test.pntr, 8);
}

#[test]
fn multi_cmd_exp0() {
    let mut test = IntCodeProgram {
        memory: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
        pntr: 0,
    };
    test.execute_cmd();
    test.execute_cmd();
    assert_eq!(
        test.memory,
        vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
    assert_eq!(test.pntr, 8);
}

#[test]
fn run_exp0() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_0.txt").run(false),
        3500
    );
}

#[test]
fn run_exp1() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_1.txt").run(false),
        2
    );
}

#[test]
fn run_exp2() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_2.txt").run(false),
        2
    );
}

#[test]
fn run_exp3() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_3.txt").run(false),
        2
    );
}

#[test]
fn run_exp4() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_4.txt").run(false),
        30
    );
}
