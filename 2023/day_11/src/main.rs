/*
 * --- Day 11: Cosmic Expansion ---
 *
 * The researcher has collected a bunch of data and compiled the data
 * into a single giant image (your puzzle input). The image includes
 * empty space (.) and galaxies (#).
 *
 * The researcher is trying to figure out the sum of the lengths of the
 * shortest path between every pair of galaxies. However, there's a
 * catch: the universe expanded in the time it took the light from those
 * galaxies to reach the observatory.
 *
 * Due to something involving gravitational effects, only some space
 * expands. In fact, the result is that any rows or columns that contain
 * no galaxies should all actually be twice as big.
 *
 * Only count each pair once; order within the pair doesn't matter. For
 * each pair, find any shortest path between the two galaxies using only
 * steps that move up, down, left, or right exactly one . or # at a
 * time. (The shortest path between two galaxies is allowed to pass
 * through another galaxy.)
 *
 * Part 1 - Expand the universe, then find the length of the shortest
 *          path between every pair of galaxies. What is the sum of
 *          these lengths?
 */

use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Read a file and return a vector of the galaxy coordinate positions
pub fn read_galaxy_img(filepath: &str) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();

    /* Read the file. */
    let file = File::open(filepath).expect("File could not be read!");
    let reader = BufReader::new(file);

    /* Iterate over the file line by line. */
    for (rw_idx, raw_line) in reader.lines().enumerate() {
        /* Iterate over the chars in the line. */
        for (cl_idx, lchar) in raw_line.unwrap().chars().enumerate() {
            if lchar == '#' {
                galaxies.push((cl_idx, rw_idx));
            }
        }
    }
    return galaxies;
}

/// Find columns and rows that contain no galaxies
pub fn find_empty_space(galaxies: &Vec<(usize, usize)>) -> (Vec<usize>, Vec<usize>) {
    let mut seen_x: HashSet<usize> = HashSet::new();
    let mut seen_y: HashSet<usize> = HashSet::new();
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    /* Find all the rows and column that have a galaxy in them, also find
    maximum extent of the universe.  */
    for (x, y) in galaxies.iter() {
        seen_x.insert(*x);
        seen_y.insert(*y);

        if x > &max_x {
            max_x = *x;
        }

        if y > &max_y {
            max_y = *y;
        }
    }

    /* Generate lists of the columns & rows that have no galaxies. */
    let empty_cols: Vec<usize> = (0..max_x).filter(|x| !seen_x.contains(x)).collect();
    let empty_rows: Vec<usize> = (0..max_y).filter(|y| !seen_y.contains(y)).collect();

    return (empty_cols, empty_rows);
}

fn main() {
    let mut galaxy_positions = read_galaxy_img("./data/example_01.txt");
    let empty_univ = find_empty_space(&galaxy_positions);
}
