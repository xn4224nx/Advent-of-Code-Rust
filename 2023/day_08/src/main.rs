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

fn main() {
    let (turns, raw_map) = read_map_data("./data/example_01.txt");
    let desert_map = parse_map(raw_map);

    println!("{:?}", desert_map);
}
