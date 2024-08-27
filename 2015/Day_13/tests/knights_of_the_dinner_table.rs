#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_13;
use day_13::{
    find_minimum_change, read_guest_prefs, score_seating_arrange, FeelingChange, Relationship,
};

use std::collections::{HashMap, HashSet};

#[test]
fn read_guest_prefs_ex_01() {
    assert_eq!(
        read_guest_prefs("./data/example_01.txt").1,
        HashMap::from([
            (
                (String::from("Alice"), String::from("Bob")),
                Relationship {
                    start: String::from("Alice"),
                    feel: FeelingChange::Gain,
                    mag: 54,
                    end: String::from("Bob"),
                }
            ),
            (
                (String::from("Alice"), String::from("Carol")),
                Relationship {
                    start: String::from("Alice"),
                    feel: FeelingChange::Lose,
                    mag: 79,
                    end: String::from("Carol"),
                }
            ),
            (
                (String::from("Alice"), String::from("David")),
                Relationship {
                    start: String::from("Alice"),
                    feel: FeelingChange::Lose,
                    mag: 2,
                    end: String::from("David"),
                }
            ),
            (
                (String::from("Bob"), String::from("Alice")),
                Relationship {
                    start: String::from("Bob"),
                    feel: FeelingChange::Gain,
                    mag: 83,
                    end: String::from("Alice"),
                }
            ),
            (
                (String::from("Bob"), String::from("Carol")),
                Relationship {
                    start: String::from("Bob"),
                    feel: FeelingChange::Lose,
                    mag: 7,
                    end: String::from("Carol"),
                }
            ),
            (
                (String::from("Bob"), String::from("David")),
                Relationship {
                    start: String::from("Bob"),
                    feel: FeelingChange::Lose,
                    mag: 63,
                    end: String::from("David"),
                }
            ),
            (
                (String::from("Carol"), String::from("Alice")),
                Relationship {
                    start: String::from("Carol"),
                    feel: FeelingChange::Lose,
                    mag: 62,
                    end: String::from("Alice"),
                }
            ),
            (
                (String::from("Carol"), String::from("Bob")),
                Relationship {
                    start: String::from("Carol"),
                    feel: FeelingChange::Gain,
                    mag: 60,
                    end: String::from("Bob"),
                }
            ),
            (
                (String::from("Carol"), String::from("David")),
                Relationship {
                    start: String::from("Carol"),
                    feel: FeelingChange::Gain,
                    mag: 55,
                    end: String::from("David"),
                }
            ),
            (
                (String::from("David"), String::from("Alice")),
                Relationship {
                    start: String::from("David"),
                    feel: FeelingChange::Gain,
                    mag: 46,
                    end: String::from("Alice"),
                }
            ),
            (
                (String::from("David"), String::from("Bob")),
                Relationship {
                    start: String::from("David"),
                    feel: FeelingChange::Lose,
                    mag: 7,
                    end: String::from("Bob"),
                }
            ),
            (
                (String::from("David"), String::from("Carol")),
                Relationship {
                    start: String::from("David"),
                    feel: FeelingChange::Gain,
                    mag: 41,
                    end: String::from("Carol"),
                }
            ),
        ])
    );
}

#[test]
fn read_guest_list_ex_01() {
    let (guests, _) = read_guest_prefs("./data/example_01.txt");

    assert_eq!(guests.len(), 4);
    assert_eq!(
        HashSet::<String>::from_iter(guests),
        HashSet::from_iter(vec![
            String::from("Alice"),
            String::from("Bob"),
            String::from("Carol"),
            String::from("David"),
        ])
    )
}

#[test]
fn test_score_seating_arrange_ex_01() {
    let (guests, prefs) = read_guest_prefs("./data/example_01.txt");
    assert_eq!(score_seating_arrange(&guests, &prefs,), 330);
}

#[test]
fn test_find_minimum_change_ex_01() {
    let (guests, prefs) = read_guest_prefs("./data/example_01.txt");
    assert_eq!(find_minimum_change(&guests, &prefs,), 330);
}
