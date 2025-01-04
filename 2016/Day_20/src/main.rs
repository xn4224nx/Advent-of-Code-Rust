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
 *
 * PART 2:  How many IPs are allowed by the blacklist?
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
        /* Order the ranges smallest to largest, elementwise. */
        tmp_ranges.sort();

        return BlackList { ranges: tmp_ranges };
    }

    /// Find the lowest allowed IP address by the set of blacklist range
    pub fn lowest_allowed_ip(&mut self) -> u32 {
        let mut min_ip = 0;

        /* Find the first rule to allow the current min_ip */
        for idx in 0..self.ranges.len() {
            if self.ranges[idx].0 <= min_ip && min_ip <= self.ranges[idx].1 {
                min_ip = self.ranges[idx].1 + 1
            }
        }
        return min_ip;
    }

    /// Count the number of allowed IP addresses
    pub fn num_allowed_ips(&self) -> u32 {
        let mut total = 0;
        let mut curr_ip = 0;
        let mut idx = 0;

        while curr_ip < u32::MAX {
            if curr_ip >= self.ranges[idx].0 {
                if curr_ip <= self.ranges[idx].1 {
                    curr_ip = self.ranges[idx].1.saturating_add(1);
                    continue;
                }
                idx += 1;
            } else {
                total += self.ranges[idx].0 - curr_ip;
                curr_ip = self.ranges[idx].0;
            }
        }
        return total;
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        BlackList::new("./data/input.txt").lowest_allowed_ip()
    );
    println!(
        "Part 2 = {}",
        BlackList::new("./data/input.txt").num_allowed_ips()
    );
}
