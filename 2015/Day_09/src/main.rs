/*
 * --- Day 9: All in a Single Night ---
 *
 * Every year, Santa manages to deliver all of his presents in a single night.
 *
 * This year, however, he has some new locations to visit; his elves have
 * provided him the distances between every pair of locations. He can start and
 * end at any two (different) locations he wants, but he must visit each location
 * exactly once. What is the shortest distance he can travel to achieve this?
 *
 * PART 1:  What is the distance of the shortest route?
 *
 * The next year, just to show off, Santa decides to take the route with the
 * longest distance instead.
 *
 * He can still start and end at any two (different) locations he wants, and he
 * still must visit each location exactly once.
 *
 * PART 2:  What is the distance of the longest route?
 */

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Read the data file and parse into a structured format.
pub fn read_dist_data(file_path: &str) -> HashMap<String, HashMap<String, usize>> {
    let file = File::open(file_path).unwrap();
    let mut fp = BufReader::new(file);
    let mut buffer = String::new();
    let mut data = HashMap::new();
    let re = Regex::new(r"([A-Za-z]+) to ([A-Za-z]+) = (\d+)").unwrap();

    while fp.read_line(&mut buffer).unwrap() > 0 {
        let caps = re.captures(&buffer).unwrap();
        let start = caps.get(1).unwrap().as_str().to_string();
        let end = caps.get(2).unwrap().as_str().to_string();
        let dist = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

        /* Ensure the location exists in the Map */
        if !data.contains_key(&start) {
            data.insert(start.clone(), HashMap::new());
        };

        if !data.contains_key(&end) {
            data.insert(end.clone(), HashMap::new());
        };

        /* Populate the sub-maps */
        if !data.get(&start).unwrap().contains_key(&end) {
            data.get_mut(&start).unwrap().insert(end.clone(), dist);
        }
        if !data.get(&end).unwrap().contains_key(&start) {
            data.get_mut(&end).unwrap().insert(start.clone(), dist);
        }

        buffer.clear();
    }

    return data;
}

/// Iterate over every possible route in the map and find the shortest and
/// longest route lengths.
pub fn find_minmax_path(data: &HashMap<String, HashMap<String, usize>>) -> (usize, usize) {
    let all_locs: Vec<&String> = data.keys().collect();
    let mut min_route = usize::MAX;
    let mut max_route = 0;

    /* Iterate over every possible ordering of the locations. */
    'outer: for perm in all_locs.iter().permutations(all_locs.len()) {
        let mut dist = 0;

        /* Iterate over each location in the possible path. */
        for idx in 1..perm.len() {
            /* Check that the current and previous location are actually linked */
            if idx < perm.len() - 1
                && !data
                    .get(&perm[idx] as &String)
                    .unwrap()
                    .contains_key(&perm[idx - 1] as &String)
            {
                continue 'outer;
            }

            /* Extract the value from the nested dictionary. */
            dist += data
                .get(&perm[idx] as &String)
                .unwrap()
                .get(&perm[idx - 1] as &String)
                .unwrap()
        }

        /* Check for a new min or mix */
        if dist > max_route {
            max_route = dist;
        }
        if dist < min_route {
            min_route = dist;
        }
    }

    return (min_route, max_route);
}

fn main() {
    let route_data = read_dist_data("./data/input.txt");
    let (r_min, r_max) = find_minmax_path(&route_data);

    println!("Part 1 = {}\nPart 2 = {}", r_min, r_max);
}
