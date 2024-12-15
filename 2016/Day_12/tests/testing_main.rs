#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_12;
use day_12::{Command, Computer};

#[test]
fn initialise_class() {
    let mut test = Computer::new("");

    assert_eq!(test.register, vec![0, 0, 0, 0]);
    assert_eq!(test.data_file, String::from(""));
    assert_eq!(test.instructs, Vec::new());
    assert_eq!(test.instruct_idx, 0);
}

#[test]
fn parse_exp_instructions() {
    let mut test = Computer::new("./data/example_01.txt");
    test.parse_instructs();

    assert_eq!(
        test.instructs,
        vec![
            Command::copy_val(41, 0),
            Command::incr(0),
            Command::incr(0),
            Command::decr(0),
            Command::jump_reg(0, 2),
            Command::decr(0)
        ]
    )
}

#[test]
fn parse_input_instructions() {
    let mut test = Computer::new("./data/input.txt");
    test.parse_instructs();

    assert_eq!(
        test.instructs,
        vec![
            Command::copy_val(1, 0),
            Command::copy_val(1, 1),
            Command::copy_val(26, 3),
            Command::jump_reg(2, 2),
            Command::jump_val(1, 2),
            Command::copy_val(7, 2),
            Command::incr(3),
            Command::decr(2),
            Command::jump_reg(2, -2),
            Command::copy_reg(0, 2),
            Command::incr(0),
            Command::decr(1),
            Command::jump_reg(1, -2),
            Command::copy_reg(2, 1),
            Command::decr(3),
            Command::jump_reg(3, -6),
            Command::copy_val(13, 2),
            Command::copy_val(14, 3),
            Command::incr(0),
            Command::decr(3),
            Command::jump_reg(3, -2),
            Command::decr(2),
            Command::jump_reg(2, -5),
        ]
    )
}

#[test]
fn command_copy_val() {
    let mut test = Computer::new("");
    test.instructs = vec![
        Command::copy_val(1, 0),
        Command::copy_val(1, 1),
        Command::copy_val(1, 2),
        Command::copy_val(1, 3),
    ];
    assert_eq!(test.register, vec![0, 0, 0, 0]);

    test.exe_curr_instr();
    test.exe_curr_instr();
    test.exe_curr_instr();
    test.exe_curr_instr();

    assert_eq!(test.register, vec![1, 1, 1, 1]);
    assert_eq!(test.instruct_idx, 4);
}

#[test]
fn command_copy_reg() {
    let mut test = Computer::new("");
    test.register = vec![10, 0, 0, 0];
    test.instructs = vec![
        Command::copy_reg(0, 1),
        Command::copy_reg(1, 2),
        Command::copy_reg(2, 3),
    ];

    test.exe_curr_instr();
    test.exe_curr_instr();
    test.exe_curr_instr();

    assert_eq!(test.register, vec![10, 10, 10, 10]);
    assert_eq!(test.instruct_idx, 3);
}

#[test]
fn command_incr() {
    let mut test = Computer::new("");
    test.instructs = vec![
        Command::incr(0),
        Command::incr(1),
        Command::incr(2),
        Command::incr(3),
    ];
    assert_eq!(test.register, vec![0, 0, 0, 0]);

    test.exe_curr_instr();
    test.exe_curr_instr();
    test.exe_curr_instr();
    test.exe_curr_instr();

    assert_eq!(test.register, vec![1, 1, 1, 1]);
    assert_eq!(test.instruct_idx, 4);
}

#[test]
fn command_decr() {
    let mut test = Computer::new("");
    test.instructs = vec![
        Command::decr(0),
        Command::decr(1),
        Command::decr(2),
        Command::decr(3),
    ];
    assert_eq!(test.register, vec![0, 0, 0, 0]);

    test.exe_curr_instr();
    test.exe_curr_instr();
    test.exe_curr_instr();
    test.exe_curr_instr();

    assert_eq!(test.register, vec![-1, -1, -1, -1]);
    assert_eq!(test.instruct_idx, 4);
}

#[test]
fn command_jump_val() {
    let mut test = Computer::new("");
    test.instructs = vec![Command::jump_val(0, 1)];
    test.exe_curr_instr();
    assert_eq!(test.instruct_idx, 0);

    test.instruct_idx = 0;
    test.instructs = vec![Command::jump_val(1, 1)];
    test.exe_curr_instr();
    assert_eq!(test.instruct_idx, 1);

    test.instruct_idx = 0;
    test.instructs = vec![Command::jump_val(4, 6)];
    test.exe_curr_instr();
    assert_eq!(test.instruct_idx, 6);

    test.instructs = vec![Command::jump_val(-1, -1)];
    test.exe_curr_instr();
    assert_eq!(test.instruct_idx, 5);
}

#[test]
fn command_jump_reg() {
    let mut test = Computer::new("");
    test.instructs = vec![Command::jump_reg(0, 1)];
    assert_eq!(test.instruct_idx, 0);
    test.register = vec![10, 0, 0, 0];
    test.exe_curr_instr();
    assert_eq!(test.instruct_idx, 1);

    test.instruct_idx = 0;
    test.instructs = vec![Command::jump_reg(1, 1)];
    test.exe_curr_instr();
    assert_eq!(test.instruct_idx, 0);

    test.instruct_idx = 0;
    test.instructs = vec![Command::jump_reg(0, 9)];
    test.exe_curr_instr();
    assert_eq!(test.instruct_idx, 9);

    test.instructs = vec![Command::jump_reg(0, -4)];
    test.exe_curr_instr();
    assert_eq!(test.instruct_idx, 5);
}

#[test]
fn execute_exp1() {
    let mut test = Computer::new("./data/example_01.txt");
    test.instructs = vec![
        Command::copy_val(41, 0),
        Command::incr(0),
        Command::incr(0),
        Command::decr(0),
        Command::jump_reg(0, 2),
        Command::decr(0),
    ];
    test.execute_all();
    assert_eq!(test.register[0], 42);
}
