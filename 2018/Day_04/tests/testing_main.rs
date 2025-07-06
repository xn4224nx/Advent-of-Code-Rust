#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod main;
use crate::main::SecuritySchedule;
use std::collections::HashMap;

#[test]
fn new_schedule_exp0() {
    assert_eq!(
        SecuritySchedule::new("./data/example_0.txt").midnight_sleeps,
        HashMap::from([
            (
                10,
                vec![
                    0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1,
                    1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 0, 0, 0, 0, 0,
                ]
            ),
            (
                99,
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 1, 1,
                    1, 1, 1, 0, 0, 0, 0, 0,
                ]
            ),
        ])
    );
}

#[test]
fn find_sleepiest_guard_exp0() {
    assert_eq!(
        SecuritySchedule::new("./data/example_0.txt").sleepiest_guard_id(),
        240
    );
}

#[test]
fn guard_most_reliable_asleep_exp0() {
    assert_eq!(
        SecuritySchedule::new("./data/example_0.txt").guard_most_reliable_asleep(),
        4455
    );
}
