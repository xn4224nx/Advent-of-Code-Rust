#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_25;
use day_25::{convert_char_to_idx, Command, Computer};

#[test]
fn reading_data_exp1() {
    let test = Computer::new("./data/example_01.txt");
    assert_eq!(test.register, vec![0, 0, 0, 0]);
    assert_eq!(
        test.instructs,
        vec![
            Command::CopyVal(41, 0),
            Command::Inc(0),
            Command::Inc(0),
            Command::Dec(0),
            Command::JumpReg(0, 2),
            Command::Dec(0),
            Command::Out(0),
        ]
    );
    assert_eq!(test.display, Vec::new());
    assert_eq!(test.curr_instruc, 0);
}

#[test]
fn copy_value() {
    let mut test = Computer::new("./data/example_01.txt");
    test.instructs = vec![
        Command::CopyVal(-7, 0),
        Command::CopyVal(244, 1),
        Command::CopyVal(6, 2),
        Command::CopyVal(7, 3),
    ];

    test.execute_instruct(0);
    test.execute_instruct(1);
    test.execute_instruct(2);
    test.execute_instruct(3);

    assert_eq!(test.register, vec![-7, 244, 6, 7]);
}

#[test]
fn copy_register() {
    let mut test = Computer::new("./data/example_01.txt");
    test.register = vec![9, 0, 0, -9];
    test.instructs = vec![
        Command::CopyReg(0, 1),
        Command::CopyReg(1, 2),
        Command::CopyReg(3, 0),
        Command::CopyReg(2, 3),
    ];
    test.execute_instruct(0);
    test.execute_instruct(1);
    test.execute_instruct(2);
    test.execute_instruct(3);
    assert_eq!(test.register, vec![-9, 9, 9, 9]);
}

#[test]
fn increment() {
    let mut test = Computer::new("./data/example_01.txt");
    test.instructs = vec![Command::Inc(1), Command::Inc(0), Command::Inc(0)];
    test.execute_instruct(0);
    test.execute_instruct(1);
    test.execute_instruct(2);

    assert_eq!(test.register, vec![2, 1, 0, 0]);
}

#[test]
fn deincrement() {
    let mut test = Computer::new("./data/example_01.txt");
    test.instructs = vec![Command::Dec(2), Command::Dec(3), Command::Dec(3)];
    test.execute_instruct(0);
    test.execute_instruct(1);
    test.execute_instruct(2);

    assert_eq!(test.register, vec![0, 0, -1, -2]);
}

#[test]
fn jump_value() {
    let mut test = Computer::new("./data/example_01.txt");
    test.instructs = vec![
        Command::JumpVal(1, 7),
        Command::JumpVal(0, 3),
        Command::JumpVal(-1, 2),
        Command::JumpVal(7, 3),
    ];

    test.execute_instruct(0);
    assert_eq!(test.curr_instruc, 7);

    test.execute_instruct(1);
    assert_eq!(test.curr_instruc, 8);

    test.execute_instruct(2);
    assert_eq!(test.curr_instruc, 10);

    test.execute_instruct(3);
    assert_eq!(test.curr_instruc, 13);
}

#[test]
fn jump_register() {
    let mut test = Computer::new("./data/example_01.txt");
    test.instructs = vec![
        Command::JumpReg(1, 2),
        Command::JumpReg(0, -2),
        Command::JumpReg(2, 7),
        Command::JumpReg(3, -9),
    ];
    test.register = vec![0, 1, -2, 0];

    test.execute_instruct(0);
    assert_eq!(test.curr_instruc, 2);

    test.execute_instruct(1);
    assert_eq!(test.curr_instruc, 3);

    test.execute_instruct(2);
    assert_eq!(test.curr_instruc, 10);

    test.execute_instruct(3);
    assert_eq!(test.curr_instruc, 11);
}

#[test]
fn out() {
    let mut test = Computer::new("./data/example_01.txt");
    test.instructs = vec![
        Command::Out(1),
        Command::Out(0),
        Command::Out(2),
        Command::Out(3),
    ];
    test.register = vec![0, 1, -2, 0];

    test.execute_instruct(0);
    test.execute_instruct(1);
    test.execute_instruct(2);
    test.execute_instruct(3);

    assert_eq!(test.display, vec![1, 0, -2, 0]);
}

#[test]
fn modify_instruc_idx() {
    let mut test = Computer::new("./data/example_01.txt");

    test.modify_instruc_idx(4);
    assert_eq!(test.curr_instruc, 4);

    test.modify_instruc_idx(4);
    assert_eq!(test.curr_instruc, 8);

    test.modify_instruc_idx(-8);
    assert_eq!(test.curr_instruc, 0);

    test.modify_instruc_idx(-1);
    assert_eq!(test.curr_instruc, usize::MAX);
}

#[test]
fn convert_char_to_number() {
    assert_eq!(convert_char_to_idx("a"), 0);
    assert_eq!(convert_char_to_idx("b"), 1);
    assert_eq!(convert_char_to_idx("c"), 2);
    assert_eq!(convert_char_to_idx("d"), 3);
}
