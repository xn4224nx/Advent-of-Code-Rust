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

pub fn read_guest_prefs(data_file: &str) -> Vec<Relationship> {
    let file = File::open(data_file).unwrap();
    let mut fp = BufReader::new(file);

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

    let mut buffer = String::new();
    let mut prefs = Vec::new();

    while fp.read_line(&mut buffer).unwrap() > 0 {
        let caps = re_rel.captures(&buffer).unwrap();

        let feel_parsed = match &caps[2] {
            "gain" => FeelingChange::Gain,
            "lose" => FeelingChange::Lose,
            _ => FeelingChange::Neutral,
        };

        prefs.push(Relationship {
            start: caps[1].to_string(),
            feel: feel_parsed,
            mag: caps[3].parse::<u32>().unwrap(),
            end: caps[4].to_string(),
        });

        buffer.clear();
    }

    return prefs;
}

pub fn score_seating_arrange(guest_prefs: &Vec<Relationship>, guest_order: &Vec<String>) -> u32 {
    0
}

pub fn find_minimum_change(guest_prefs: &Vec<Relationship>, guest_order: &Vec<String>) -> u32 {
    0
}

fn main() {
    let rels = read_guest_prefs("./data/input.txt");
}
