/*
 * --- Day 1: No Time for a Taxicab ---
 *
 * Santa's sleigh uses a very high-precision clock to guide its movements, and
 * the clock's oscillator is regulated by stars. Unfortunately, the stars have
 * been stolen... by the Easter Bunny. To save Christmas, Santa needs you to
 * retrieve all fifty stars by December 25th.
 *
 * Collect stars by solving puzzles. Two puzzles will be made available on each
 * day in the Advent calendar; the second puzzle is unlocked when you complete
 * the first. Each puzzle grants one star. Good luck!
 *
 * You're airdropped near Easter Bunny Headquarters in a city somewhere. "Near",
 * unfortunately, is as close as you can get - the instructions on the Easter
 * Bunny Recruiting Document the Elves intercepted start here, and nobody had
 * time to work them out further.
 *
 * The Document indicates that you should start at the given coordinates (where
 * you just landed) and face North. Then, follow the provided sequence: either
 * turn left (L) or right (R) 90 degrees, then walk forward the given number of
 * blocks, ending at a new intersection.
 *
 * There's no time to follow such ridiculous instructions on foot, though, so
 * you take a moment and work out the destination. Given that you can only walk
 * on the street grid of the city, how far is the shortest path to the
 * destination?
 *
 * PART 1:  How many blocks away is Easter Bunny HQ?
 */

use num_complex::Complex;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Debug)]
pub enum Direc {
    L(u8),
    R(u8),
}

/// Open a directions file and parse it into a structured format
pub fn read_directions(file_path: &str) -> Vec<Direc> {
    let mut buffer = String::new();
    let mut results = Vec::new();
    let dir_re_pat = Regex::new(r"(R|L)(\d+)").unwrap();

    /* Open the file. */
    let file = File::open(file_path).unwrap();
    let mut file_ptr = BufReader::new(file);

    /* Parse the file line by line. */
    while file_ptr.read_line(&mut buffer).unwrap() > 0 {
        for capture in dir_re_pat.captures_iter(&buffer) {
            let mag = capture[2].parse::<u8>().unwrap();

            results.push(match &capture[1] {
                "L" => Direc::L(mag),
                "R" => Direc::R(mag),
                _ => panic!("{} is not a recognised direction", &capture[0]),
            })
        }
        buffer.clear();
    }
    return results;
}

/// Follow directions and work out the grid distance from the origin
pub fn find_shortest_path(directions: &Vec<Direc>) -> u32 {
    let mut curr_direc = Complex::<i32>::new(0, 1);
    let mut curr_point = Complex::<i32>::new(0, 0);

    /* Process the directions in order. */
    for direc_t in directions.iter() {
        /* Turn the curr_direc and extract the magnitude of the move. */
        match direc_t {
            Direc::L(mag) => {
                curr_direc *= Complex::<i32>::new(0, -1);
                magniut = *mag;
            }
            Direc::R(mag) => {
                curr_direc *= Complex::<i32>::new(0, 1);
                magniut = *mag;
            }
        }
        /* Move the point along its currect direction. */
        curr_point += curr_direc.scale(magniut as i32);
    }
    /* Work out the grid distance from the origin to the current point. */
    return (curr_point.im.abs() + curr_point.re.abs()) as u32;
}

fn main() {
    let directs = read_directions("./data/input.txt");
    println!("Part 1 = {}", find_shortest_path(&directs));
}
