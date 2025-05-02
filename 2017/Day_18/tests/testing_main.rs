#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_18;
use day_18::{Cmd, Duo};

#[test]
fn new_class_exp01() {
    let test = Duo::new("./data/example_01.txt");
    assert_eq!(test.buffer0, Vec::new());
    assert_eq!(test.buffer1, Vec::new());
    assert_eq!(test.register, vec![0]);
    assert_eq!(test.cmd_idx, 0);
    assert_eq!(
        test.instructions,
        vec![
            Cmd::SetVal(0, 1),
            Cmd::AddVal(0, 2),
            Cmd::MulReg(0, 0),
            Cmd::ModVal(0, 5),
            Cmd::SendReg(0),
            Cmd::SetVal(0, 0),
            Cmd::RcvReg(0),
            Cmd::JgzRegVal(0, -1),
            Cmd::SetVal(0, 1),
            Cmd::JgzRegVal(0, -2),
        ]
    );
}

#[test]
fn new_class_exp02() {
    let test = Duo::new("./data/example_02.txt");
    assert_eq!(test.buffer0, Vec::new());
    assert_eq!(test.register, vec![0; 16]);
    assert_eq!(test.cmd_idx, 0);
    assert_eq!(
        test.instructions,
        vec![
            Cmd::SendVal(1),
            Cmd::SendVal(2),
            Cmd::SendReg(15),
            Cmd::RcvReg(0),
            Cmd::RcvReg(1),
            Cmd::RcvReg(2),
            Cmd::RcvReg(3),
        ]
    )
}

#[test]
fn execute_cmd_exp01() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![0];
    test.instructions = vec![Cmd::SendVal(3)];
    test.execute_cmd(0);
    assert_eq!(test.buffer0, vec![3]);
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp02() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![3];
    test.instructions = vec![Cmd::SendReg(0)];
    test.execute_cmd(0);
    assert_eq!(test.buffer0, vec![3]);
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp03() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![0];
    test.instructions = vec![Cmd::SetVal(0, -7)];
    test.execute_cmd(0);
    assert_eq!(test.register, vec![-7]);
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp04() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![3, 5];
    test.instructions = vec![Cmd::SetReg(0, 1)];
    test.execute_cmd(0);
    assert_eq!(test.register, vec![5, 5]);
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp05() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![3];
    test.instructions = vec![Cmd::AddVal(0, 1)];
    test.execute_cmd(0);
    assert_eq!(test.register, vec![4]);
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp06() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![3, 5];
    test.instructions = vec![Cmd::AddReg(0, 1)];
    test.execute_cmd(0);
    assert_eq!(test.register, vec![8, 5]);
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp07() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![3, 5];
    test.instructions = vec![Cmd::MulVal(0, -7)];
    test.execute_cmd(0);
    assert_eq!(test.register, vec![-21, 5]);
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp08() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![3, 5];
    test.instructions = vec![Cmd::MulReg(0, 1)];
    test.execute_cmd(0);
    assert_eq!(test.register, vec![15, 5]);
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp09() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![14];
    test.instructions = vec![Cmd::ModVal(0, 7)];
    test.execute_cmd(0);
    assert_eq!(test.register, vec![0]);
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp10() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![5, 16];
    test.instructions = vec![Cmd::ModReg(1, 0)];
    test.execute_cmd(0);
    assert_eq!(test.register, vec![5, 1]);
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp11() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![3];
    test.buffer0 = vec![14];
    test.instructions = vec![Cmd::RcvReg(0)];
    test.execute_cmd(0);
    assert_eq!(test.buffer0, Vec::new());
    assert_eq!(test.buffer1, vec![14]);
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp12() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![0];
    test.buffer0 = vec![14];
    test.instructions = vec![Cmd::RcvReg(0)];
    test.execute_cmd(0);
    assert_eq!(test.buffer0, vec![14]);
    assert_eq!(test.buffer1, Vec::new());
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp13() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![6, 16];
    test.instructions = vec![Cmd::JgzRegReg(1, 0)];
    test.execute_cmd(0);
    assert_eq!(test.cmd_idx, 6);
}

#[test]
fn execute_cmd_exp14() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![5, 0];
    test.instructions = vec![Cmd::JgzRegReg(1, 0)];
    test.execute_cmd(0);
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp15() {
    let mut test = Duo::new("./data/example_01.txt");
    test.instructions = vec![Cmd::JgzValVal(-1, 7)];
    test.execute_cmd(0);
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp16() {
    let mut test = Duo::new("./data/example_01.txt");
    test.instructions = vec![Cmd::JgzValVal(1, 7)];
    test.execute_cmd(0);
    assert_eq!(test.cmd_idx, 7);
}

#[test]
fn execute_cmd_exp17() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![0];
    test.instructions = vec![Cmd::JgzRegVal(0, 7)];
    test.execute_cmd(0);
    assert_eq!(test.register, vec![0]);
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp18() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![1];
    test.instructions = vec![Cmd::JgzRegVal(0, 3)];
    test.execute_cmd(0);
    assert_eq!(test.register, vec![1]);
    assert_eq!(test.cmd_idx, 3);
}

#[test]
fn execute_cmd_exp19() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![10];
    test.instructions = vec![Cmd::JgzValReg(0, 0)];
    test.execute_cmd(0);
    assert_eq!(test.cmd_idx, 1);
}

#[test]
fn execute_cmd_exp20() {
    let mut test = Duo::new("./data/example_01.txt");
    test.register = vec![10];
    test.instructions = vec![Cmd::JgzValReg(3, 0)];
    test.execute_cmd(0);
    assert_eq!(test.cmd_idx, 10);
}

#[test]
fn first_recent_sound_exp01() {
    assert_eq!(Duo::new("./data/example_01.txt").first_recent_sound(), 4);
}
