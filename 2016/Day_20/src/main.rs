/*
 * --- Day 20: Firewall Rules ---
 *
 * You'd like to set up a small hidden computer here so you can use it to get
 * back into the network later. However, the corporate firewall only allows
 * communication with certain external IP addresses.
 *
 * You've retrieved the list of blocked IPs from the firewall, but the list
 * seems to be messy and poorly maintained, and it's not clear which IPs are
 * allowed. Also, rather than being written in dot-decimal notation, they are
 * written as plain 32-bit integers, which can have any value from 0 through
 * 4294967295, inclusive.
 *
 * For example, suppose only the values 0 through 9 were valid, and that you
 * retrieved the following blacklist:
 *
 *      5-8
 *      0-2
 *      4-7
 *
 * The blacklist specifies ranges of IPs (inclusive of both the start and end
 * value) that are not allowed. Then, the only IPs that this firewall allows are
 * 3 and 9, since those are the only numbers not in any range.
 *
 * PART 1:  Given the list of blocked IPs you retrieved from the firewall (your
 *          puzzle input), what is the lowest-valued IP that is not blocked?
 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct BlackList {
    pub ranges: Vec<(u32, u32)>,
}

impl BlackList {
    pub fn new(data_file: &str) -> Self {
        let mut tmp_ranges = Vec::new();
        let mut buffer = String::new();
        let blk_re = Regex::new(r"(\d+)\-(\d+)").unwrap();

        /* Open the data file. */
        let file = File::open(data_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Read the file line by line and parse the IP addresses. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            if let Some(caps) = blk_re.captures(&buffer) {
                tmp_ranges.push((
                    caps[1].parse::<u32>().unwrap(),
                    caps[2].parse::<u32>().unwrap(),
                ))
            }
            buffer.clear();
        }
        return BlackList { ranges: tmp_ranges };
    }

    /// Sort the ranges and combine the overlapping ones
    pub fn compress_ranges(&mut self) {
        let mut changes_made = true;

        /* Order the ranges smallest to largest, elementwise. */
        self.ranges.sort();

        while changes_made {
            changes_made = false;
            let mut new_ranges = Vec::new();

            /* Assess the blacklist ranges in pairs */
            for idx in (1..self.ranges.len()).step_by(2) {
                let rng_0 = self.ranges[idx - 1];
                let rng_1 = self.ranges[idx];

                /* If there is some overlap in the ranges, combine them. */
                if rng_0.1 >= rng_1.0 {
                    new_ranges.push((rng_0.0, rng_1.1));
                    changes_made = true;

                /* Otherwise return both ranges */
                } else {
                    new_ranges.push(rng_0);
                    new_ranges.push(rng_1);
                }
            }

            /* If the total ranges were odd save the remaining one. */
            if self.ranges.len() % 2 != 0 {
                let f_idx = new_ranges.len() - 1;

                /* Then check if a combine could be made using the odd range. */
                if new_ranges[f_idx].1 >= self.ranges[self.ranges.len() - 1].0 {
                    new_ranges[f_idx].1 = self.ranges[self.ranges.len() - 1].1;
                    changes_made = true;

                /* Or add it to the end of the of this batch */
                } else {
                    new_ranges.push(self.ranges[self.ranges.len() - 1]);
                }
            }

            /* Overwrite the old with the new. */
            self.ranges = new_ranges;
        }
    }

    /// Find the lowest allowed IP address by the set of blacklist range
    pub fn lowest_allowed_ip(&mut self) -> u32 {
        let mut lowest_ip = 0;
        self.compress_ranges();

        for idx in 0..self.ranges.len() {
            /* If the current lowest ip is in this blacklist range increase it. */
            if lowest_ip >= self.ranges[idx].0 && lowest_ip <= self.ranges[idx].1 {
                lowest_ip = self.ranges[idx].1 + 1;

            /* If not we have a solution! */
            } else {
                return lowest_ip;
            }
        }

        /* We assume here that the lowest IP is one above the final blacklist. */
        return lowest_ip;
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        BlackList::new("./data/input.txt").lowest_allowed_ip()
    );
}
