/*
 * --- Day 24: Air Duct Spelunking ---
 *
 * You've finally met your match; the doors that provide access to the roof are
 * locked tight, and all of the controls and related electronics are
 * inaccessible. You simply can't reach them.
 *
 * The robot that cleans the air ducts, however, can.
 *
 * It's not a very fast little robot, but you reconfigure it to be able to
 * interface with some of the exposed wires that have been routed through the
 * HVAC system. If you can direct it to each of those locations, you should be
 * able to bypass the security controls.
 *
 * You extract the duct layout for this area from some blueprints you acquired
 * and create a map with the relevant locations marked (your puzzle input). 0 is
 * your current location, from which the cleaning robot embarks; the other
 * numbers are (in no particular order) the locations the robot needs to visit
 * at least once each. Walls are marked as #, and open passages are marked as ..
 * Numbers behave like open passages.
 *
 * Since the robot isn't very fast, you need to find it the shortest route. This
 * path is the fewest steps required to start at 0 and then visit every other
 * location at least once.
 *
 * PART 1:  Given your actual map, and starting from location 0, what is the
 *          fewest number of steps required to visit every non-0 number marked
 *          on the map at least once?
 */

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct AirDucts {
    pub clear_tiles: HashSet<(usize, usize)>,
    pub numbr_tiles: Vec<(usize, usize)>,
}

impl AirDucts {
    pub fn new(data_file: &str) -> Self {
        let mut clear_tiles = HashSet::new();
        let mut tmp_number_tiles = HashMap::new();
        let mut buffer = String::new();

        /* Open the file. */
        let file = File::open(data_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Read the file line by line */
        let mut line_idx = 0;
        while fp.read_line(&mut buffer).unwrap() > 0 {
            /* Iterate over the characters in the line. */
            for (char_idx, ad_char) in buffer.chars().enumerate() {
                match ad_char {
                    '.' => {
                        clear_tiles.insert((char_idx, line_idx));
                    }
                    '0'..='9' => {
                        clear_tiles.insert((char_idx, line_idx));
                        tmp_number_tiles.insert(ad_char as usize, (char_idx, line_idx));
                    }
                    _ => {}
                }
            }
            buffer.clear();
            line_idx += 1;
        }

        /* Construct the vector of number coordinates */
        let mut num_order = tmp_number_tiles.keys().map(|x| *x).collect::<Vec<usize>>();
        num_order.sort();
        let numbr_tiles = num_order.iter().map(|x| tmp_number_tiles[x]).collect();

        return AirDucts {
            clear_tiles,
            numbr_tiles,
        };
    }

    /// From the current coordinate find all the next possible steps
    pub fn next_steps(&self, coord: (usize, usize)) -> Vec<(usize, usize)> {
        let mut nxt_steps = Vec::new();

        /* Is the tile above this one clear? */
        let up_c = (coord.0, coord.1 - 1);
        if self.clear_tiles.contains(&up_c) {
            nxt_steps.push(up_c)
        };

        /* Is the tile below this one clear? */
        let dw_c = (coord.0, coord.1 + 1);
        if self.clear_tiles.contains(&dw_c) {
            nxt_steps.push(dw_c)
        };

        /* Is the tile to the left of this one clear? */
        let lf_c = (coord.0 - 1, coord.1);
        if self.clear_tiles.contains(&lf_c) {
            nxt_steps.push(lf_c)
        };

        /* Is the tile to the righ of this one clear? */
        let rt_c = (coord.0 + 1, coord.1);
        if self.clear_tiles.contains(&rt_c) {
            nxt_steps.push(rt_c)
        };

        return nxt_steps;
    }

    /// Find the smallest distance between two numbers in the air ducts
    pub fn min_numbr_dist(&self, num_1: usize, num_2: usize) -> usize {
        let start_coord = self.numbr_tiles[num_1];
        let end_coord = self.numbr_tiles[num_2];

        let mut curr_points = HashSet::from([start_coord]);
        let mut seen_points = HashSet::new();
        let mut move_idx = 0;

        loop {
            move_idx += 1;
            let mut next_points = HashSet::new();

            /* Determine all possible next moves from all current points. */
            for c_pnt in curr_points {
                seen_points.insert(c_pnt);

                for n_pnt in self.next_steps(c_pnt) {
                    /* Check for a solution */
                    if n_pnt == end_coord {
                        return move_idx;
                    }

                    /* Don't re-visit previous steps. */
                    if !seen_points.contains(&n_pnt) {
                        next_points.insert(n_pnt);
                    }
                }
            }

            /* Prepare for the next loop iteration. */
            if next_points.len() > 0 {
                curr_points = next_points;
            } else {
                panic!("No next steps possible!");
            }
        }
    }

    /// Calculate the minmum number of steps to vist all numbers starting at
    /// zero.
    pub fn min_traversal(&self) -> usize {
        let mut path_combs = HashMap::new();

        /* Find the smallest path between each pair of numbers. */
        for num_comb in (0..self.numbr_tiles.len()).combinations(2) {
            let min_path = self.min_numbr_dist(num_comb[0], num_comb[1]);
            path_combs.insert((num_comb[0], num_comb[1]), min_path);
            path_combs.insert((num_comb[1], num_comb[0]), min_path);
        }

        /* For each route through the numbers calculate the total length. */
        let mut all_path_lens = HashSet::new();
        for path_comb in (1..self.numbr_tiles.len()).permutations(self.numbr_tiles.len() - 1) {
            let fpath_comb = [vec![0], path_comb].concat();

            /* Calculate the total length of this route */
            all_path_lens.insert(
                (1..fpath_comb.len())
                    .map(|x| path_combs[&(fpath_comb[x - 1], fpath_comb[x])])
                    .sum::<usize>(),
            );
        }

        /* Return the smallest route length found. */
        return *all_path_lens.iter().min().unwrap();
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        AirDucts::new("./data/input.txt").min_traversal()
    );
}
