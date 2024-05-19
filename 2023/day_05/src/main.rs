/*
* --- Day 5: If You Give A Seed A Fertilizer ---
*
* The almanac (your puzzle input) lists all of the seeds that need to be
* planted. It also lists what type of soil to use with each kind of seed, what
* type of fertilizer to use with each kind of soil, what type of water to use
* with each kind of fertilizer, and so on. Every type of seed, soil, fertilizer
* and so on is identified with a number, but numbers are reused by each category
* - that is, soil 123 and fertilizer 123 aren't necessarily related to each
* other.
*
* The almanac starts by listing which seeds need to be planted. The rest of the
* almanac contains a list of maps which describe how to convert numbers from a
* source category into numbers in a destination category. That is, the section
* that starts with seed-to-soil map: describes how to convert a seed number (the
* source) to a soil number (the destination).
*
* Rather than list every source number and its corresponding destination number
* one by one, the maps describe entire ranges of numbers that can be converted.
* Each line within a map contains three numbers: the destination range start,
* the source range start, and the range length.
*
* Any source numbers that aren't mapped correspond to the same destination
* number. So, seed number 10 corresponds to soil number 10.
*
* Part 1 - What is the lowest location number that corresponds to any of the
*          initial seed numbers?
*
* The values on the initial seeds: line come in pairs. Within each pair, the
* first value is the start of the range and the second value is the length of
* the range.
*
* Part 2 - Consider all of the initial seed numbers listed in the ranges on the
*          first line of the almanac. What is the lowest location number that
*          corresponds to any of the initial seed numbers?
*/

use itertools::Itertools;
use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Parse an almanac file into a structured format
fn read_almanac(file_path: &str) -> (Vec<u64>, Vec<Vec<(u64, u64, u64)>>) {
    let mut seeds: Vec<u64> = Vec::new();
    let mut all_maps: Vec<Vec<(u64, u64, u64)>> = Vec::new();

    let file = File::open(file_path).expect("File could not be opened!");
    let reader = BufReader::new(file);

    for (idx, raw_line) in reader.lines().enumerate() {
        /* Extract the line or stop reading the file. */
        let Ok(line) = raw_line else { break };

        /* Read the seed numbers */
        if idx == 0 {
            let line_parts: Vec<&str> = line.split(":").collect();
            seeds = line_parts[1]
                .split_whitespace()
                .map(|x| x.trim().parse::<u64>().unwrap())
                .collect();

        /* Detect the map changing. */
        } else if line.contains("map:") {
            all_maps.push(Vec::new())

        /* Parse the map numbers */
        } else if line.contains(" ") {
            let map_line: (u64, u64, u64) = line
                .split_whitespace()
                .map(|x| x.trim().parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();

            let final_idx = all_maps.len() - 1;

            all_maps[final_idx].push(map_line)
        }
    }
    return (seeds, all_maps);
}

/// Follow the seed through all the maps
fn all_map_follow(seeds: &mut Vec<u64>, all_seed_maps: &Vec<Vec<(u64, u64, u64)>>) {
    /* Follow seed values through each multi-map */
    for multi_map in all_seed_maps.iter() {
        /* Calculate each seed value individually. */
        for idx in 0..seeds.len() {
            let seed = seeds[idx];

            /* Find the first map that changes the seed */
            for seed_map in multi_map.iter() {
                if seed >= seed_map.1 && seed <= seed_map.1 + seed_map.2 {
                    /* Set the new seed value and stop checking maps */
                    seeds[idx] = seed_map.0 + seed - seed_map.1;
                    break;
                }
            }
        }
    }
}

/// Follow range of seeds through maps 
fn range_map_follow(input_seeds: &mut Vec<u64>, all_seed_maps: &Vec<Vec<(u64, u64, u64)>>) {
    
    let mut lowest: Vec<u64> = Vec::new();
    
    /* Calculate each seed range individually. */
    for idx in (0..input_seeds.len()).step_by(2) {
        
        /* Generate the full seed range. */
        let mut seeds = expand_seed_range((input_seeds[idx], input_seeds[idx+1]));
        
        /* Follow this group of seeds through the maps */
        all_map_follow(&mut seeds, &all_seed_maps);
        
        /* Save the smallest result. */
        lowest.push(*seeds.iter().min().unwrap())
    }
    
    /* Set all the lowest results in the seed vector. */
    input_seeds.clear();
    input_seeds.append(&mut lowest);
}

/// Expand range of seeds to seed values
fn expand_seed_range(seed_range: (u64, u64)) -> Vec<u64> { 
    let start = seed_range.0;
    let end = seed_range.0 + seed_range.1;
    return (start..end).collect();
}

fn main() {
    let (seeds, all_seed_maps) = read_almanac("./data/input.txt");
    let mut p1_seeds = seeds.clone();
    let mut p2_seeds = seeds.clone();
       
    all_map_follow(&mut p1_seeds, &all_seed_maps);
    println!("The answer to part 1: {}", *p1_seeds.iter().min().unwrap());

    range_map_follow(&mut p2_seeds, &all_seed_maps);
    println!(
        "The answer to part 2: {}",
        *p2_seeds.iter().min().unwrap()
    );
}
