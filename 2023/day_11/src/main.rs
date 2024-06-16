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
 *
 * The galaxies are much older (and thus much farther apart) than the researcher
 * initially estimated.
 *
 * Now, instead of the expansion you did before, make each empty row or column
 * one million times larger. That is, each empty row should be replaced with
 * 1000000 empty rows, and each empty column should be replaced with 1000000
 * empty columns.
 *
 * Part 2 - Starting with the same initial image, expand the universe according
 *          to these new rules, then find the length of the shortest path
 *          between every pair of galaxies. What is the sum of these lengths?
 */

use itertools::Itertools;
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

/// Expand the empty space in a universe image
pub fn expand_empty_space(
    galaxies: &mut Vec<(usize, usize)>,
    empty_space: &(Vec<usize>, Vec<usize>),
    expansion_factor: usize,
) {
    /* For each galaxy increase its coords based on empty space being before it. */
    for galx_idx in 0..galaxies.len() {
        /* Expand the columns in reverse order. */
        for empty_col in empty_space.0.iter().rev() {
            if galaxies[galx_idx].0 > *empty_col {
                galaxies[galx_idx].0 += expansion_factor - 1;
            }
        }

        /* Expand the rows in reverse order. */
        for empty_row in empty_space.1.iter().rev() {
            if galaxies[galx_idx].1 > *empty_row {
                galaxies[galx_idx].1 += expansion_factor - 1;
            }
        }
    }
}

/// Calculate the manhattan distances between two points
pub fn manh_dist(pnt_0: &(usize, usize), pnt_1: &(usize, usize)) -> usize {
    let mut dist = 0;

    /* x coordinates */
    if pnt_0.0 > pnt_1.0 {
        dist += pnt_0.0 - pnt_1.0;
    } else {
        dist += pnt_1.0 - pnt_0.0;
    }

    /* y coordinates */
    if pnt_0.1 > pnt_1.1 {
        dist += pnt_0.1 - pnt_1.1;
    } else {
        dist += pnt_1.1 - pnt_0.1;
    }

    return dist;
}

/// Calculate the sum of the manhattan distances between all the galaxy combinations
pub fn sum_galaxy_distances(galaxies: &Vec<(usize, usize)>) -> usize {
    return galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| manh_dist(a, b))
        .sum();
}

fn main() {
    let mut p1_galaxy_positions = read_galaxy_img("./data/input.txt");
    let mut p2_galaxy_positions = p1_galaxy_positions.clone();
    
    let empty_space = find_empty_space(&p1_galaxy_positions);
    
    expand_empty_space(&mut p1_galaxy_positions, &empty_space, 2);
    println!(
        "Part 1 answer = {}",
        sum_galaxy_distances(&p1_galaxy_positions)
    );
    
    expand_empty_space(&mut p2_galaxy_positions, &empty_space, 1000000);
    println!(
        "Part 2 answer = {}",
        sum_galaxy_distances(&p2_galaxy_positions)
    );
}
