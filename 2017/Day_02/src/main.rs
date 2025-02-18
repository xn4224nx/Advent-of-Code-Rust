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
 */

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

    /// The sum of the ranges of each row in the spreadsheet
    pub fn checksum(&self) -> u32 {
        return (0..self.data.len()).map(|x| self.row_range(x)).sum::<u32>();
    }
}

fn main() {
    let corrupt = SpreadSheet::new("./data/input.txt");
    println!("Part 1 = {}", corrupt.checksum());
}
