#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_11;
use day_11::{
    create_next_states, find_min_move_to_top, is_state_finished, is_state_safe,
    read_generator_state,
};
use std::collections::HashSet;

#[test]
fn read_generator_state_exp_1() {
    assert_eq!(
        read_generator_state("./data/example_01.txt"),
        vec![0, 1, 2, 0, 0]
    );
}

#[test]
fn read_generator_state_input() {
    assert_eq!(
        read_generator_state("./data/input.txt"),
        vec![0, 1, 1, 1, 0, 1, 2, 2, 2, 0, 2]
    );
}

#[test]
fn create_next_states_ground() {
    let test: HashSet<Vec<u8>> = create_next_states(&vec![0, 0, 0, 0, 0])
        .into_iter()
        .collect();
    let truth: HashSet<Vec<u8>> = vec![
        vec![1, 1, 1, 0, 0],
        vec![1, 0, 0, 1, 1],
        vec![1, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![1, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1],
    ]
    .into_iter()
    .collect();
    assert_eq!(test, truth);
}

#[test]
fn create_next_states_middle() {
    let test: HashSet<Vec<u8>> = create_next_states(&vec![1, 1, 1, 1, 1])
        .into_iter()
        .collect();
    let truth: HashSet<Vec<u8>> = vec![
        vec![0, 0, 0, 1, 1],
        vec![0, 1, 1, 0, 0],
        vec![0, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0],
        vec![0, 1, 1, 0, 1],
        vec![0, 1, 1, 1, 0],
        vec![2, 1, 1, 2, 2],
        vec![2, 2, 2, 1, 1],
        vec![2, 2, 1, 2, 1],
        vec![2, 1, 2, 1, 2],
        vec![2, 1, 1, 2, 1],
        vec![2, 1, 1, 1, 2],
    ]
    .into_iter()
    .collect();

    assert_eq!(test, truth);
}

#[test]
fn create_next_states_top() {
    let test: HashSet<Vec<u8>> = create_next_states(&vec![3, 3, 3, 3, 3])
        .into_iter()
        .collect();
    let truth: HashSet<Vec<u8>> = vec![
        vec![3, 2, 2, 3, 3],
        vec![3, 3, 3, 2, 2],
        vec![3, 2, 3, 2, 3],
        vec![3, 3, 2, 3, 2],
        vec![3, 3, 3, 2, 3],
        vec![3, 3, 3, 3, 2],
    ]
    .into_iter()
    .collect();

    assert_eq!(test, truth);
}

#[test]
fn is_state_safe_exp1() {
    assert_eq!(is_state_safe(&vec![3, 3, 3, 3, 3]), true);
    assert_eq!(is_state_safe(&vec![0, 3, 3, 1, 1]), true);
    assert_eq!(is_state_safe(&vec![0, 2, 3, 1, 1]), true);
    assert_eq!(is_state_safe(&vec![0, 3, 3, 1, 3]), true);
    assert_eq!(is_state_safe(&vec![0, 3, 1, 1, 3]), false);
}

#[test]
fn is_state_safe_exp2() {
    assert_eq!(is_state_safe(&vec![0, 3, 3, 2, 2]), true);
    assert_eq!(is_state_safe(&vec![0, 0, 0, 0, 2]), true);
    assert_eq!(is_state_safe(&vec![0, 2, 1, 0, 2]), false);
    assert_eq!(is_state_safe(&vec![0, 2, 0, 1, 2]), false);
    assert_eq!(is_state_safe(&vec![3, 3, 2, 3, 3]), false);
}

#[test]
fn is_state_finished_exp1() {
    assert_eq!(is_state_finished(&vec![3, 3, 3, 3, 3]), true);
}

#[test]
fn is_state_finished_exp2() {
    assert_eq!(is_state_finished(&vec![2, 3, 3, 3, 3]), false);
}

#[test]
fn is_state_finished_exp3() {
    assert_eq!(is_state_finished(&vec![3, 3, 3, 3, 3, 3, 2]), false);
}

#[test]
fn find_min_move_to_top_exp1() {
    assert_eq!(find_min_move_to_top(&vec![0, 1, 2, 0, 0]), 11);
}
