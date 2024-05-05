/*
 * --- Day 3: Gear Ratios ---
 *
 * The engineer explains that an engine part seems to be missing from the
 * engine, but nobody can figure out which one. If you can add up all the part
 * numbers in the engine schematic, it should be easy to work out which part is
 * missing.
 *
 * The engine schematic (your puzzle input) consists of a visual representation
 * of the engine. There are lots of numbers and symbols you don't really
 * understand, but apparently any number adjacent to a symbol, even diagonally,
 * is a "part number" and should be included in your sum. (Periods (.) do not
 * count as a symbol.)
 *
 * Part 1: What is the sum of all of the part numbers in the engine schematic?
 */

use std::fs;

/// Load the data into a vector of strings.
fn file_to_str_vec(file_pth: &str) -> Vec<String> {
    return fs::read_to_string(file_pth)
        .expect("Unable to read file")
        .split("\n")
        .map(|line| line.to_string())
        .collect();
}

fn main() {
    let data = file_to_grid("./data/example_0.txt");
    println!("{:?}", data);
}
