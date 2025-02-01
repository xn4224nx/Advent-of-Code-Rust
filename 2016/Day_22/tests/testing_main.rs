#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_22;
use day_22::{ComputingGrid, Status};

fn reading_data() {
    let test = ComputingGrid::new("./data/example_01.txt");
    assert_eq!(
        test.node_locs,
        vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2)
        ]
    );
    assert_eq!(
        test.node_status,
        vec![
            Status::Full,
            Status::Full,
            Status::Blocked,
            Status::Full,
            Status::Empty,
            Status::Full,
            Status::Goal,
            Status::Full,
            Status::Full
        ]
    );
    assert_eq!(test.goal_node_idx, 6);
    assert_eq!(test.dest_node_idx, 3);
    assert_eq!(test.empt_node_idx, 4);
}

#[test]
#[should_panic]
fn no_empty_node() {
    ComputingGrid::new("./data/example_03.txt");
}

#[test]
fn viable_swaps() {
    assert_eq!(
        ComputingGrid::new("./data/example_01.txt").len_viable_swaps(),
        7
    );
}

#[test]
fn viable_moves() {
    let test = ComputingGrid::new("./data/example_01.txt");
    assert_eq!(test.viable_moves(4), vec![1, 3, 5, 7]);
    assert_eq!(test.viable_moves(0), vec![1, 3]);
    assert_eq!(test.viable_moves(1), vec![0, 2, 4]);
    assert_eq!(test.viable_moves(8), vec![5, 7]);
}

#[test]
fn fewest_steps_to_dest() {
    assert_eq!(
        ComputingGrid::new("./data/example_01.txt").fewest_steps_to_dest(),
        1
    );
}

#[test]
fn fewest_steps_to_data_exp_1() {
    assert_eq!(
        ComputingGrid::new("./data/example_01.txt").fewest_steps_to_data(),
        7
    );
}

#[test]
fn fewest_steps_to_data_exp_2() {
    assert_eq!(
        ComputingGrid::new("./data/example_02.txt").fewest_steps_to_data(),
        225
    );
}
