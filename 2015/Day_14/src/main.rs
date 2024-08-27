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
 *
 * Seeing how reindeer move in bursts, Santa decides he's not pleased with the
 * old scoring system.
 *
 * Instead, at the end of each second, he awards one point to the reindeer
 * currently in the lead. (If there are multiple reindeer tied for the lead,
 * they each get one point.) He keeps the traditional 2503 second time limit, of
 * course, as doing otherwise would be entirely ridiculous.
 *
 * PART 2:  Again given the descriptions of each reindeer (in your puzzle
 *          input), after exactly 2503 seconds, how many points does the winning
 *          reindeer have?
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

/// Determine the distance traveled for an interval for a racing
/// Reindeer and return a vector of those distances.
pub fn dist_travelled(rein: &Reindeer, curr_race_time: u32) -> u32 {
    let comp_cycles = curr_race_time / (rein.run_time + rein.rest_time);
    let cycle_dist = comp_cycles * rein.speed * rein.run_time;
    let rem_time = curr_race_time - comp_cycles * (rein.run_time + rein.rest_time);

    /* Determine if the Reindeer is currently resting. */
    let rem_dist = if rem_time > rein.run_time {
        rein.run_time * rein.speed
    } else {
        rem_time * rein.speed
    };

    return cycle_dist + rem_dist;
}

/// Work out which of the Reindeer will win the race by travelling the
/// furthest distance in the given time. Return the winning distance.
pub fn winning_dist(competitors: &Vec<Reindeer>, race_time: u32) -> u32 {
    let mut max_dist = 0;

    for rein in competitors.iter() {
        let com_dist = dist_travelled(&rein, race_time);

        if com_dist > max_dist {
            max_dist = com_dist;
        }
    }
    return max_dist;
}

/// Work out which of the Reindeer will win the race by accruing the highest
/// score. Return the winning score.
pub fn winning_score(competitors: &Vec<Reindeer>, race_time: u32) -> u32 {
    let mut scores = vec![0; competitors.len()];

    for sec_idx in 1..=race_time {
        let curr_dists: Vec<u32> = competitors
            .iter()
            .map(|x| dist_travelled(&x, sec_idx))
            .collect();
        let curr_max: u32 = *curr_dists.iter().max().unwrap();

        /* If a Reindeer is at the maximum distance it gets a point. */
        for rein_idx in 0..curr_dists.len() {
            if curr_dists[rein_idx] == curr_max {
                scores[rein_idx] += 1;
            }
        }
    }
    return *scores.iter().max().unwrap();
}

fn main() {
    let data = read_reindeer_data("./data/input.txt");
    println!("Part 1 = {}", winning_dist(&data, 2503));
    println!("Part 2 = {}", winning_score(&data, 2503));
}
