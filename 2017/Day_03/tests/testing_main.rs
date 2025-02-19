#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_3;
use day_3::{coords_in_spiral, moves_to_exit};

#[test]
fn coordinates_of_steps_exp01() {
    assert_eq!(coords_in_spiral(1), (0, 0));
}

#[test]
fn coordinates_of_steps_exp02() {
    assert_eq!(coords_in_spiral(2), (1, 0));
}

#[test]
fn coordinates_of_steps_exp03() {
    assert_eq!(coords_in_spiral(3), (1, 1));
}

#[test]
fn coordinates_of_steps_exp04() {
    assert_eq!(coords_in_spiral(4), (0, 1));
}

#[test]
fn coordinates_of_steps_exp05() {
    assert_eq!(coords_in_spiral(5), (-1, 1));
}

#[test]
fn coordinates_of_steps_exp06() {
    assert_eq!(coords_in_spiral(7), (-1, -1));
}
#[test]
fn coordinates_of_steps_exp07() {
    assert_eq!(coords_in_spiral(8), (0, -1));
}
#[test]
fn coordinates_of_steps_exp08() {
    assert_eq!(coords_in_spiral(9), (1, -1));
}
#[test]
fn coordinates_of_steps_exp09() {
    assert_eq!(coords_in_spiral(13), (2, 2));
}
#[test]
fn coordinates_of_steps_exp10() {
    assert_eq!(coords_in_spiral(10), (2, -1));
}
#[test]
fn coordinates_of_steps_exp11() {
    assert_eq!(coords_in_spiral(25), (2, -2));
}
#[test]
fn coordinates_of_steps_exp12() {
    assert_eq!(coords_in_spiral(14), (1, 2));
}
#[test]
fn coordinates_of_steps_exp13() {
    assert_eq!(coords_in_spiral(15), (0, 2));
}
#[test]
fn coordinates_of_steps_exp14() {
    assert_eq!(coords_in_spiral(20), (-2, -1));
}
#[test]
fn coordinates_of_steps_exp15() {
    assert_eq!(coords_in_spiral(22), (-1, -2));
}

#[test]
fn coordinates_of_steps_exp16() {
    assert_eq!(coords_in_spiral(24), (1, -2));
}

#[test]
fn exit_moves_exp1() {
    assert_eq!(moves_to_exit(2), 1)
}
#[test]
fn exit_moves_exp2() {
    assert_eq!(moves_to_exit(12), 3)
}
#[test]
fn exit_moves_exp3() {
    assert_eq!(moves_to_exit(23), 2)
}
#[test]
fn exit_moves_exp4() {
    assert_eq!(moves_to_exit(1024), 31)
}
#[test]
fn exit_moves_exp5() {
    assert_eq!(moves_to_exit(1), 0)
}
