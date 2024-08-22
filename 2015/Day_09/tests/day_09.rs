#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_09;
use day_09::{find_shortest_path, read_dist_data};
use std::collections::HashMap;

#[test]
fn read_example_01() {
    assert_eq!(
        read_dist_data("./data/example_01.txt"),
        HashMap::from([
            (
                "London".to_string(),
                HashMap::from([("Belfast".to_string(), 518), ("Dublin".to_string(), 464)])
            ),
            (
                "Belfast".to_string(),
                HashMap::from([("London".to_string(), 518), ("Dublin".to_string(), 141)])
            ),
            (
                "Dublin".to_string(),
                HashMap::from([("London".to_string(), 464), ("Belfast".to_string(), 141)])
            ),
        ])
    )
}

#[test]
fn shortest_route_expl_01() {
    let data = read_dist_data("./data/example_01.txt");
    assert_eq!(find_shortest_path(&data), 605);
}
