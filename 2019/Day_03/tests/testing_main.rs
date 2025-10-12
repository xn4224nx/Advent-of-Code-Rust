#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod main;
use main::WireInteraction;
use std::collections::HashMap;

#[test]
fn new_pair_of_wires_exp0() {
    let test = WireInteraction::from_file("./data/example_0.txt");

    assert_eq!(
        test.wire_loc_dists,
        vec![
            HashMap::from([
                ((1, 0), 1),
                ((2, 0), 2),
                ((3, 0), 3),
                ((4, 0), 4),
                ((5, 0), 5),
                ((6, 0), 6),
                ((7, 0), 7),
                ((8, 0), 8),
                ((8, 1), 9),
                ((8, 2), 10),
                ((8, 3), 11),
                ((8, 4), 12),
                ((8, 5), 13),
                ((7, 5), 14),
                ((6, 5), 15),
                ((5, 5), 16),
                ((4, 5), 17),
                ((3, 5), 18),
                ((3, 4), 19),
                ((3, 3), 20),
                ((3, 2), 21),
            ]),
            HashMap::from([
                ((0, 1), 1),
                ((0, 2), 2),
                ((0, 3), 3),
                ((0, 4), 4),
                ((0, 5), 5),
                ((0, 6), 6),
                ((0, 7), 7),
                ((1, 7), 8),
                ((2, 7), 9),
                ((3, 7), 10),
                ((4, 7), 11),
                ((5, 7), 12),
                ((6, 7), 13),
                ((6, 6), 14),
                ((6, 5), 15),
                ((6, 4), 16),
                ((6, 3), 17),
                ((5, 3), 18),
                ((4, 3), 19),
                ((3, 3), 20),
                ((2, 3), 21),
            ])
        ]
    );
}

#[test]
fn test_short_circuit_dist_exp_0() {
    assert_eq!(
        WireInteraction::from_file("./data/example_0.txt").short_circuit_dist(false),
        6
    );
}

#[test]
fn test_short_circuit_dist_exp_1() {
    assert_eq!(
        WireInteraction::from_file("./data/example_1.txt").short_circuit_dist(false),
        159
    );
}

#[test]
fn test_short_circuit_dist_exp_2() {
    assert_eq!(
        WireInteraction::from_file("./data/example_2.txt").short_circuit_dist(false),
        135
    );
}

#[test]
fn test_short_circuit_dist_exp_3() {
    assert_eq!(
        WireInteraction::from_file("./data/example_0.txt").short_circuit_dist(true),
        30
    );
}

#[test]
fn test_short_circuit_dist_exp_4() {
    assert_eq!(
        WireInteraction::from_file("./data/example_1.txt").short_circuit_dist(true),
        610
    );
}

#[test]
fn test_short_circuit_dist_exp_5() {
    assert_eq!(
        WireInteraction::from_file("./data/example_2.txt").short_circuit_dist(true),
        410
    );
}
