#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_23;
use day_23::{Command, Computer};

#[test]
fn reading_data_exp1() {
    let test = Computer::new("./data/example_01.txt");
    assert_eq!(test.register, vec![0, 0, 0, 0]);
    assert_eq!(
        test.instructs,
        vec![
            Command::CopyVal(2, 0),
            Command::Toggle(0),
            Command::Toggle(0),
            Command::Toggle(0),
            Command::CopyVal(1, 0),
            Command::Dec(0),
            Command::Dec(0),
        ]
    );
    assert_eq!(
        test.toggled,
        vec![false, false, false, false, false, false, false]
    );
    assert_eq!(test.curr_instruc, 0);
}

#[test]
fn reading_data_exp2() {
    let test = Computer::new("./data/example_02.txt");
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
        ]
    );
    assert_eq!(test.toggled, vec![false, false, false, false, false, false]);
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
        Command::JumpVal(1, 2),
        Command::JumpVal(0, -2),
        Command::JumpVal(-1, 7),
        Command::JumpVal(7, -9),
    ];

    test.execute_instruct(0);
    assert_eq!(test.curr_instruc, 2);

    test.execute_instruct(1);
    assert_eq!(test.curr_instruc, 3);

    test.execute_instruct(2);
    assert_eq!(test.curr_instruc, 10);

    test.execute_instruct(3);
    assert_eq!(test.curr_instruc, 1);
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
fn test_command_inversion() {
    let mut test = Computer::new("./data/example_01.txt");
    test.instructs = vec![
        Command::Inc(0),
        Command::Dec(1),
        Command::Toggle(2),
        Command::CopyVal(1, 2),
        Command::CopyReg(2, 8),
        Command::JumpVal(1, 2),
        Command::JumpReg(3, -9),
        Command::JumpReg(3, 0),
    ];
    test.toggled = vec![true, true, true, true, true, true, true, true];

    /* One Argument Instructions. */
    test.execute_instruct(0);
    assert_eq!(test.register, vec![-1, 0, 0, 0]);
    assert_eq!(test.curr_instruc, 1);

    test.execute_instruct(1);
    assert_eq!(test.register, vec![-1, 1, 0, 0]);
    assert_eq!(test.curr_instruc, 2);

    test.execute_instruct(2);
    assert_eq!(test.register, vec![-1, 1, 1, 0]);
    assert_eq!(test.curr_instruc, 3);

    /* Two Argument Instructions */
    test.execute_instruct(3);
    assert_eq!(test.register, vec![-1, 1, 1, 0]);
    assert_eq!(test.curr_instruc, 5);

    test.execute_instruct(4);
    assert_eq!(test.register, vec![-1, 1, 1, 0]);
    assert_eq!(test.curr_instruc, 13);

    test.execute_instruct(5);
    assert_eq!(test.register, vec![-1, 1, 1, 0]);
    assert_eq!(test.curr_instruc, 14);

    test.execute_instruct(6);
    assert_eq!(test.register, vec![-1, 1, 1, 0]);
    assert_eq!(test.curr_instruc, 15);

    test.execute_instruct(7);
    assert_eq!(test.register, vec![0, 1, 1, 0]);
    assert_eq!(test.curr_instruc, 16);
}

#[test]
fn toggle() {
    let mut test = Computer::new("./data/example_01.txt");
    test.instructs = vec![
        Command::Toggle(0),
        Command::Toggle(3),
        Command::Toggle(6),
        Command::Toggle(5),
    ];
    test.execute_instruct(0);
    test.execute_instruct(1);
    test.execute_instruct(2);
    test.execute_instruct(3);
    assert_eq!(
        test.toggled,
        vec![true, false, false, true, false, true, true]
    );
}

#[test]
fn crack_safe_exp1() {
    assert_eq!(Computer::new("./data/example_01.txt").crack_safe(0), 3)
}

#[test]
fn crack_safe_exp2() {
    assert_eq!(Computer::new("./data/example_02.txt").crack_safe(0), 3)
}
