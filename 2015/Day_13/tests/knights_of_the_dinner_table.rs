#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_13;
use day_13::{
    find_minimum_change, read_guest_prefs, score_seating_arrange, FeelingChange, Relationship,
};

#[test]
fn read_guest_prefs_ex_01() {
    assert_eq!(
        read_guest_prefs("./data/example_01.txt"),
        vec![
            Relationship {
                start: String::from("Alice"),
                feel: FeelingChange::Gain,
                mag: 54,
                end: String::from("Bob"),
            },
            Relationship {
                start: String::from("Alice"),
                feel: FeelingChange::Gain,
                mag: 79,
                end: String::from("Carol"),
            },
            Relationship {
                start: String::from("Alice"),
                feel: FeelingChange::Gain,
                mag: 2,
                end: String::from("David"),
            },
            Relationship {
                start: String::from("Bob"),
                feel: FeelingChange::Gain,
                mag: 83,
                end: String::from("Alice"),
            },
            Relationship {
                start: String::from("Bob"),
                feel: FeelingChange::Gain,
                mag: 7,
                end: String::from("Carol"),
            },
            Relationship {
                start: String::from("Bob"),
                feel: FeelingChange::Gain,
                mag: 63,
                end: String::from("David"),
            },
            Relationship {
                start: String::from("Carol"),
                feel: FeelingChange::Gain,
                mag: 62,
                end: String::from("Alice"),
            },
            Relationship {
                start: String::from("Carol"),
                feel: FeelingChange::Gain,
                mag: 60,
                end: String::from("Bob"),
            },
            Relationship {
                start: String::from("Carol"),
                feel: FeelingChange::Gain,
                mag: 55,
                end: String::from("David"),
            },
            Relationship {
                start: String::from("David"),
                feel: FeelingChange::Gain,
                mag: 46,
                end: String::from("Bob"),
            },
            Relationship {
                start: String::from("David"),
                feel: FeelingChange::Gain,
                mag: 7,
                end: String::from("Bob"),
            },
            Relationship {
                start: String::from("David"),
                feel: FeelingChange::Gain,
                mag: 41,
                end: String::from("Carol"),
            },
        ]
    );
}

#[test]
fn test_score_seating_arrange_ex_01() {
    let data = read_guest_prefs("./data/example_01.txt");
    assert_eq!(
        score_seating_arrange(
            &data,
            &vec![
                String::from("Alice"),
                String::from("Bob"),
                String::from("Carol"),
                String::from("David"),
            ]
        ),
        330
    );
}

#[test]
fn test_find_minimum_change_ex_01() {
    let data = read_guest_prefs("./data/example_01.txt");
    assert_eq!(
        score_seating_arrange(
            &data,
            &vec![
                String::from("Bob"),
                String::from("Carol"),
                String::from("David"),
                String::from("Alice"),
            ]
        ),
        330
    );
}
