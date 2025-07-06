#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod main;
use crate::main::Fabric;

use std::collections::{HashMap, HashSet};

#[test]
fn read_claims_exp0() {
    assert_eq!(
        Fabric::new("./data/example_0.txt").claims,
        HashMap::from([
            ((3, 1), HashSet::from([2])),
            ((4, 1), HashSet::from([2])),
            ((5, 1), HashSet::from([2])),
            ((6, 1), HashSet::from([2])),
            ((3, 2), HashSet::from([2])),
            ((4, 2), HashSet::from([2])),
            ((5, 2), HashSet::from([2])),
            ((6, 2), HashSet::from([2])),
            ((1, 3), HashSet::from([1])),
            ((2, 3), HashSet::from([1])),
            ((3, 3), HashSet::from([1, 2])),
            ((4, 3), HashSet::from([1, 2])),
            ((5, 3), HashSet::from([2])),
            ((6, 3), HashSet::from([2])),
            ((1, 4), HashSet::from([1])),
            ((2, 4), HashSet::from([1])),
            ((3, 4), HashSet::from([1, 2])),
            ((4, 4), HashSet::from([1, 2])),
            ((5, 4), HashSet::from([2])),
            ((6, 4), HashSet::from([2])),
            ((1, 5), HashSet::from([1])),
            ((2, 5), HashSet::from([1])),
            ((3, 5), HashSet::from([1])),
            ((4, 5), HashSet::from([1])),
            ((5, 5), HashSet::from([3])),
            ((6, 5), HashSet::from([3])),
            ((1, 6), HashSet::from([1])),
            ((2, 6), HashSet::from([1])),
            ((3, 6), HashSet::from([1])),
            ((4, 6), HashSet::from([1])),
            ((5, 6), HashSet::from([3])),
            ((6, 6), HashSet::from([3])),
        ])
    );
}

#[test]
fn overlaping_sqrs_exp0() {
    assert_eq!(Fabric::new("./data/example_0.txt").overlapping_sqrs(), 4);
}

#[test]
fn non_overlapping_claim_exp0() {
    assert_eq!(
        Fabric::new("./data/example_0.txt").find_non_overlapping_claim(),
        3
    );
}
