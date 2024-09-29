/*
 * --- Day 24: It Hangs in the Balance ---
 *
 * It's Christmas Eve, and Santa is loading up the sleigh for this year's
 * deliveries. However, there's one small problem: he can't get the sleigh to
 * balance. If it isn't balanced, he can't defy physics, and nobody gets
 * presents this year.
 *
 * No pressure.
 *
 * Santa has provided you a list of the weights of every package he needs to fit
 * on the sleigh. The packages need to be split into three groups of exactly the
 * same weight, and every package has to fit. The first group goes in the
 * passenger compartment of the sleigh, and the second and third go in
 * containers on either side. Only when all three groups weigh exactly the same
 * amount will the sleigh be able to fly. Defying physics has rules, you know!
 *
 * Of course, that's not the only problem. The first group - the one going in
 * the passenger compartment - needs as few packages as possible so that Santa
 * has some legroom left over. It doesn't matter how many packages are in either
 * of the other two groups, so long as all of the groups weigh the same.
 *
 * Furthermore, Santa tells you, if there are multiple ways to arrange the
 * packages such that the fewest possible are in the first group, you need to
 * choose the way where the first group has the smallest quantum entanglement to
 * reduce the chance of any "complications". The quantum entanglement of a group
 * of packages is the product of their weights, that is, the value you get when
 * you multiply their weights together. Only consider quantum entanglement if
 * the first group has the fewest possible number of packages in it and all
 * groups weigh the same amount.
 *
 * PART 1:  What is the quantum entanglement of the first group of packages in
 *          the ideal configuration?
 */

use itertools::Itertools;
use std::fs::read_to_string;

/// Read the box weights from a file
pub fn read_box_weights(file_path: &str) -> Vec<u64> {
    read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

/// What is the quantum entanglement of this grouping
pub fn qe_calc(grouping: &Vec<Vec<u64>>) -> u64 {
    grouping[0].iter().product()
}

/// Find the smallest possible quantum entanglement for a group of boxes
pub fn find_ideal_config_qe(boxes: &Vec<u64>, groups: usize) -> u64 {
    let max_first_grp_size = boxes.len() - (groups - 1);
    let group_weight = boxes.iter().sum::<u64>() / groups as u64;
    let mut lowest_valid_size_found = false;
    let mut lowest_qe = u64::MAX;

    /* We only care about the smallest valid first group. */
    for first_grp_size in 1..=max_first_grp_size {
        let rem_pos_grp_sizes = boxes.len() - first_grp_size;
        let max_other_grp = rem_pos_grp_sizes - (groups - 1);

        if lowest_valid_size_found {
            break;
        }

        println!("First group size = {}", first_grp_size);

        /* Pick possible group sizes. */
        for other_grps in (1..=max_other_grp).combinations_with_replacement(groups - 1) {
            let group_sizes = [vec![first_grp_size], other_grps].concat();

            /* Ensure that the group combination is valid. */
            if group_sizes.iter().sum::<usize>() != boxes.len() {
                continue;
            };

            /* Pick the contents of the first group. */
            for grp_0 in boxes.iter().combinations(group_sizes[0]) {
                if grp_0.iter().map(|x| **x).sum::<u64>() != group_weight {
                    continue;
                };

                let mut rem_boxes_1: Vec<u64> = boxes
                    .iter()
                    .filter(|x| !grp_0.contains(x))
                    .map(|x| *x)
                    .collect();

                /* Pick the second group. */
                for grp_1 in rem_boxes_1.iter().combinations(group_sizes[1]) {
                    if grp_1.iter().map(|x| **x).sum::<u64>() != group_weight {
                        continue;
                    };

                    let mut rem_boxes_2: Vec<u64> = rem_boxes_1
                        .iter()
                        .filter(|x| !grp_1.contains(x))
                        .map(|x| *x)
                        .collect();

                    /* Pick the third group. */
                    for grp_2 in rem_boxes_2.iter().combinations(group_sizes[2]) {
                        if grp_2.iter().map(|x| **x).sum::<u64>() != group_weight {
                            continue;
                        };

                        /* Calculate the qe of this combination  */
                        lowest_valid_size_found = true;
                        let qe = grp_0.iter().map(|x| **x).product();

                        if qe < lowest_qe {
                            println!("New qe found = {}", qe);
                            lowest_qe = qe;
                        }
                    }
                }
            }
        }
    }

    return lowest_qe;
}

fn main() {
    let boxes = read_box_weights("./data/input.txt");
    println!("Part 1 = {}", find_ideal_config_qe(&boxes, 3));
}
