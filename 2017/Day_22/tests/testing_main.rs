#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_22;
use day_22::Infection;
use num::complex::Complex;
use std::collections::HashSet;

#[test]
fn new_infection_exp01() {
    let test = Infection::new("./data/input.txt");
    assert_eq!(test.carr_loc, (1, 1));
    assert_eq!(test.carr_dir, Complex::new(0, 1));
    assert_eq!(test.infected_nodes, HashSet::from([(2, 0), (0, 1),]));
}

#[test]
fn test_burst_exp01() {
    let mut test = Infection::new("./data/input.txt");
    test.burst();
    assert_eq!(test.carr_loc, (0, 1));
    assert_eq!(test.carr_dir, Complex::new(-1, 0));
    assert_eq!(
        test.infected_nodes,
        HashSet::from([(2, 0), (0, 1), (1, 1),])
    );
}

#[test]
fn test_burst_exp02() {
    let mut test = Infection::new("./data/input.txt");
    test.carr_loc = (0, 1);
    test.carr_dir = Complex::new(-1, 0);
    test.infected_nodes = HashSet::from([(2, 0), (0, 1), (1, 1)]);
    test.burst();
    assert_eq!(test.carr_loc, (0, 0));
    assert_eq!(test.carr_dir, Complex::new(0, 1));
    assert_eq!(test.infected_nodes, HashSet::from([(2, 0), (1, 1),]));
}

#[test]
fn test_burst_exp03() {
    let mut test = Infection::new("./data/input.txt");
    for _ in 0..6 {
        test.burst()
    }

    assert_eq!(test.carr_loc, (0, 0));
    assert_eq!(test.carr_dir, Complex::new(0, 1));
    assert_eq!(
        test.infected_nodes,
        HashSet::from([(0, 1), (-1, 0), (0, 0), (-1, 1), (1, 1), (2, 0),])
    );
}

#[test]
fn test_burst_exp04() {
    let mut test = Infection::new("./data/input.txt");
    for _ in 0..7 {
        test.burst()
    }

    assert_eq!(test.carr_loc, (1, 0));
    assert_eq!(test.carr_dir, Complex::new(1, 0));
    assert_eq!(
        test.infected_nodes,
        HashSet::from([(0, 1), (-1, 1), (1, 1), (2, 0), (-1, 0)])
    );
}

#[test]
fn test_burst_exp05() {
    let mut test = Infection::new("./data/input.txt");
    for _ in 0..70 {
        test.burst()
    }

    assert_eq!(test.carr_loc, (2, 0));
    assert_eq!(test.carr_dir, Complex::new(0, 1));
    assert_eq!(
        test.infected_nodes,
        HashSet::from([
            (5, -1),
            (-1, 0),
            (2, 2),
            (4, -2),
            (1, 0),
            (3, 2),
            (4, 1),
            (2, -3),
            (-1, 1),
            (1, 1),
            (3, -3),
            (1, -2),
            (5, 0),
            (0, -1),
        ])
    );
}
