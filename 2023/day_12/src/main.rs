/*
 * --- Day 12: Hot Springs ---
 *
 * There's just one problem - many of the springs have fallen into
 * disrepair, so they're not actually sure which springs would even be
 * safe to use! Worse yet, their condition records of which springs are
 * damaged (your puzzle input) are also damaged! You'll need to help
 * them repair the damaged records.
 *
 * In the giant field just outside, the springs are arranged into rows.
 * For each row, the condition records show every spring and whether it
 * is operational (.) or damaged (#). This is the part of the condition
 * records that is itself damaged; for some springs, it is simply
 * unknown (?) whether the spring is operational or damaged.
 *
 * However, the engineer that produced the condition records also
 * duplicated some of this information in a different format! After the
 * list of springs for a given row, the size of each contiguous group of
 * damaged springs is listed in the order those groups appear in the
 * row. This list always accounts for every damaged spring, and each
 * number is the entire size of its contiguous group (that is, groups
 * are always separated by at least one operational spring: #### would
 * always be 4, never 2,2).
 *
 * Equipped with this information, it is your job to figure out how many
 * different arrangements of operational and broken springs fit the
 * given criteria in each row.
 *
 * Part 1 - For each row, count all of the different arrangements of
 *          operational and broken springs that meet the given criteria.
 *          What is the sum of those counts?
 */

use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Parse a file with spring condition data into a structured format
pub fn read_spring_condition_data(file_path: &str) -> Vec<(String, Vec<usize>)> {
    let mut spring_combination = Vec::new();

    /* Read the file into a buffer. */
    let file = File::open(file_path).expect("File could not be opened!");
    let reader = BufReader::new(file);

    /* Iterate over the file line by line. */
    for raw_line in reader.lines() {
        /* Check the line can be read otherwise skip it. */
        let Ok(clean_line) = raw_line else {
            continue;
        };

        /* Split the line into two based on the whitespace. */
        let mut parts = clean_line.split(" ");

        /* Extract the first part as the record of spring status. */
        let spring_record = parts.next().unwrap().to_string();

        /* Parse the second part as a vector of integers. */
        let groups = parts
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        spring_combination.push((spring_record, groups));
    }

    return spring_combination;
}

/// Count the number of visible and hidden working and broken springs
/// Returns a tuple of the key values:
/// (visible broken, hidden broken, visible working , hidden working)
/// (#, ? == #, ., ? == .)
pub fn generate_spring_stats(
    init_config: &String,
    broken_groups: &Vec<usize>,
) -> (usize, usize, usize, usize) {
    let mut wrk_spring_cnt = 0;
    let mut brk_spring_cnt = 0;

    /* Determine the contents of the inital spring configration string. */
    for spring_char in init_config.chars() {
        match spring_char {
            '#' => brk_spring_cnt += 1,
            '.' => wrk_spring_cnt += 1,
            _ => continue,
        };
    }

    /* Determine the total spring type counts from the working groups */
    let total_brk: usize = broken_groups.iter().sum();
    let total_wrk: usize = init_config.chars().count() - total_brk;

    /* Work out the contents of the lost data. */
    let missing_wrk = total_wrk - wrk_spring_cnt;
    let missing_brk = total_brk - brk_spring_cnt;

    return (brk_spring_cnt, missing_brk, wrk_spring_cnt, missing_wrk);
}

/// Determine if a spring configuration matches the groups of damaged
/// springs. Returns a bool based on if the match is correct.
pub fn validate_spring_config(
    init_config: &String,
    broken_groups: &Vec<usize>,
    consistent_order: bool,
) -> bool {
    let re_brk_springs = Regex::new(r"(#+)").unwrap();

    /* Count the length of the broken groups in the spring config str */
    let broke_grps_guess: Vec<usize> = re_brk_springs
        .captures_iter(init_config)
        .filter_map(|x| x.get(0))
        .map(|x| x.as_str().chars().count())
        .collect();

    if consistent_order {
        return *broken_groups == broke_grps_guess;
    } else {
        /* Cast both broken groups as sets and check they are identical. */
        let brk_grp_0: HashSet<usize> = broke_grps_guess.into_iter().collect();
        let brk_grp_1: HashSet<usize> = broken_groups.into_iter().cloned().collect();

        return brk_grp_0 == brk_grp_1;
    }
}

/// For a hidden config find the possible unhidden configs
pub fn gen_valid_spring_config(init_config: &String, broken_groups: &Vec<usize>) -> Vec<String> {
    let mut valid_spr_combs = Vec::new();

    let (s_brk_spr, h_brk_spr, s_wrk_spr, h_wrk_spr) =
        generate_spring_stats(init_config, broken_groups);

    /* Generate the possible missing springs */
    let miss_spr: Vec<char> = [vec!['#'; h_brk_spr], vec!['.'; h_wrk_spr]].concat();

    /* Iterate over all possible permutations of the unseen springs. */
    for hidd_sprs in miss_spr.iter().permutations(miss_spr.len()).unique() {
        let mut miss_cnt = 0;
        let mut tmp_config: Vec<char> = Vec::new();

        /* Create a new version of the spring config with the unknowns filled in. */
        for spr in init_config.chars() {
            if spr == '?' {
                tmp_config.push(*hidd_sprs[miss_cnt]);
                miss_cnt += 1;
            } else {
                tmp_config.push(spr)
            }
        }

        /* Convert the vector of chars to a string */
        let tmp_config = tmp_config.into_iter().collect();

        /* Only accept valid configurations. */
        if validate_spring_config(&tmp_config, broken_groups, true) {
            valid_spr_combs.push(tmp_config);
        }
    }

    /* Remove duplicate configurations. */
    let valid_spr_combs = valid_spr_combs.into_iter().unique().collect();

    return valid_spr_combs;
}

fn main() {
    let spring_data = read_spring_condition_data("./data/example_01.txt");
}
