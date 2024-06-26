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
*
* As the race is about to start, you realize the piece of paper with race times
* and record distances you got earlier actually just has very bad kerning.
* There's really only one race - ignore the spaces between the numbers on each
* line.
*
* Part 2 - How many ways can you beat the record in this one much longer race?
*/

use regex::Regex;
use std::fs;
use std::iter::zip;

/// Read the race file and create a vector of the time and distances
fn read_races(file_path: &str) -> Vec<(u64, u64)> {
    let num_re = Regex::new(r"[\d]+").unwrap();
    let mut extracted_nums: Vec<Vec<u64>> = Vec::new();

    /* Read the file as a string. */
    let raw_file_contents =
        fs::read_to_string(file_path).expect("File '{file_path}' could not be read!");

    /* Parse the data in the string by iteration over all the lines. */
    for raw_line in raw_file_contents.lines() {
        /* Extract the numbers in the row. */
        extracted_nums.push(
            num_re
                .find_iter(raw_line)
                .filter_map(|x| x.as_str().parse::<u64>().ok())
                .collect(),
        );
    }

    /* Extract the distance and time into matching tuples. */
    let ret = zip(&extracted_nums[0], &extracted_nums[1])
        .map(|(x, y)| (*x, *y))
        .collect::<Vec<(u64, u64)>>();

    return ret;
}

/// Determine the number of ways a race could be won
fn ways_to_win(race_dets: &(u64, u64)) -> u64 {
    let race_time = race_dets.0;
    let max_dist = race_dets.1;
    let mut ways_to_win = 0;

    /* Determine for each time what the distance would be.  */
    for time in 0..=race_time {
        if race_distance(time, race_time) > max_dist {
            ways_to_win += 1;
        }
    }
    return ways_to_win;
}

/// Race Distance Calculation
fn race_distance(time_button_held: u64, max_race_time: u64) -> u64 {
    return time_button_held * (max_race_time - time_button_held);
}

/// Transform the race numbers into one long one
fn rm_kerning(race_details: &Vec<(u64, u64)>) -> (u64, u64) {
    let mut time = String::new();
    let mut distance = String::new();

    for (x, y) in race_details.iter() {
        time.push_str(&x.to_string());
        distance.push_str(&y.to_string());
    }

    return (
        time.parse::<u64>().unwrap(),
        distance.parse::<u64>().unwrap(),
    );
}

fn main() {
    let race_details = read_races("./data/input.txt");
    let long_race = rm_kerning(&race_details);

    println!(
        "The answer to part 1 is: {}",
        race_details.iter().map(|x| ways_to_win(x)).product::<u64>()
    );

    println!("The answer to part 2 is: {}", ways_to_win(&long_race));
}
