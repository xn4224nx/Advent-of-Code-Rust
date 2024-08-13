/*
 * --- Day 5: Doesn't He Have Intern-Elves For This? ---
 *
 * Santa needs help figuring out which strings in his text file
 * are naughty or nice.
 *
 * A nice string is one with all of the following properties:
 *
 *      - It contains at least three vowels (aeiou only),
 *      like aei, xazegov, or aeiouaeiouaeiou.
 *
 *      - It contains at least one letter that appears twice
 *      in a row, like xx, abcdde (dd), or aabbccdd (aa, bb,
 *      cc, or dd).
 *
 *      - It does not contain the strings ab, cd, pq, or xy,
 *      even if they are part of one of the other
 *      requirements.
 *
 * PART 1:  How many strings are nice?
 */

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

const VOWELS: &[u8] = &[97, 101, 105, 111, 117];
const FORBIDDEN_PAIRS: &[(u8, u8)] = &[(97, 98), (99, 100), (112, 113), (120, 121)];

/// Determine if a string is naughty
pub fn is_nice(sample: &Vec<u8>) -> bool {
    let mut vowel_cnt = 0;
    let mut double = false;

    for idx in 0..sample.len() {
        /* Check for the presence of a vowel. */
        if vowel_cnt < 3 && VOWELS.contains(&sample[idx]) {
            vowel_cnt += 1;
        };

        /* Compare the previous char to the current one. */
        if idx != 0 {
            /* Check for a duplicate letter. */
            if !double && sample[idx] == sample[idx - 1] {
                double = true;
            };

            /* Ensure there no forbidden substrings. */
            for (p1, p2) in FORBIDDEN_PAIRS.iter() {
                if *p1 == sample[idx - 1] && *p2 == sample[idx] {
                    return false;
                };
            }
        };
    }

    return double && vowel_cnt >= 3;
}

/// Count the number of nice strings in a file
fn count_nice_strings(filepath: &str) -> usize {
    let file = File::open(filepath).unwrap();
    let mut fp = BufReader::new(file);

    let mut nice_cnt = 0;
    let mut buffer = vec![];

    while fp.read_until(b'\n', &mut buffer).unwrap() > 0 {
        if is_nice(&buffer) {
            nice_cnt += 1;
        }

        buffer.clear();
    }
    return nice_cnt;
}

fn main() {
    println!("Part 1 = {}", count_nice_strings("./data/input.txt"));
}
