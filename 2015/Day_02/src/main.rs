/*
 * --- Day 2: I Was Told There Would Be No Math ---
 *
 * The elves are running low on wrapping paper, and so they
 * need to submit an order for more. They have a list of the
 * dimensions (length l, width w, and height h) of each
 * present, and only want to order exactly as much as they
 * need.
 *
 * Fortunately, every present is a box (a perfect right
 * rectangular prism), which makes calculating the required
 * wrapping paper for each gift a little easier: find the
 * surface area of the box, which is 2*l*w + 2*w*h + 2*h*l. The
 * elves also need a little extra paper for each present: the
 * area of the smallest side.
 *
 * All numbers in the elves' list are in feet.
 *
 * PART 1:  How many total square feet of wrapping paper should
 *          they order?
 */

use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Read the instruction file and parse into a vector of tuples
/// comprised of three integers.
pub fn read_instructions(inst_filepath: &str) -> Vec<(usize, usize, usize)> {
    let mut parsed_inst = Vec::new();
    let re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();

    /* Open the instruction file. */
    let file_ptr = File::open(inst_filepath).unwrap();
    let reader = BufReader::new(file_ptr);

    for raw_line in reader.lines() {
        /* Parse the line or skip it */
        let Ok(line) = raw_line else {
            continue;
        };

        match re.captures(&line) {
            Some(caps) => {
                parsed_inst.push((
                    caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                ));
            }
            None => {
                continue;
            }
        };
    }
    return parsed_inst;
}

fn main() {}
