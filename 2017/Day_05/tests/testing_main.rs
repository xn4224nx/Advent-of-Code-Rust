#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_5;
use day_5::Instructions;

#[test]
fn read_data_exp01() {
    assert_eq!(
        Instructions::new("./data/example_01.txt", false),
        Instructions {
            jumps: vec![0, 3, 0, 1, -3],
            jmp_idx: 0,
            strange: false
        }
    );
}

#[test]
fn execute_step_exp01() {
    let mut test = Instructions {
        jumps: vec![0, 3, 0, 1, -3],
        jmp_idx: 0,
        strange: false,
    };
    test.execute_curr();
    assert_eq!(
        test,
        Instructions {
            jumps: vec![1, 3, 0, 1, -3],
            jmp_idx: 0,
            strange: false
        }
    );
}

#[test]
fn execute_step_exp02() {
    let mut test = Instructions {
        jumps: vec![1, 3, 0, 1, -3],
        jmp_idx: 0,
        strange: false,
    };
    test.execute_curr();
    assert_eq!(
        test,
        Instructions {
            jumps: vec![2, 3, 0, 1, -3],
            jmp_idx: 1,
            strange: false
        }
    );
}

#[test]
fn execute_step_exp03() {
    let mut test = Instructions {
        jumps: vec![2, 3, 0, 1, -3],
        jmp_idx: 1,
        strange: false,
    };
    test.execute_curr();
    assert_eq!(
        test,
        Instructions {
            jumps: vec![2, 4, 0, 1, -3],
            jmp_idx: 4,
            strange: false
        }
    );
}

#[test]
fn execute_step_exp04() {
    let mut test = Instructions {
        jumps: vec![2, 4, 0, 1, -3],
        jmp_idx: 4,
        strange: false,
    };
    test.execute_curr();
    assert_eq!(
        test,
        Instructions {
            jumps: vec![2, 4, 0, 1, -2,],
            jmp_idx: 1,
            strange: false
        }
    );
}

#[test]
fn execute_step_exp05() {
    let mut test = Instructions {
        jumps: vec![2, 4, 0, 1, -2],
        jmp_idx: 1,
        strange: false,
    };
    test.execute_curr();
    assert_eq!(
        test,
        Instructions {
            jumps: vec![2, 5, 0, 1, -2],
            jmp_idx: 5,
            strange: false
        }
    );
}

#[test]
fn steps_to_exit_exp1() {
    assert_eq!(
        Instructions::new("./data/example_01.txt", false).steps_to_exit(),
        5
    );
}

#[test]
fn steps_to_exit_exp2() {
    let mut test = Instructions::new("./data/example_01.txt", true);
    assert_eq!(test.steps_to_exit(), 10);
}

#[test]
fn steps_to_exit_exp3() {
    let mut test = Instructions::new("./data/example_01.txt", true);
    test.steps_to_exit();
    assert_eq!(test.jumps, vec![2, 3, 2, 3, -1]);
}
