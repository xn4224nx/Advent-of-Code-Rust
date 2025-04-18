#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_12;
use day_12::ProgramVillage;
use std::collections::{HashMap, HashSet};

#[test]
fn parse_data_exp01() {
    assert_eq!(
        ProgramVillage::new("./data/example_01.txt").links,
        HashMap::from([
            (0, HashSet::from([2])),
            (1, HashSet::new()),
            (2, HashSet::from([0, 3, 4])),
            (3, HashSet::from([2, 4])),
            (4, HashSet::from([2, 3, 6])),
            (5, HashSet::from([6])),
            (6, HashSet::from([4, 5])),
        ])
    );
}

#[test]
fn group_census_exp01() {
    assert_eq!(
        ProgramVillage::new("./data/example_01.txt").full_prog_group(0),
        HashSet::from([0, 2, 3, 4, 5, 6])
    );
}

#[test]
fn group_census_exp02() {
    assert_eq!(
        ProgramVillage::new("./data/example_01.txt").full_prog_group(1),
        HashSet::from([1])
    );
}

#[test]
fn group_census_exp03() {
    assert_eq!(
        ProgramVillage::new("./data/example_01.txt").full_prog_group(2),
        HashSet::from([0, 2, 3, 4, 5, 6])
    );
}

#[test]
fn group_census_exp04() {
    assert_eq!(
        ProgramVillage::new("./data/example_01.txt").full_prog_group(3),
        HashSet::from([0, 2, 3, 4, 5, 6])
    );
}

#[test]
fn group_census_exp05() {
    assert_eq!(
        ProgramVillage::new("./data/example_01.txt").full_prog_group(4),
        HashSet::from([0, 2, 3, 4, 5, 6])
    );
}

#[test]
fn group_census_exp06() {
    assert_eq!(
        ProgramVillage::new("./data/example_01.txt").full_prog_group(5),
        HashSet::from([0, 2, 3, 4, 5, 6])
    );
}

#[test]
fn group_census_exp07() {
    assert_eq!(
        ProgramVillage::new("./data/example_01.txt").full_prog_group(6),
        HashSet::from([0, 2, 3, 4, 5, 6])
    );
}

#[test]
fn total_groups_in_village() {
    assert_eq!(
        ProgramVillage::new("./data/example_01.txt").total_groups(),
        2
    );
}
