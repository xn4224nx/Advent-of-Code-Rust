#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_01;
use day_01::{find_shortest_path, read_directions, Direc};

#[test]
fn test_read_directions_01() {
    assert_eq!(
        read_directions("./data/example_01.txt"),
        vec![Direc::R(2), Direc::L(3)]
    );
}

#[test]
fn test_read_directions_02() {
    assert_eq!(
        read_directions("./data/example_02.txt"),
        vec![Direc::R(2), Direc::R(2), Direc::R(2)]
    );
}

#[test]
fn test_read_directions_03() {
    assert_eq!(
        read_directions("./data/example_03.txt"),
        vec![Direc::R(5), Direc::L(5), Direc::R(5), Direc::R(3)]
    );
}

#[test]
fn test_read_directions_04() {
    assert_eq!(
        read_directions("./data/example_04.txt"),
        vec![Direc::R(8), Direc::R(4), Direc::R(4), Direc::R(8),]
    );
}

#[test]
fn test_find_shortest_path_1() {
    assert_eq!(
        find_shortest_path(&vec![Direc::R(2), Direc::L(3)], false),
        5
    )
}

#[test]
fn test_find_shortest_path_2() {
    assert_eq!(
        find_shortest_path(&vec![Direc::R(2), Direc::R(2), Direc::R(2)], false),
        2
    )
}

#[test]
fn test_find_shortest_path_3() {
    assert_eq!(
        find_shortest_path(
            &vec![Direc::R(5), Direc::L(5), Direc::R(5), Direc::R(3)],
            false
        ),
        12
    )
}

#[test]
fn test_find_shortest_path_4() {
    assert_eq!(
        find_shortest_path(
            &vec![Direc::R(8), Direc::R(4), Direc::R(4), Direc::R(8),],
            true
        ),
        4
    )
}
