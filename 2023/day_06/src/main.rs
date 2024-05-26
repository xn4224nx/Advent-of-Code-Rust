/*
* --- Day 6: Wait For It ----
*
* You get a sheet of paper (your puzzle input) that lists the time allowed for
* each race and also the best distance ever recorded in that race. To guarantee
* you win the grand prize, you need to make sure you go farther in each race
* than the current record holder.
*
* Holding down the button charges the boat, and releasing the button allows the
* boat to move. Boats move faster if their button was held longer, but time
* spent holding the button counts against the total race time. You can only hold
* the button at the start of the race, and boats don't move until the button is
* released.
*
* Your toy boat has a starting speed of zero millimeters per millisecond. For
* each whole millisecond you spend at the beginning of the race holding down the
* button, the boat's speed increases by one millimeter per millisecond.
*
* To see how much margin of error you have, determine the number of ways you can
* beat the record in each race.
*
* Part 1 - Determine the number of ways you could beat the record in each race.
*          What do you get if you multiply these numbers together?
*/

use regex::Regex;
use std::fs;
use std::iter::zip;

/// Read the race file and create a vector of the time and distances
fn read_races(file_path: &str) -> Vec<(u32, u32)> {
    let num_re = Regex::new(r"[\d]+").unwrap();
    let mut extracted_nums: Vec<Vec<u32>> = Vec::new();

    /* Read the file as a string. */
    let raw_file_contents =
        fs::read_to_string(file_path).expect("File '{file_path}' could not be read!");

    /* Parse the data in the string by iteration over all the lines. */
    for raw_line in raw_file_contents.lines() {
        /* Extract the numbers in the row. */
        extracted_nums.push(
            num_re
                .find_iter(raw_line)
                .filter_map(|x| x.as_str().parse::<u32>().ok())
                .collect(),
        );
    }

    /* Extract the distance and time into matching tuples. */
    let ret = zip(&extracted_nums[0], &extracted_nums[1])
        .map(|(x, y)| (*x, *y))
        .collect::<Vec<(u32, u32)>>();

    return ret;
}

fn main() {
    let example = read_races("./data/example.txt");
    let input = read_races("./data/input.txt");

    println!("\nexample races: {:?}\ninput races: {:?}\n", example, input);
}
