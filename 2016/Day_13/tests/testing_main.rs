#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_13;
use day_13::Maze;
use std::collections::HashSet;

#[test]
fn is_open_space_exp1() {
    let test = Maze::new(10);

    assert_eq!(test.is_open_space((0, 0)), true);
    assert_eq!(test.is_open_space((7, 4)), true);
    assert_eq!(test.is_open_space((7, 2)), true);
    assert_eq!(test.is_open_space((9, 5)), true);

    assert_eq!(test.is_open_space((1, 0)), false);
    assert_eq!(test.is_open_space((1, 4)), false);
    assert_eq!(test.is_open_space((5, 4)), false);
    assert_eq!(test.is_open_space((5, 4)), false);
}

#[test]
fn next_viable_moves_exp1() {
    let test = Maze::new(10);
    assert_eq!(
        HashSet::from_iter(test.next_viable_moves((2, 2))),
        HashSet::from([(1, 2), (3, 2)])
    );
    assert_eq!(
        HashSet::from_iter(test.next_viable_moves((7, 5))),
        HashSet::from([(7, 4), (6, 5)])
    );
    assert_eq!(
        HashSet::from_iter(test.next_viable_moves((7, 1))),
        HashSet::from([(7, 0), (7, 2), (6, 1), (8, 1)])
    );
    assert_eq!(
        HashSet::from_iter(test.next_viable_moves((0, 5))),
        HashSet::from([(0, 4), (1, 5)])
    );
}

#[test]
fn shortest_route_to_point_exp1() {
    let test = Maze::new(10);
    assert_eq!(test.shortest_route_to_point((7, 4)), 11);
}
