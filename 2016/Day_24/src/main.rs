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

use std::collections::HashSet;

pub struct AirDucts {
    pub clear_tiles: HashSet<(usize, usize)>,
    pub numbr_tiles: Vec<(usize, usize)>,
}

impl AirDucts {
    pub fn new(data_file: &str) -> Self {
        AirDucts {
            clear_tiles: HashSet::new(),
            numbr_tiles: Vec::new(),
        }
    }

    /// From the current coordinate find all the next possible steps
    pub fn next_steps(&self, coord: (usize, usize)) -> Vec<(usize, usize)> {
        Vec::new()
    }

    /// Find the smallest distance between two numbers in the air ducts
    pub fn min_numbr_dist(&self, num_1: usize, num_2: usize) -> usize {
        0
    }

    /// Calculate the minmum number of steps to vist all numbers starting at
    /// zero.
    pub fn min_traversal(&self) -> usize {
        0
    }
}

fn main() {}
