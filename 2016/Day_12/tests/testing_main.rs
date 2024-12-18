#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_12;
use day_12::{Command, Computer};

#[test]
fn initialise_class() {
    let test = Computer::new("");

    assert_eq!(test.register, vec![0, 0, 0, 0]);
    assert_eq!(test.data_file, String::from(""));
    assert_eq!(test.instructs, Vec::new());
    assert_eq!(test.idx, 0);
}

#[test]
fn parse_exp_instructions() {
    let mut test = Computer::new("./data/example_01.txt");
    test.parse_instructs();

    assert_eq!(
        test.instructs,
        vec![
            Command::CopyVal(41, 0),
            Command::Incr(0),
            Command::Incr(0),
            Command::Decr(0),
            Command::JumpReg(0, 2),
            Command::Decr(0)
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
            Command::CopyVal(1, 0),
            Command::CopyVal(1, 1),
            Command::CopyVal(26, 3),
            Command::JumpReg(2, 2),
            Command::JumpVal(1, 5),
            Command::CopyVal(7, 2),
            Command::Incr(3),
            Command::Decr(2),
            Command::JumpReg(2, -2),
            Command::CopyReg(0, 2),
            Command::Incr(0),
            Command::Decr(1),
            Command::JumpReg(1, -2),
            Command::CopyReg(2, 1),
            Command::Decr(3),
            Command::JumpReg(3, -6),
            Command::CopyVal(13, 2),
            Command::CopyVal(14, 3),
            Command::Incr(0),
            Command::Decr(3),
            Command::JumpReg(3, -2),
            Command::Decr(2),
            Command::JumpReg(2, -5),
        ]
    )
}

#[test]
fn command_copyval() {
    let mut test = Computer::new("");
    test.instructs = vec![
        Command::CopyVal(1, 0),
        Command::CopyVal(1, 1),
        Command::CopyVal(1, 2),
        Command::CopyVal(1, 3),
    ];
    assert_eq!(test.register, vec![0, 0, 0, 0]);

    test.exe_curr_instr();
    test.exe_curr_instr();
    test.exe_curr_instr();
    test.exe_curr_instr();

    assert_eq!(test.register, vec![1, 1, 1, 1]);
    assert_eq!(test.idx, 4);
}

#[test]
fn command_copyreg() {
    let mut test = Computer::new("");
    test.register = vec![10, 0, 0, 0];
    test.instructs = vec![
        Command::CopyReg(0, 1),
        Command::CopyReg(1, 2),
        Command::CopyReg(2, 3),
    ];

    test.exe_curr_instr();
    test.exe_curr_instr();
    test.exe_curr_instr();

    assert_eq!(test.register, vec![10, 10, 10, 10]);
    assert_eq!(test.idx, 3);
}

#[test]
fn command_incr() {
    let mut test = Computer::new("");
    test.instructs = vec![
        Command::Incr(0),
        Command::Incr(1),
        Command::Incr(2),
        Command::Incr(3),
    ];
    assert_eq!(test.register, vec![0, 0, 0, 0]);

    test.exe_curr_instr();
    test.exe_curr_instr();
    test.exe_curr_instr();
    test.exe_curr_instr();

    assert_eq!(test.register, vec![1, 1, 1, 1]);
    assert_eq!(test.idx, 4);
}

#[test]
fn command_decr() {
    let mut test = Computer::new("");
    test.instructs = vec![
        Command::Decr(0),
        Command::Decr(1),
        Command::Decr(2),
        Command::Decr(3),
    ];
    assert_eq!(test.register, vec![0, 0, 0, 0]);

    test.exe_curr_instr();
    test.exe_curr_instr();
    test.exe_curr_instr();
    test.exe_curr_instr();

    assert_eq!(test.register, vec![-1, -1, -1, -1]);
    assert_eq!(test.idx, 4);
}

#[test]
fn command_jumpval() {
    let mut test = Computer::new("");
    test.instructs = vec![Command::JumpVal(0, 1)];
    test.exe_curr_instr();
    assert_eq!(test.idx, 1);

    test.idx = 0;
    test.instructs = vec![Command::JumpVal(1, 1)];
    test.exe_curr_instr();
    assert_eq!(test.idx, 1);

    test.idx = 0;
    test.instructs = vec![Command::JumpVal(4, 6)];
    test.exe_curr_instr();
    assert_eq!(test.idx, 6);

    test.idx = 0;
    test.instructs = vec![Command::JumpVal(-1, 10)];
    test.exe_curr_instr();
    assert_eq!(test.idx, 10);
}

#[test]
fn command_jumpreg() {
    let mut test = Computer::new("");
    test.instructs = vec![Command::JumpReg(0, 1)];
    assert_eq!(test.idx, 0);
    test.register = vec![10, 0, 0, 0];
    test.exe_curr_instr();
    assert_eq!(test.idx, 1);

    test.idx = 0;
    test.instructs = vec![Command::JumpReg(1, 1)];
    test.exe_curr_instr();
    assert_eq!(test.idx, 1);

    test.idx = 0;
    test.instructs = vec![Command::JumpReg(0, 9)];
    test.exe_curr_instr();
    assert_eq!(test.idx, 9);

    test.idx = 0;
    test.instructs = vec![Command::JumpReg(0, 5)];
    test.exe_curr_instr();
    assert_eq!(test.idx, 5);
}

#[test]
fn execute_exp1() {
    let mut test = Computer::new("./data/example_01.txt");
    test.instructs = vec![
        Command::CopyVal(41, 0),
        Command::Incr(0),
        Command::Incr(0),
        Command::Decr(0),
        Command::JumpReg(0, 2),
        Command::Decr(0),
    ];
    test.execute_all();
    assert_eq!(test.register[0], 42);
}
