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

fn main() {
    println!("{:?}", read_scratch_cards("./data/Example_0.txt"));
}
