#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_14;
use day_14::{dist_travelled, read_reindeer_data, winning_dist, Reindeer};

#[test]
fn read_reindeer_data_ex_01() {
    assert_eq!(
        read_reindeer_data("./data/example_01.txt"),
        vec![
            Reindeer {
                speed: 14,
                run_time: 10,
                rest_time: 127
            },
            Reindeer {
                speed: 16,
                run_time: 11,
                rest_time: 162
            },
        ]
    );
}

#[test]
fn dist_travelled_comet() {
    assert_eq!(
        *dist_travelled(
            &Reindeer {
                speed: 14,
                run_time: 10,
                rest_time: 127
            },
            1000
        )
        .last()
        .unwrap(),
        1120
    )
}

#[test]
fn dist_travelled_dancer() {
    assert_eq!(
        *dist_travelled(
            &Reindeer {
                speed: 16,
                run_time: 11,
                rest_time: 162
            },
            1000
        )
        .last()
        .unwrap(),
        1056
    )
}

#[test]
fn winning_dist_ex_01() {
    assert_eq!(
        winning_dist(&read_reindeer_data("./data/example_01.txt"), 1000),
        1120
    )
}
