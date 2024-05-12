/*
* --- Day 4: Scratchcards ---
*
* It looks like each card has two lists of numbers separated by a vertical bar
* (|): a list of winning numbers and then a list of numbers you have.
*
* As far as the Elf has been able to figure out, you have to figure out which
* of the numbers you have appear in the list of winning numbers. The first match
* makes the card worth one point and each match after the first doubles the
* point value of that card.
*
* Part 1: How many points are they worth in total?
*/

use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Read the scratch card file into a structured form
fn read_scratch_cards(file_path: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    let mut all_scratch_cards = Vec::new();
    let file_ptr = File::open(file_path).expect("File could not be opened.");
    let reader = BufReader::new(file_ptr);

    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let re_pat = Regex::new(r"\d+").unwrap();

    for raw_line in reader.lines() {
        /* Extract the line or stop reading the file. */
        let Ok(line) = raw_line else { break };

        /* Split by | to obtain two string vectors with the numbers. */
        let line_parts: Vec<&str> = line.split("|").collect();

        /* Extract the winning numbers (ignore the card No.). */
        let win_num: Vec<u32> = re_pat
            .find_iter(&line_parts[0])
            .skip(1)
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect();

        /* Extract the users numbers. */
        let usr_num: Vec<u32> = re_pat
            .find_iter(&line_parts[1])
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect();

        all_scratch_cards.push((win_num, usr_num))
    }

    return all_scratch_cards;
}

/// Calculate the total points for a vector of scratch cards
fn all_scratch_point_total(pile_of_cards: &Vec<(Vec<u32>, Vec<u32>)>) -> u32 {
    return pile_of_cards
        .iter()
        .map(|x| matches_2_pnts(scrt_matches_total(x)))
        .sum();
}

/// Calculate the total points for one scratch card
fn scrt_matches_total(card: &(Vec<u32>, Vec<u32>)) -> u32 {
    let win_num: HashSet<&u32> = HashSet::from_iter(&card.0);
    let usr_num: HashSet<&u32> = HashSet::from_iter(&card.1);
    return usr_num.intersection(&win_num).count() as u32;
}

/// Convert the number of matches to points
fn matches_2_pnts(matches: u32) -> u32 {
    return if matches != 0 {
        u32::pow(2, matches - 1)
    } else {
        matches
    };
}

fn main() {
    let card_pile = read_scratch_cards("./data/input.txt");
    println!(
        "The answer to part 1 = {}",
        all_scratch_point_total(&card_pile)
    );
}
