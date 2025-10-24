#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod main;
use main::{IntCodeProgram, OPCode};
use rand::Rng;

#[test]
fn new_program_exp0() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_0.txt").memory,
        vec![3, 0, 4, 0, 99]
    );
}

#[test]
fn new_program_exp1() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_1.txt").memory,
        vec![1002, 4, 3, 4, 33]
    );
}

#[test]
fn new_program_exp2() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_2.txt").memory,
        vec![1101, 100, -1, 4, 0]
    );
}

#[test]
fn new_program_exp3() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_3.txt").memory,
        vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]
    );
}

#[test]
fn new_program_exp4() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_4.txt").memory,
        vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]
    );
}

#[test]
fn new_program_exp5() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_5.txt").memory,
        vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]
    );
}

#[test]
fn new_program_exp6() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_6.txt").memory,
        vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]
    );
}

#[test]
fn new_program_exp7() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_7.txt").memory,
        vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]
    );
}

#[test]
fn new_program_exp8() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_8.txt").memory,
        vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
    );
}

#[test]
fn new_program_exp9() {
    assert_eq!(
        IntCodeProgram::from_file("./data/example_9.txt").memory,
        vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99
        ]
    );
}

#[test]
fn execute_print_cmd() {
    let mut test = IntCodeProgram::from_file("./data/example_0.txt");
    test.input_val = rand::thread_rng().gen_range(-10000..10000);

    test.execute_cmd();
    assert_eq!(test.memory, vec![test.input_val, 0, 4, 0, 99]);

    test.execute_cmd();
    assert_eq!(test.outputs, vec![test.input_val]);
}

#[test]
fn execute_multiply_cmd() {
    let mut test = IntCodeProgram::from_file("./data/example_1.txt");
    test.execute_cmd();
    assert_eq!(test.memory, vec![1002, 4, 3, 4, 99]);
}

#[test]
fn execute_add_cmd() {
    let mut test = IntCodeProgram::from_file("./data/example_2.txt");
    test.execute_cmd();
    assert_eq!(test.memory, vec![1101, 100, -1, 4, 99]);
}

#[test]
fn extract_modes_exp0() {
    assert_eq!(IntCodeProgram::extract_modes(1002), (2, 0, 1, 0));
}

#[test]
fn extract_modes_exp1() {
    assert_eq!(IntCodeProgram::extract_modes(4), (4, 0, 0, 0));
}

#[test]
fn extract_modes_exp2() {
    assert_eq!(IntCodeProgram::extract_modes(3), (3, 0, 0, 0));
}

#[test]
fn extract_modes_exp3() {
    assert_eq!(IntCodeProgram::extract_modes(1207), (7, 2, 1, 0));
}

#[test]
fn extract_modes_exp4() {
    assert_eq!(IntCodeProgram::extract_modes(91207), (7, 2, 1, 9));
}

#[test]
fn extract_modes_exp5() {
    assert_eq!(IntCodeProgram::extract_modes(9127), (27, 1, 9, 0));
}

#[test]
fn final_diag_code_exp0() {
    let rnd_val = rand::thread_rng().gen_range(-10000..10000);
    assert_eq!(
        IntCodeProgram::from_file("./data/example_0.txt").final_diag_code(rnd_val),
        rnd_val
    );
}
