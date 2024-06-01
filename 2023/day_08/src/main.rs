/*
* --- Day 8: Haunted Wasteland ---
*
* One of the camel's pouches is labeled "maps" - sure enough, it's full of
* documents (your puzzle input) about how to navigate the desert. At least,
* you're pretty sure that's what they are; one of the documents contains a list
* of left/right instructions, and the rest of the documents seem to describe
* some kind of network of labeled nodes.
*
* It seems like you're meant to use the left/right instructions to navigate the
* network. Perhaps if you have the camel follow the same instructions, you can
* escape the haunted wasteland!
*
* After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel
* like AAA is where you are now, and you have to follow the left/right
* instructions until you reach ZZZ.
*
* Starting with AAA, you need to look up the next element based on the next
* left/right instruction in your input. Of course, you might not find ZZZ right
* away. If you run out of left/right instructions, repeat the whole sequence of
* instructions as necessary.
*
* Part 1 - Starting at AAA, follow the left/right instructions. How many steps
*          are required to reach ZZZ?
*
* After examining the maps a bit longer, your attention is drawn to a curious
* fact: the number of nodes with names ending in A is equal to the number
* ending in Z! If you were a ghost, you'd probably just start at every node
* that ends with A and follow all of the paths at the same time until they
* all simultaneously end up at nodes that end with Z.
*
* As you follow each left/right instruction, use that instruction to
* simultaneously navigate away from both nodes you're currently on. Repeat
* this process until all of the nodes you're currently on end with Z. (If
* only some of the nodes you're on end with Z, they act like any other node
* and you continue as normal.)
*
* Part 2 - Simultaneously start on every node that ends with A. How many
*          steps does it take before you're only on nodes that end with Z?
*/

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

/// Read the map file and parse it to a vector
pub fn read_map_data(file_path: &str) -> (String, Vec<(String, String, String)>) {
    let mut map_data = Vec::new();
    let mut turns = String::new();
    let re_map = Regex::new("[A-Z]{3}").unwrap();

    /* Read the entire file into a string. */
    let raw_file = fs::read_to_string(file_path).expect("Could not open file!");

    for (idx, raw_line) in raw_file.lines().enumerate() {
        /* Capture the turns at the start of the file. */
        if idx == 0 {
            turns = raw_line.to_string();
            continue;
        }

        /* Parse the node identities and convert to a three string tuple.*/
        let raw_nodes = re_map
            .find_iter(raw_line)
            .filter_map(|x| Some(x.as_str().to_string()))
            .collect_tuple();

        /* If the parse was sucessful retain it otherwise continue the loop. */
        let Some(nodes) = raw_nodes else { continue };
        map_data.push(nodes);
    }

    return (turns, map_data);
}

/// Convert the map to a HashMap version
pub fn parse_map(raw_map: Vec<(String, String, String)>) -> HashMap<String, (String, String)> {
    let mut new_map = HashMap::new();

    for line in raw_map {
        new_map.entry(line.0).or_insert((line.1, line.2));
    }

    return new_map;
}

/// Count the steps required to reach the end of the map
pub fn count_map_steps(turns: String, desert_map: HashMap<String, (String, String)>) -> u32 {
    /* This is the node that the map starts */
    let start_node = String::from("AAA");

    let mut curr_node = &start_node;
    let final_node = &String::from("ZZZ");
    let mut path_cnt = 0;

    for path in turns.chars().cycle() {
        /* Get the two possible paths */
        let paths: &(String, String) = desert_map.get(curr_node).unwrap();

        if path == 'L' {
            curr_node = &paths.0
        } else {
            curr_node = &paths.1
        }

        path_cnt += 1;

        /* If the destination has been reached break the loop. */
        if curr_node == final_node {
            break;
        }
    }

    return path_cnt;
}

fn main() {
    let (turns, raw_map) = read_map_data("./data/input.txt");
    let desert_map = parse_map(raw_map);

    println!("Answer to part 1 = {}", count_map_steps(turns, desert_map));
}
