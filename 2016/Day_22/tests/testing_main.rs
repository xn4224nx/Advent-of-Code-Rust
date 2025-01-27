#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_22;
use day_22::{ComputingGrid, Status};

#[test]
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
        test.start_node_status,
        vec![
            Status::Full,
            Status::Full,
            Status::Blocked,
            Status::Full,
            Status::Empty,
            Status::Full,
            Status::Full,
            Status::Full,
            Status::Full
        ]
    );
}

#[test]
fn viable_swaps() {
    let test = ComputingGrid::new("./data/example_01.txt");
    assert_eq!(test.len_viable_swaps(&test.start_node_status), 7);
}
