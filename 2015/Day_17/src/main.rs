/*
 * --- Day 17: No Such Thing as Too Much ---
 *
 * The elves bought too much eggnog again - 150 liters this time. To fit it all
 * into your refrigerator, you'll need to move it into smaller containers. You
 * take an inventory of the capacities of the available containers.
 *
 * PART 1:  Filling all containers entirely, how many different combinations of
 *          containers can exactly fit all 150 liters of eggnog?
 *
 * While playing with all the containers in the kitchen, another load of eggnog
 * arrives! The shipping and receiving department is requesting as many
 * containers as you can spare.
 *
 * PART 2:  Find the minimum number of containers that can exactly fit all 150
 *          liters of eggnog. How many different ways can you fill that number of
 *          containers and still hold exactly 150 litres?
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

/// Ensure that a set of containers can contain the supplied volume of eggnog.
pub fn does_comb_fit(cont_comb: &Vec<&u32>, eggnog_vol: u32) -> bool {
    return cont_comb.iter().map(|x| *x).sum::<u32>() == eggnog_vol;
}

/// Test every combination of container and count the number of combinations
/// that can exactly fit the volume of eggnog.
pub fn count_cont_combs(containers: &Vec<u32>, eggnog_vol: u32, min_size: bool) -> u32 {
    let mut valid_combs = vec![0; containers.len()];

    /*
     * For each possible number of containers pick the valid number of
     * container combinations and test to see if it is valid.
     */
    for num_conts in 1..=containers.len() {
        for comb in containers.iter().combinations(num_conts) {
            if does_comb_fit(&comb, eggnog_vol) {
                valid_combs[num_conts - 1] += 1
            }
        }
    }

    /* Find the combinations for the smallest number of containers. */
    if min_size {
        for comb in valid_combs.iter() {
            if *comb != 0 {
                return *comb;
            }
        }
        panic!("No valid combination size found!");
    } else {
        return valid_combs.iter().sum::<u32>();
    }
}

fn main() {
    let conts = read_container_sizes("./data/input.txt");
    println!("Part 1 = {}", count_cont_combs(&conts, 150, false));
    println!("Part 2 = {}", count_cont_combs(&conts, 150, true));
}
