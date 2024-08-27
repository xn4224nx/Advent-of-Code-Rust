/*
 * --- Day 13: Knights of the Dinner Table ---
 *
 * In years past, the holiday feast with your family hasn't gone so well. Not
 * everyone gets along! This year, you resolve, will be different. You're going
 * to find the optimal seating arrangement and avoid all those awkward
 * conversations.
 *
 * You start by writing up a list of everyone invited and the amount their
 * happiness would increase or decrease if they were to find themselves sitting
 * next to each other person. You have a circular table that will be just big
 * enough to fit everyone comfortably, and so each person will have exactly two
 * neighbors.
 *
 * PART 1:  What is the total change in happiness for the optimal seating
 *          arrangement of the actual guest list?
 */

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Debug)]
pub enum FeelingChange {
    Gain,
    Lose,
    Neutral,
}

#[derive(PartialEq, Debug)]
pub struct Relationship {
    pub start: String,
    pub feel: FeelingChange,
    pub mag: u32,
    pub end: String,
}

/// Read the relationship file and return a vector of each person and a HashMap
/// of the relationships to one another.
pub fn read_guest_prefs(data_file: &str) -> (Vec<String>, HashMap<(String, String), Relationship>) {
    let mut guests = HashSet::new();
    let mut prefs = HashMap::new();

    /* Open the file. */
    let file = File::open(data_file).unwrap();
    let mut fp = BufReader::new(file);

    /* Each line strictly conforms to a set structure. */
    let re_rel = Regex::new(
        format!(
            r"{}{}{}",
            r"([a-zA-Z]+) would ([a-zA-Z]+) ",
            r"(\d+) happiness units by ",
            r"sitting next to ([a-zA-Z]+)"
        )
        .as_str(),
    )
    .unwrap();

    /* Each line to put in a buffer that is wiped after the line is processed. */
    let mut buffer = String::new();

    /* Try and parse each line of the file. */
    while fp.read_line(&mut buffer).unwrap() > 0 {
        let caps = re_rel.captures(&buffer).unwrap();

        /* Match the feeling to an enum variant. */
        let feel_parsed = match &caps[2] {
            "gain" => FeelingChange::Gain,
            "lose" => FeelingChange::Lose,
            _ => FeelingChange::Neutral,
        };

        let guest_1 = caps[1].to_string();
        let guest_2 = caps[4].to_string();

        /* Save how the guests feel about one another. */
        prefs.insert(
            (guest_1.clone(), guest_2.clone()),
            Relationship {
                start: guest_1.clone(),
                feel: feel_parsed,
                mag: caps[3].parse::<u32>().unwrap(),
                end: guest_2.clone(),
            },
        );

        /* Save a copy of the unique guests at the party. */
        guests.insert(guest_1);
        guests.insert(guest_2);

        buffer.clear();
    }

    return (Vec::from_iter(guests), prefs);
}

pub fn score_seating_arrange(
    guest_order: &Vec<String>,
    guest_prefs: &HashMap<(String, String), Relationship>,
) -> u32 {
    0
}

pub fn find_minimum_change(
    guest_order: &Vec<String>,
    guest_prefs: &HashMap<(String, String), Relationship>,
) -> u32 {
    0
}

fn main() {
    let (guests, rels) = read_guest_prefs("./data/input.txt");
}
