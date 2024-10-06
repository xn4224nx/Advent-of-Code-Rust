#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_01;
use day_01::{find_shortest_path, move_point, read_directions, Direc};

#[test]
fn test_read_directions_01() {
    assert_eq!(
        read_directions("./data/example_01"),
        vec![Direc::R(2), Direc::L(3)]
    );
}

#[test]
fn test_read_directions_02() {
    assert_eq!(
        read_directions("./data/example_02"),
        vec![Direc::R(2), Direc::R(2), Direc::R(2)]
    );
}

#[test]
fn test_read_directions_03() {
    assert_eq!(
        read_directions("./data/example_03"),
        vec![Direc::R(5), Direc::L(5), Direc::R(5), Direc::R(3)]
    );
}

#[test]
fn test_read_directions_04() {
    assert_eq!(
        read_directions("./data/example_04"),
        vec![Direc::R(8), Direc::R(4), Direc::R(4), Direc::R(8),]
    );
}

#[test]
fn test_find_shortest_path_1() {
    assert_eq!(find_shortest_path(&vec![Direc::R(2), Direc::L(3)]), 5)
}

#[test]
fn test_find_shortest_path_2() {
    assert_eq!(
        find_shortest_path(&vec![Direc::R(2), Direc::R(2), Direc::R(2)]),
        2
    )
}

#[test]
fn test_find_shortest_path_3() {
    assert_eq!(
        find_shortest_path(&vec![Direc::R(5), Direc::L(5), Direc::R(5), Direc::R(3)]),
        12
    )
}
