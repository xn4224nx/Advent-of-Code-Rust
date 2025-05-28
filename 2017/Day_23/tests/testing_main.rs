#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_23;
use day_23::{CPU, Command};

#[test]
fn new_computer_exp01() {
    let test = CPU::new("./data/example_01.txt");
    assert_eq!(test.register, vec![0; 8]);
    assert_eq!(
        test.instructions,
        vec![
            Command::SetVal(0, 1),
            Command::SubVal(0, 2),
            Command::MulReg(0, 0),
            Command::SetVal(0, 0),
            Command::JnzVal(0, -1),
            Command::SetVal(0, 1),
        ]
    );
    assert_eq!(test.instruc_idx, 0);
}

#[test]
fn new_enum_exp01() {
    assert_eq!(Command::new("set a 1"), Command::SetVal(0, 1));
}

#[test]
fn new_enum_exp02() {
    assert_eq!(Command::new("sub a 2"), Command::SubVal(0, 2));
}

#[test]
fn new_enum_exp03() {
    assert_eq!(Command::new("mul a a"), Command::MulReg(0, 0));
}

#[test]
fn new_enum_exp04() {
    assert_eq!(Command::new("set a 0"), Command::SetVal(0, 0));
}

#[test]
fn new_enum_exp05() {
    assert_eq!(Command::new("jnz a -1"), Command::JnzVal(0, -1));
}

#[test]
fn new_enum_exp06() {
    assert_eq!(Command::new("set a 1"), Command::SetVal(0, 1));
}

#[test]
fn new_enum_exp07() {
    assert_eq!(Command::new("jnz a -2"), Command::JnzVal(0, -2));
}

#[test]
fn execute_instruc_exp01() {
    let mut test = CPU::new("./data/example_01.txt");
    test.instructions = vec![Command::SetVal(1, 4)];
    test.execute_instruc();
    assert_eq!(test.register[1], 4);
    assert_eq!(test.instruc_idx, 1);
}

#[test]
fn execute_instruc_exp02() {
    let mut test = CPU::new("./data/example_01.txt");
    test.instructions = vec![Command::SubVal(1, 4)];
    test.execute_instruc();
    assert_eq!(test.register[1], -4);
    assert_eq!(test.instruc_idx, 1);
}

#[test]
fn execute_instruc_exp03() {
    let mut test = CPU::new("./data/example_01.txt");
    test.instructions = vec![Command::MulVal(1, 4)];
    test.execute_instruc();
    assert_eq!(test.register[1], 0);
    assert_eq!(test.instruc_idx, 1);
}

#[test]
fn execute_instruc_exp04() {
    let mut test = CPU::new("./data/example_01.txt");
    test.register[1] = 4;
    test.instructions = vec![Command::JnzVal(1, 4)];
    test.execute_instruc();
    assert_eq!(test.register[1], 4);
    assert_eq!(test.instruc_idx, 4);
}

#[test]
fn execute_instruc_exp05() {
    let mut test = CPU::new("./data/example_01.txt");
    test.instructions = vec![Command::JnzVal(1, 4)];
    test.execute_instruc();
    assert_eq!(test.register[1], 0);
    assert_eq!(test.instruc_idx, 1);
}

#[test]
fn execute_instruc_exp06() {
    let mut test = CPU::new("./data/example_01.txt");
    test.register = vec![1, 2, -3, 0];
    test.instructions = vec![Command::SetReg(1, 2)];
    test.execute_instruc();
    assert_eq!(test.register, vec![1, -3, -3, 0]);
    assert_eq!(test.instruc_idx, 1);
}

#[test]
fn execute_instruc_exp07() {
    let mut test = CPU::new("./data/example_01.txt");
    test.register = vec![1, 2, -3, 0];
    test.instructions = vec![Command::SubReg(1, 2)];
    test.execute_instruc();
    assert_eq!(test.register, vec![1, 5, -3, 0]);
    assert_eq!(test.instruc_idx, 1);
}

#[test]
fn execute_instruc_exp08() {
    let mut test = CPU::new("./data/example_01.txt");
    test.register = vec![1, 2, -3, 0];
    test.instructions = vec![Command::MulReg(1, 2)];
    test.execute_instruc();
    assert_eq!(test.register, vec![1, -6, -3, 0]);
    assert_eq!(test.instruc_idx, 1);
}

#[test]
fn execute_instruc_exp09() {
    let mut test = CPU::new("./data/example_01.txt");
    test.register = vec![1, 2, -3, 0];
    test.instructions = vec![Command::JnzReg(3, 2)];
    test.execute_instruc();
    assert_eq!(test.register, vec![1, 2, -3, 0]);
    assert_eq!(test.instruc_idx, 1);
}

#[test]
fn execute_instruc_exp10() {
    let mut test = CPU::new("./data/example_01.txt");
    test.register = vec![1, 2, -3, 0];
    test.instructions = vec![Command::JnzReg(1, 1)];
    test.execute_instruc();
    assert_eq!(test.register, vec![1, 2, -3, 0]);
    assert_eq!(test.instruc_idx, 2);
}

#[test]
fn run_all_exp01() {
    assert_eq!(CPU::new("./data/example_01.txt").run_all(), 1);
}
