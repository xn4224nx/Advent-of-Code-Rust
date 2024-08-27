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

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Debug)]
enum FeelingChange {
    Gain,
    Lose,
}

#[derive(PartialEq, Debug)]
pub struct Relationship {
    start: usize,
    feel: FeelingChange,
    mag: i32,
    end: usize,
}

/// Read the relationship file and return a vector of each person and a HashMap
/// of the relationships to one another.
pub fn read_guest_prefs(data_file: &str) -> HashMap<(usize, usize), Relationship> {
    let mut guest_lookup = HashMap::new();
    let mut guest_idx: usize = 0;
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
            _ => panic!("Unknown feeling!"),
        };

        /* Determine the actual name of the guests. */
        let guest_nm_1 = &caps[1].to_string();
        let guest_nm_2 = &caps[4].to_string();

        /* Replace the guest name with a number derived from guest_idx */
        if !guest_lookup.contains_key(guest_nm_1) {
            guest_lookup.insert(guest_nm_1.clone(), guest_idx);
            guest_idx += 1
        }
        if !guest_lookup.contains_key(guest_nm_2) {
            guest_lookup.insert(guest_nm_2.clone(), guest_idx);
            guest_idx += 1
        }

        /* Extract the numerical guest indicators from the HashMap. */
        let g1_idx = guest_lookup.get(guest_nm_1).unwrap();
        let g2_idx = guest_lookup.get(guest_nm_2).unwrap();

        /* Save how the guests feel about one another. */
        prefs.insert(
            (*g1_idx, *g2_idx),
            Relationship {
                start: *g1_idx,
                feel: feel_parsed,
                mag: caps[3].parse::<i32>().unwrap(),
                end: *g2_idx,
            },
        );
        buffer.clear();
    }
    return prefs;
}

pub fn score_seating_arrange(
    guest_order: &Vec<usize>,
    guest_prefs: &HashMap<(usize, usize), Relationship>,
) -> i32 {
    let mut score = 0;
    let num_guests = guest_order.len();

    for idx in 0..num_guests {
        let guest_1 = guest_order[idx];
        let guest_2 = if idx == num_guests - 1 {
            guest_order[0]
        } else {
            guest_order[idx + 1]
        };

        /* Account for a person not having details. */
        if !guest_prefs.contains_key(&(guest_1, guest_2))
            || !guest_prefs.contains_key(&(guest_2, guest_1))
        {
            continue;
        };

        /* Lookup the details of the relationships between the guests. */
        let rel_1_2 = guest_prefs.get(&(guest_1, guest_2)).unwrap();
        let rel_2_1 = guest_prefs.get(&(guest_2, guest_1)).unwrap();

        /* Change the score based on the relationships. */
        match rel_2_1.feel {
            FeelingChange::Gain => score += rel_2_1.mag,
            FeelingChange::Lose => score -= rel_2_1.mag,
        };
        match rel_1_2.feel {
            FeelingChange::Gain => score += rel_1_2.mag,
            FeelingChange::Lose => score -= rel_1_2.mag,
        };
    }
    return score;
}

pub fn find_maximum_happy(
    guest_prefs: &HashMap<(usize, usize), Relationship>,
    add_my_seat: bool,
) -> i32 {
    let mut max_rel_score = 0;
    let max_guest_in_hash = guest_prefs.keys().map(|x| x.0).max().unwrap();

    /* Add an extra guest to the table */
    let num_guests = if add_my_seat {
        max_guest_in_hash + 2
    } else {
        max_guest_in_hash + 1
    };

    for guest_perm in (0..num_guests).permutations(num_guests) {
        let perm_score = score_seating_arrange(&guest_perm, &guest_prefs);

        if perm_score > max_rel_score {
            max_rel_score = perm_score;
        }
    }

    return max_rel_score;
}

fn main() {
    let guest_prefs = read_guest_prefs("./data/input.txt");
    println!("Part 1 = {}", find_maximum_happy(&guest_prefs, false));
    println!("Part 2 = {}", find_maximum_happy(&guest_prefs, true));
}
