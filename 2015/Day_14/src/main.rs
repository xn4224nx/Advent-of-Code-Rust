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

#[derive(Debug, PartialEq)]
pub struct Reindeer {
    pub speed: u32,
    pub run_time: u32,
    pub rest_time: u32,
}

/// Read a Reindeer data file and return a vector of the key characteristics.
pub fn read_reindeer_data(data_file: &str) -> Vec<Reindeer> {
    Vec::new()
}

/// Determine the distance traveled at each interval for a racing
/// Reindeer and return a vector of those distances.
pub fn dist_travelled(contestant: &Reindeer) -> Vec<u32> {
    Vec::new()
}

/// Work out which of the Reindeer will win the race by travelling the
/// furthest distance in the given time. Return the winning distance.
pub fn winning_dist(competitors: &Vec<Reindeer>, race_time: u32) -> u32 {
    0
}

fn main() {}
