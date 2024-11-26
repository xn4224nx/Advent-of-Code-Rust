#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_10;
use day_10::{Factory, Instruc};

#[test]
fn initialise_exp_1() {
    let mut test_fac = Factory::new("./data/example_01.txt");
    test_fac.initialise();

    assert_eq!(
        test_fac.instructions,
        vec![
            Instruc::Assign(2, 5),
            Instruc::Give(2, 1, 0, true, true),
            Instruc::Assign(1, 3),
            Instruc::Give(1, 1, 0, false, true),
            Instruc::Give(0, 2, 0, false, false),
            Instruc::Assign(2, 2)
        ]
    );
    assert_eq!(test_fac.bots, vec![Vec::new(), Vec::new(), Vec::new()]);
    assert_eq!(test_fac.outputs, vec![Vec::new(), Vec::new(), Vec::new()]);
    assert_eq!(test_fac.used_instrs, vec![false; 6]);
}

#[test]
fn initialise_exp_2() {
    let mut test_fac = Factory::new("./data/example_02.txt");
    test_fac.initialise();

    assert_eq!(
        test_fac.instructions,
        vec![
            Instruc::Assign(0, 99),
            Instruc::Assign(1, 8),
            Instruc::Assign(2, 3),
            Instruc::Assign(3, 44),
            Instruc::Assign(5, 7),
            Instruc::Assign(10, 1),
            Instruc::Assign(5, 6),
            Instruc::Assign(10, 2),
        ]
    );
    assert_eq!(test_fac.bots, vec![Vec::new(); 11]);
    assert_eq!(test_fac.outputs, vec![Vec::new()]);
    assert_eq!(test_fac.used_instrs, vec![false; 8]);
}

#[test]
fn initialise_exp_3() {
    let mut test_fac = Factory::new("./data/example_03.txt");
    test_fac.initialise();

    assert_eq!(
        test_fac.instructions,
        vec![
            Instruc::Assign(0, 9),
            Instruc::Assign(1, 10),
            Instruc::Assign(3, 11),
            Instruc::Assign(3, 12),
            Instruc::Give(3, 1, 0, true, true),
            Instruc::Give(1, 1, 2, false, true),
            Instruc::Give(0, 2, 0, true, false),
        ]
    );
    assert_eq!(test_fac.bots, vec![Vec::new(); 4]);
    assert_eq!(test_fac.outputs, vec![Vec::new(); 2]);
    assert_eq!(test_fac.used_instrs, vec![false; 7]);
}

#[test]
fn assign_method() {
    let mut test_fac = Factory::new("");
    test_fac.bots = vec![Vec::new(); 4];

    test_fac.assign(0, 10);
    test_fac.assign(1, 20);
    test_fac.assign(0, 30);
    test_fac.assign(3, 40);
    test_fac.assign(1, 40);

    assert_eq!(
        test_fac.bots,
        vec![vec![10, 30], vec![20, 40], vec![], vec![40]]
    );
}

#[test]
fn give_method_bot() {
    let mut test_fac = Factory::new("");
    test_fac.bots = vec![vec![3, 4], Vec::new(), Vec::new()];
    test_fac.give(0, 1, 2, true, true);
    assert_eq!(test_fac.bots, vec![Vec::new(), vec![3], vec![4]]);
}

#[test]
fn give_method_output() {
    let mut test_fac = Factory::new("");
    test_fac.bots = vec![vec![10, 1], vec![8, 9]];
    test_fac.outputs = vec![Vec::new(), Vec::new(), Vec::new()];

    test_fac.give(0, 1, 2, false, false);
    test_fac.give(1, 0, 2, false, false);

    assert_eq!(test_fac.outputs, vec![vec![8], vec![1], vec![10, 9]]);
    assert_eq!(test_fac.bots, vec![Vec::new(), Vec::new()]);
}

#[test]
fn give_method_mixed() {
    let mut test_fac = Factory::new("");
    test_fac.bots = vec![vec![11, 34], vec![19, 18], vec![1]];
    test_fac.outputs = vec![Vec::new(), Vec::new(), Vec::new()];

    test_fac.give(0, 2, 2, true, false);
    test_fac.give(2, 0, 1, false, false);

    assert_eq!(test_fac.outputs, vec![vec![1], vec![11], vec![34]]);
    assert_eq!(test_fac.bots, vec![Vec::new(), vec![19, 18], Vec::new()]);
}

#[test]
fn execute_all_exp_01() {
    let mut test_fac = Factory::new("./data/example_01.txt");
    test_fac.initialise();
    assert_eq!(test_fac.execute_all(3, 5), 0);
}

#[test]
fn execute_all_exp_02() {
    let mut test_fac = Factory::new("./data/example_01.txt");
    test_fac.initialise();
    assert_eq!(test_fac.execute_all(5, 3), 0);
}

#[test]
fn execute_all_exp_03() {
    let mut test_fac = Factory::new("./data/example_01.txt");
    test_fac.initialise();
    assert_eq!(test_fac.execute_all(2, 3), 1);
}

#[test]
fn execute_all_exp_04() {
    let mut test_fac = Factory::new("./data/example_01.txt");
    test_fac.initialise();
    assert_eq!(test_fac.execute_all(3, 2), 1);
}

#[test]
fn execute_all_exp_05() {
    let mut test_fac = Factory::new("./data/example_01.txt");
    test_fac.initialise();
    assert_eq!(test_fac.execute_all(2, 5), 2);
}

#[test]
fn execute_all_exp_06() {
    let mut test_fac = Factory::new("./data/example_01.txt");
    test_fac.initialise();
    assert_eq!(test_fac.execute_all(5, 2), 2);
}

#[test]
fn execute_all_final_state_exp1() {
    let mut test_fac = Factory::new("./data/example_01.txt");
    test_fac.initialise();
    let _ = test_fac.execute_all(5, 2);

    assert_eq!(test_fac.outputs, vec![vec![5], vec![2], vec![3],]);
    assert_eq!(test_fac.bots, vec![Vec::new(), Vec::new(), Vec::new()]);
}

#[test]
fn execute_all_final_state_exp2() {
    let mut test_fac = Factory::new("./data/example_02.txt");
    test_fac.initialise();
    let _ = test_fac.execute_all(0, 0);

    assert_eq!(test_fac.outputs, vec![Vec::new()]);
    assert_eq!(
        test_fac.bots,
        vec![
            vec![99],
            vec![8],
            vec![3],
            vec![44],
            Vec::new(),
            vec![7, 6],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![1, 2],
        ]
    );
}

#[test]
fn execute_all_final_state_exp3() {
    let mut test_fac = Factory::new("./data/example_03.txt");
    test_fac.initialise();
    assert_eq!(test_fac.execute_all(12, 9), 0);

    assert_eq!(test_fac.outputs, vec![vec![12], vec![10]]);
    assert_eq!(
        test_fac.bots,
        vec![Vec::new(), Vec::new(), vec![11, 9], Vec::new()]
    );
}
