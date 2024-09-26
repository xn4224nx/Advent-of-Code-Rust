#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_23;
use day_23::{Comm, Computer};

#[test]
fn read_example_data() {
    let mut test_sys = Computer::new(0, 0);
    test_sys.read_comms("./data/example_01.txt");
    assert_eq!(
        test_sys.comms,
        vec![
            Comm::Increm('a'),
            Comm::JumpIfOne(('a', 2)),
            Comm::Triple('a'),
            Comm::Increm('a'),
        ]
    );
}

#[test]
fn read_input_data() {
    let mut test_sys = Computer::new(0, 0);
    test_sys.read_comms("./data/input.txt");
    assert_eq!(
        test_sys.comms,
        vec![
            Comm::JumpIfOne(('a', 18)),
            Comm::Increm('a'),
            Comm::Triple('a'),
            Comm::Increm('a'),
            Comm::Triple('a'),
            Comm::Triple('a'),
            Comm::Triple('a'),
            Comm::Increm('a'),
            Comm::Triple('a'),
            Comm::Increm('a'),
            Comm::Triple('a'),
            Comm::Increm('a'),
            Comm::Increm('a'),
            Comm::Triple('a'),
            Comm::Triple('a'),
            Comm::Triple('a'),
            Comm::Increm('a'),
            Comm::Jump(22),
            Comm::Triple('a'),
            Comm::Increm('a'),
            Comm::Triple('a'),
            Comm::Increm('a'),
            Comm::Increm('a'),
            Comm::Triple('a'),
            Comm::Increm('a'),
            Comm::Triple('a'),
            Comm::Increm('a'),
            Comm::Increm('a'),
            Comm::Triple('a'),
            Comm::Triple('a'),
            Comm::Increm('a'),
            Comm::Increm('a'),
            Comm::Triple('a'),
            Comm::Increm('a'),
            Comm::Increm('a'),
            Comm::Triple('a'),
            Comm::Increm('a'),
            Comm::Increm('a'),
            Comm::Triple('a'),
            Comm::JumpIfOne(('a', 8)),
            Comm::Increm('b'),
            Comm::JumpIfEven(('a', 4)),
            Comm::Triple('a'),
            Comm::Increm('a'),
            Comm::Jump(2),
            Comm::Half('a'),
            Comm::Jump(-7)
        ]
    );
}

#[test]
fn halve_register_a() {
    let mut test_sys = Computer::new(3, 0);
    test_sys.comms.push(Comm::Half('a'));
    test_sys.execute_comms();
    assert_eq!(test_sys.reg_a, 1);
}

#[test]
fn halve_register_b() {
    let mut test_sys = Computer::new(0, 4);
    test_sys.comms.push(Comm::Half('b'));
    test_sys.execute_comms();
    assert_eq!(test_sys.reg_b, 2);
}

#[test]
fn triple_register_a() {
    let mut test_sys = Computer::new(3, 0);
    test_sys.comms.push(Comm::Triple('a'));
    test_sys.execute_comms();
    assert_eq!(test_sys.reg_a, 9);
}

#[test]
fn triple_register_b() {
    let mut test_sys = Computer::new(0, 10);
    test_sys.comms.push(Comm::Triple('b'));
    test_sys.execute_comms();
    assert_eq!(test_sys.reg_b, 30);
}

#[test]
fn increment_register_a() {
    let mut test_sys = Computer::new(0, 0);
    test_sys.comms.push(Comm::Increm('a'));
    test_sys.execute_comms();
    assert_eq!(test_sys.reg_a, 1);
}

#[test]
fn increment_register_b() {
    let mut test_sys = Computer::new(0, 100);
    test_sys.comms.push(Comm::Increm('b'));
    test_sys.execute_comms();
    assert_eq!(test_sys.reg_b, 101);
}

#[test]
fn jump_register_a() {
    let mut test_sys = Computer::new(0, 0);
    test_sys.comms.push(Comm::Jump(5));

    test_sys.comms.push(Comm::Increm('b'));
    test_sys.comms.push(Comm::Increm('b'));
    test_sys.comms.push(Comm::Increm('b'));
    test_sys.comms.push(Comm::Increm('b'));
    test_sys.comms.push(Comm::Increm('a'));

    test_sys.execute_comms();
    assert_eq!(test_sys.reg_a, 1);
    assert_eq!(test_sys.reg_b, 0);
}

#[test]
fn jump_register_b() {
    let mut test_sys = Computer::new(0, 0);
    test_sys.comms.push(Comm::Jump(4));

    test_sys.comms.push(Comm::Increm('a'));
    test_sys.comms.push(Comm::Increm('a'));
    test_sys.comms.push(Comm::Increm('a'));
    test_sys.comms.push(Comm::Increm('b'));

    test_sys.execute_comms();
    assert_eq!(test_sys.reg_b, 1);
    assert_eq!(test_sys.reg_a, 0);
}

#[test]
fn jump_if_even_register_a() {
    let mut test_sys = Computer::new(10, 0);
    test_sys.comms.push(Comm::JumpIfEven(('a', 3)));

    test_sys.comms.push(Comm::Increm('b'));
    test_sys.comms.push(Comm::Increm('b'));
    test_sys.comms.push(Comm::Increm('a'));

    test_sys.execute_comms();
    assert_eq!(test_sys.reg_a, 11);
    assert_eq!(test_sys.reg_b, 0);
}

#[test]
fn jump_if_even_register_b() {
    let mut test_sys = Computer::new(0, 3);
    test_sys.comms.push(Comm::JumpIfEven(('b', 3)));

    test_sys.comms.push(Comm::Increm('b'));
    test_sys.comms.push(Comm::Increm('b'));
    test_sys.comms.push(Comm::Increm('a'));

    test_sys.execute_comms();
    assert_eq!(test_sys.reg_a, 1);
    assert_eq!(test_sys.reg_b, 5);
}

#[test]
fn jump_if_one_register_a() {
    let mut test_sys = Computer::new(3, 0);
    test_sys.comms.push(Comm::JumpIfOne(('a', 2)));

    test_sys.comms.push(Comm::Increm('b'));
    test_sys.comms.push(Comm::Increm('a'));

    test_sys.execute_comms();
    assert_eq!(test_sys.reg_a, 4);
    assert_eq!(test_sys.reg_b, 1);
}

#[test]
fn jump_if_one_register_b() {
    let mut test_sys = Computer::new(0, 1);
    test_sys.comms.push(Comm::JumpIfOne(('b', 2)));

    test_sys.comms.push(Comm::Increm('b'));
    test_sys.comms.push(Comm::Increm('a'));

    test_sys.execute_comms();
    assert_eq!(test_sys.reg_b, 1);
    assert_eq!(test_sys.reg_a, 1);
}
