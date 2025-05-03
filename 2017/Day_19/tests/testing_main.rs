#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_19;
use day_19::Network;
use num::complex::Complex;
use std::collections::HashMap;

#[test]
fn new_class_exp01() {
    let mut test = Network::new("./data/example_01.txt");
    assert_eq!(test.path, vec![(5, 0)]);
    assert_eq!(test.direction, Complex::new(0, 1));
    assert_eq!(
        test.pipes,
        HashMap::from([
            ((5, 0), '|'),
            ((5, 1), '|'),
            ((5, 2), 'A'),
            ((5, 3), '|'),
            ((5, 4), '|'),
            ((5, 5), '+'),
            ((6, 5), 'B'),
            ((7, 5), '-'),
            ((8, 5), '+'),
            ((8, 4), '|'),
            ((8, 3), '-'),
            ((8, 2), '|'),
            ((8, 1), '+'),
            ((9, 1), '-'),
            ((10, 1), '-'),
            ((11, 1), '+'),
            ((11, 2), 'C'),
            ((11, 3), '|'),
            ((11, 4), '|'),
            ((11, 5), '+'),
            ((12, 5), '-'),
            ((13, 5), '-'),
            ((14, 5), '+'),
            ((14, 4), 'D'),
            ((14, 3), '+'),
            ((13, 3), '-'),
            ((12, 3), '-'),
            ((10, 3), 'E'),
            ((9, 3), '-'),
            ((8, 3), '-'),
            ((7, 3), '-'),
            ((6, 3), '-'),
            ((4, 3), '-'),
            ((3, 3), '-'),
            ((2, 3), '-'),
            ((1, 3), 'F'),
        ])
    );
}

#[test]
fn test_step_exp01() {
    let mut test = Network::new("./data/example_01.txt");
    test.path.push((5, 0));
    test.direction = Complex::new(0, 1);
    test.step();
    assert_eq!(*test.path.last().unwrap(), (5, 1));
    assert_eq!(test.direction, Complex::new(0, 1));
}

#[test]
fn test_step_exp02() {
    let mut test = Network::new("./data/example_01.txt");
    test.path.push((5, 1));
    test.direction = Complex::new(0, 1);
    test.step();
    assert_eq!(*test.path.last().unwrap(), (5, 2));
    assert_eq!(test.direction, Complex::new(0, 1));
}

#[test]
fn test_step_exp03() {
    let mut test = Network::new("./data/example_01.txt");
    test.path.push((5, 2));
    test.direction = Complex::new(0, 1);
    test.step();
    assert_eq!(*test.path.last().unwrap(), (5, 3));
    assert_eq!(test.direction, Complex::new(0, 1));
}

#[test]
fn test_step_exp04() {
    let mut test = Network::new("./data/example_01.txt");
    test.path.push((5, 3));
    test.direction = Complex::new(0, 1);
    test.step();
    assert_eq!(*test.path.last().unwrap(), (5, 4));
    assert_eq!(test.direction, Complex::new(0, 1));
}

#[test]
fn test_step_exp05() {
    let mut test = Network::new("./data/example_01.txt");
    test.path.push((5, 4));
    test.direction = Complex::new(0, 1);
    test.step();
    assert_eq!(*test.path.last().unwrap(), (5, 5));
    assert_eq!(test.direction, Complex::new(0, 1));
}

#[test]
fn test_step_exp06() {
    let mut test = Network::new("./data/example_01.txt");
    test.path.push((5, 5));
    test.direction = Complex::new(1, 0);
    test.step();
    assert_eq!(*test.path.last().unwrap(), (6, 5));
    assert_eq!(test.direction, Complex::new(1, 0));
}

#[test]
fn test_step_exp07() {
    let mut test = Network::new("./data/example_01.txt");
    test.path.push((6, 5));
    test.direction = Complex::new(1, 0);
    test.step();
    assert_eq!(*test.path.last().unwrap(), (7, 5));
    assert_eq!(test.direction, Complex::new(1, 0));
}

#[test]
fn test_step_exp08() {
    let mut test = Network::new("./data/example_01.txt");
    test.path.push((7, 5));
    test.direction = Complex::new(1, 0);
    test.step();
    assert_eq!(*test.path.last().unwrap(), (8, 5));
    assert_eq!(test.direction, Complex::new(1, 0));
}

#[test]
fn test_step_exp09() {
    let mut test = Network::new("./data/example_01.txt");
    test.path.push((8, 5));
    test.direction = Complex::new(0, -1);
    test.step();
    assert_eq!(*test.path.last().unwrap(), (8, 4));
    assert_eq!(test.direction, Complex::new(0, -1));
}

#[test]
fn test_step_exp10() {
    let mut test = Network::new("./data/example_01.txt");
    test.path.push((8, 4));
    test.direction = Complex::new(0, -1);
    test.step();
    assert_eq!(*test.path.last().unwrap(), (8, 3));
    assert_eq!(test.direction, Complex::new(0, -1));
}

#[test]
fn test_step_exp11() {
    let mut test = Network::new("./data/example_01.txt");
    test.path.push((8, 3));
    test.direction = Complex::new(0, -1);
    test.step();
    assert_eq!(*test.path.last().unwrap(), (8, 2));
    assert_eq!(test.direction, Complex::new(0, -1));
}

#[test]
fn test_step_exp12() {
    let mut test = Network::new("./data/example_01.txt");
    test.path.push((8, 2));
    test.direction = Complex::new(0, -1);
    test.step();
    assert_eq!(*test.path.last().unwrap(), (8, 1));
    assert_eq!(test.direction, Complex::new(0, -1));
}

#[test]
fn test_path_letters_exp01() {
    assert_eq!(
        Network::new("./data/example_01.txt").find_path(),
        (String::from("ABCDEF"), 38)
    );
}
