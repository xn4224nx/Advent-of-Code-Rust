/*
 * --- Day 14: Reindeer Olympics ---
 *
 * This year is the Reindeer Olympics! Reindeer can fly at high speeds, but must
 * rest occasionally to recover their energy. Santa would like to know which of
 * his reindeer is fastest, and so he has them race.
 *
 * Reindeer can only either be flying (always at their top speed) or resting (not
 * moving at all), and always spend whole seconds in either state.
 *
 * PART 1:  Given the descriptions of each reindeer (in your puzzle input),
 *          after exactly 2503 seconds, what distance has the winning reindeer
 *          traveled?
 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub struct Reindeer {
    pub speed: u32,
    pub run_time: u32,
    pub rest_time: u32,
}

/// Read a Reindeer data file and return a vector of the key characteristics.
pub fn read_reindeer_data(data_file: &str) -> Vec<Reindeer> {
    let mut data = Vec::new();

    /* Open the file. */
    let file = File::open(data_file).unwrap();
    let mut fp = BufReader::new(file);

    /* Extract the three +VE integers in the line. */
    let re_uint = Regex::new(r"\d+").unwrap();

    /* Read the file line by line into a buffer. */
    let mut buffer = String::new();
    while fp.read_line(&mut buffer).unwrap() > 0 {
        let nums: Vec<u32> = re_uint
            .find_iter(&buffer)
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect();

        /* Insert the extracted integers into a structure and then vector. */
        data.push(Reindeer {
            speed: nums[0],
            run_time: nums[1],
            rest_time: nums[2],
        });

        buffer.clear();
    }

    return data;
}

/// Determine the distance traveled at each interval for a racing
/// Reindeer and return a vector of those distances.
pub fn dist_travelled(rein: &Reindeer, race_time: u32) -> Vec<u32> {
    let mut dists: Vec<u32> = Vec::new();

    for sec_idx in 0..=race_time {
        let comp_cycles = sec_idx / (rein.run_time + rein.rest_time);
        let cycle_dist = comp_cycles * rein.speed * rein.run_time;
        let rem_time = sec_idx - comp_cycles * (rein.run_time + rein.rest_time);

        /* Determine if the Reindeer is currently resting. */
        let rem_dist = if rem_time > rein.run_time {
            rein.run_time * rein.speed
        } else {
            rem_time * rein.speed
        };

        dists.push(cycle_dist + rem_dist)
    }
    return dists;
}

/// Work out which of the Reindeer will win the race by travelling the
/// furthest distance in the given time. Return the winning distance.
pub fn winning_dist(competitors: &Vec<Reindeer>, race_time: u32) -> u32 {
    let mut max_dist = 0;

    for rein in competitors.iter() {
        let com_dist = dist_travelled(&rein, race_time)[race_time as usize];

        if com_dist > max_dist {
            max_dist = com_dist;
        }
    }
    return max_dist;
}

fn main() {}
