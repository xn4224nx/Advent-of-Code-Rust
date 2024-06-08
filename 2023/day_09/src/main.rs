/*
 * --- Day 9: Mirage Maintenance ---
 *
 * The OASIS produces a report of many values and how they are changing
 * over time (your puzzle input). Each line in the report contains the
 * history of a single value.
 *
 * To best protect the oasis, your environmental report should include a
 * prediction of the next value in each history. To do this, start by
 * making a new sequence from the difference at each step of your
 * history. If that sequence is not all zeroes, repeat this process,
 * using the sequence you just generated as the input sequence. Once all
 * of the values in your latest sequence are zeroes, you can extrapolate
 * what the next value of the original history should be.
 *
 * Part 1 - Analyze your OASIS report and extrapolate the next value for
 *          each history. What is the sum of these extrapolated values?
 */

use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Parse sequence data into a machine readable format
pub fn read_seq_data(filepath: &str) -> Vec<Vec<i32>> {
    let file = File::open(filepath).expect("Unable to read file!");
    let reader = BufReader::new(file);

    let seq_re = Regex::new(r"\-?[\d]+").unwrap();

    let mut all_seq: Vec<Vec<i32>> = Vec::new();

    for raw_line in reader.lines() {
        /* Check the line can be read otherwise skip it. */
        let Ok(c_line) = raw_line else {
            continue;
        };

        /* Create a vector of numbers out of the line */
        let seq: Vec<i32> = seq_re
            .find_iter(&c_line)
            .map(|x| x.as_str().parse::<i32>().unwrap())
            .collect();

        all_seq.push(seq);
    }

    return all_seq;
}

/// Find the depths of a sequence differences
pub fn plumb_seq_depths(seq: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut seq_depths: Vec<Vec<i32>> = Vec::new();
    let mut depth_idx = 0;

    /* The top level of the depths is the original sequence. */
    seq_depths.push((*seq.clone()).to_vec());

    /* Plumb the depths of the sequence. */
    loop {
        /* If all values are the same there is a common difference */
        if seq_depths[depth_idx]
            .iter()
            .all(|&x| x == seq_depths[depth_idx][0])
        {
            break;
        }

        /* Calculate the common difference between common differences.*/
        let diff = seq_depths[depth_idx]
            .windows(2)
            .map(|x| x[1] - x[0])
            .collect();

        seq_depths.push(diff);
        depth_idx += 1;
    }

    return seq_depths;
}

/// Find the next value in a polynomial sequence
pub fn next_seq_val(seq: &Vec<i32>) -> i32 {
    /* Plumb the depths of the sequence. */
    let seq_depths = plumb_seq_depths(&seq);

    let mut next_vals: Vec<i32> = Vec::new();

    /* Work up from the depths determining the next value. */
    for (idx, seq_dif) in seq_depths.iter().rev().enumerate() {
        if idx == 0 {
            next_vals.push(seq_dif[idx]);
        } else {
            let prev_new_diff = next_vals.last().unwrap();
            let curr_last_val = seq_dif.last().unwrap();
            next_vals.push(curr_last_val + prev_new_diff);
        }
    }

    return *next_vals.last().unwrap();
}

fn main() {
    let all_seq = read_seq_data("./data/input.txt");

    println!(
        "The answer to part 1 = {}",
        all_seq.iter().map(|x| next_seq_val(x)).sum::<i32>()
    );
}
