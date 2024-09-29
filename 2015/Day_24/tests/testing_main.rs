#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_24;
use day_24::{find_ideal_config_qe, qe_calc, read_box_weights, valid_grouping};

#[test]
fn read_example_box_weights() {
    assert_eq!(
        read_box_weights("./data/example_01.txt"),
        vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11,]
    );
}

#[test]
fn read_input_box_weights() {
    assert_eq!(
        read_box_weights("./data/input.txt"),
        vec![
            1, 2, 3, 7, 11, 13, 17, 19, 23, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
            97, 101, 103, 107, 109, 113,
        ]
    );
}

#[test]
fn qe_calc_0() {
    assert_eq!(
        qe_calc(vec![vec![11, 9], vec![10, 8, 2], vec![7, 5, 4, 3, 1],]),
        99
    );
}

#[test]
fn qe_calc_1() {
    assert_eq!(
        qe_calc(vec![vec![10, 9, 1], vec![11, 7, 2], vec![8, 5, 4, 3],]),
        90
    );
}

#[test]
fn qe_calc_2() {
    assert_eq!(
        qe_calc(vec![vec![10, 4, 3, 2, 1], vec![11, 9], vec![8, 7, 5],]),
        240
    );
}

#[test]
fn find_ideal_config_qe_for_example() {
    let example_boxes = read_box_weights("./data/example_01.txt");
    assert_eq!(find_ideal_config_qe(&example_boxes, 3), 99);
}
