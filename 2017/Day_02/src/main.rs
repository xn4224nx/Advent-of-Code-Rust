/*
 * --- Day 2: Corruption Checksum ---
 *
 * As you walk through the door, a glowing humanoid shape yells in your
 * direction. "You there! Your state appears to be idle. Come help us repair the
 * corruption in this spreadsheet - if we take another millisecond, we'll have
 * to display an hourglass cursor!"
 *
 * The spreadsheet consists of rows of apparently-random numbers. To make sure
 * the recovery process is on the right track, they need you to calculate the
 * spreadsheet's checksum. For each row, determine the difference between the
 * largest value and the smallest value; the checksum is the sum of all of these
 * differences.
 *
 * For example, given the following spreadsheet:
 *
 *      5 1 9 5
 *      7 5 3
 *      2 4 6 8
 *
 *      -   The first row's largest and smallest values are 9 and 1, and their
 *          difference is 8.
 *
 *      -   The second row's largest and smallest values are 7 and 3, and their
 *          difference is 4.
 *
 *      -   The third row's difference is 6.
 *
 * In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
 *
 * PART 1:  What is the checksum for the spreadsheet in your puzzle input?
 *
 * "Great work; looks like we're on the right track after all. Here's a star for
 * your effort." However, the program seems a little worried. Can programs be
 * worried?
 *
 * "Based on what we're seeing, it looks like all the User wanted is some
 * information about the evenly divisible values in the spreadsheet.
 * Unfortunately, none of us are equipped for that kind of calculation - most
 * of us specialize in bitwise operations."
 *
 * It sounds like the goal is to find the only two numbers in each row where
 * one evenly divides the other - that is, where the result of the division
 * operation is a whole number. They would like you to find those numbers on
 * each line, divide them, and add up each line's result.
 *
 * For example, given the following spreadsheet:
 *
 *      5 9 2 8
 *      9 4 7 3
 *      3 8 6 5
 *
 *      -   In the first row, the only two numbers that evenly divide are 8 and
 *          2; the result of this division is 4.
 *
 *      -   In the second row, the two numbers are 9 and 3; the result is 3.
 *
 *      -   In the third row, the result is 2.
 *
 * In this example, the sum of the results would be 4 + 3 + 2 = 9.
 *
 * PART 2:  What is the sum of each row's result in your puzzle input?
 */

use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct SpreadSheet {
    pub data: Vec<Vec<u32>>,
}

impl SpreadSheet {
    pub fn new(data_file: &str) -> Self {
        let mut data: Vec<Vec<u32>> = Vec::new();
        let mut buffer = String::new();

        /* Open the file */
        let file = File::open(data_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Read the file line by line and convert the string into a Vec<u32>. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            data.push(
                buffer
                    .as_str()
                    .split_ascii_whitespace()
                    .filter_map(|x| x.parse::<u32>().ok())
                    .collect(),
            );
            buffer.clear();
        }
        return SpreadSheet { data };
    }

    /// Calculate the difference between the max and min in the row
    pub fn row_range(&self, row_idx: usize) -> u32 {
        let mut max_num = 0;
        let mut min_num = u32::MAX;

        /* Check each number to find the max and min. */
        for col_idx in 0..self.data[row_idx].len() {
            if self.data[row_idx][col_idx] > max_num {
                max_num = self.data[row_idx][col_idx]
            };

            if self.data[row_idx][col_idx] < min_num {
                min_num = self.data[row_idx][col_idx]
            };
        }
        return max_num - min_num;
    }

    /// Find the only two numbers in the row that are divisible and return
    /// the result of the division.
    pub fn row_divide(&self, row_idx: usize) -> u32 {
        /* Test each combination of numbers in the row */
        for mut comb in self.data[row_idx].iter().combinations(2) {
            comb.sort();

            /* If they are perfectly divisible return the division. */
            if comb[1] % comb[0] == 0 {
                return (comb[1] / comb[0]) as u32;
            }
        }
        panic!("No numbers were divisible!")
    }

    /// The sum of the ranges of each row in the spreadsheet
    pub fn checksum(&self, range: bool) -> u32 {
        return if range {
            (0..self.data.len()).map(|x| self.row_range(x)).sum::<u32>()
        } else {
            (0..self.data.len())
                .map(|x| self.row_divide(x))
                .sum::<u32>()
        };
    }
}

fn main() {
    let corrupt = SpreadSheet::new("./data/input.txt");
    println!(
        "Part 1 = {}\nPart 2 = {}",
        corrupt.checksum(true),
        corrupt.checksum(false)
    );
}
