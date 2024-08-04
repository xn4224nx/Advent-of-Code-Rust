#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_03;
use day_03::GridDir::{East, North, South, West};

#[test]
fn read_example_01() {
    assert_eq!(day_03::read_directions("./data/example_01.txt"), vec![East])
}

#[test]
fn read_example_02() {
    assert_eq!(
        day_03::read_directions("./data/example_02.txt"),
        vec![North, East, South, West]
    )
}

#[test]
fn read_example_03() {
    assert_eq!(
        day_03::read_directions("./data/example_03.txt"),
        vec![North, South, North, South, North, South, North, South, North, South]
    )
}

#[test]
fn read_example_04() {
    assert_eq!(
        day_03::read_directions("./data/example_04.txt"),
        vec![North, South]
    )
}

#[test]
fn unique_visits_ex_01() {
    assert_eq!(day_03::count_visited_houses(&vec![East], false), 2)
}

#[test]
fn unique_visits_ex_02() {
    assert_eq!(
        day_03::count_visited_houses(&vec![North, East, South, West], false),
        4
    )
}

#[test]
fn unique_visits_ex_03() {
    assert_eq!(
        day_03::count_visited_houses(
            &vec![North, South, North, South, North, South, North, South, North, South],
            false
        ),
        2
    )
}

#[test]
fn robot_visits_ex_02() {
    assert_eq!(
        day_03::count_visited_houses(&vec![North, East, South, West], true),
        3
    )
}

#[test]
fn robot_visits_ex_03() {
    assert_eq!(
        day_03::count_visited_houses(
            &vec![North, South, North, South, North, South, North, South, North, South],
            true
        ),
        11
    )
}

#[test]
fn robot_visits_ex_04() {
    assert_eq!(day_03::count_visited_houses(&vec![North, South], true), 3)
}
