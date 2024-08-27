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
    Vec::new()
}

pub fn score_seating_arrange(guest_prefs: &Vec<Relationship>, guest_order: &Vec<String>) -> u32 {
    0
}

pub fn find_minimum_change(guest_prefs: &Vec<Relationship>, guest_order: &Vec<String>) -> u32 {
    0
}

fn main() {}
