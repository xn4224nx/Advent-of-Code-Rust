/*
 * --- Day 17: No Such Thing as Too Much ---
 *
 * The elves bought too much eggnog again - 150 liters this time. To fit it all
 * into your refrigerator, you'll need to move it into smaller containers. You
 * take an inventory of the capacities of the available containers.
 *
 * PART 1:  Filling all containers entirely, how many different combinations of
 *          containers can exactly fit all 150 liters of eggnog?
 */

use itertools::Itertools;
use std::fs;

/// Read the container sizes and return a vector of the sizes
pub fn read_container_sizes(container_file: &str) -> Vec<u32> {
    return fs::read_to_string(container_file)
        .unwrap()
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
}

/// Ensure that a set of containers can contain the supplied volume of eggnog
pub fn does_comb_fit(cont_comb: &Vec<&u32>, eggnog_vol: u32) -> bool {
    return cont_comb.iter().map(|x| *x).sum::<u32>() == eggnog_vol;
}

/// Test every combination of container and count the number of combinations
/// that can exactly fit the volume of eggnog.
pub fn count_cont_combs(containers: &Vec<u32>, eggnog_vol: u32) -> u32 {
    let mut valid_combs = 0;

    /*
     * For each possible number of containers pick the valid number of
     * container combinations and test to see if it is valid.
     */
    for num_conts in 1..=containers.len() {
        for comb in containers.iter().combinations(num_conts) {
            if does_comb_fit(&comb, eggnog_vol) {
                valid_combs += 1;
            }
        }
    }
    return valid_combs;
}

fn main() {
    let conts = read_container_sizes("./data/input.txt");
    println!("Part 1 = {}", count_cont_combs(&conts, 150));
}
