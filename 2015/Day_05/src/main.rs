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
 *
 * Realizing the error of his ways, Santa has switched to a
 * better model of determining whether a string is naughty or
 * nice. None of the old rules apply, as they are all clearly
 * ridiculous.
 *
 * Now, a nice string is one with all of the following
 * properties:
 *
 *      - It contains a pair of any two letters that appears
 *      at least twice in the string without overlapping,
 *      like xyxy (xy) or aabcdefgaa (aa), but not like aaa
 *      (aa, but it overlaps).
 *
 *      - It contains at least one letter which repeats with
 *      exactly one letter between them, like xyx, abcdefeghi
 *      (efe), or even aaa.
 *
 * PART 2:  How many strings are nice under these new rules?
 *
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

/// Determine if a string is the second type of naughty
pub fn is_nice2(sample: &Vec<u8>) -> bool {
    let mut alternate_letter = false;
    let mut repeating_pair = false;

    for idx in 0..sample.len() {
        /* Extract the pairs in the string. */
        if !repeating_pair && idx + 2 < sample.len() {
            /* Check for a repeating pair. */
            for jdx in idx + 2..sample.len() - 1 {
                /* Extract the relevant pair */
                let t_slice = &sample[idx..idx + 2];

                if &sample[jdx..jdx + 2] == t_slice {
                    repeating_pair = true;
                    break;
                }
            }
        }

        /* Check for an alternating letter. */
        if idx >= 2 && sample[idx] == sample[idx - 2] {
            alternate_letter = true;
        }
    }

    return alternate_letter && repeating_pair;
}

/// Count the number of nice strings in a file.
fn count_nice_strings(filepath: &str, nice_fn: &dyn Fn(&Vec<u8>) -> bool) -> usize {
    let file = File::open(filepath).unwrap();
    let mut fp = BufReader::new(file);

    let mut nice_cnt = 0;
    let mut buffer = vec![];

    while fp.read_until(b'\n', &mut buffer).unwrap() > 0 {
        if nice_fn(&buffer) {
            nice_cnt += 1;
        }

        buffer.clear();
    }
    return nice_cnt;
}

fn main() {
    println!(
        "Part 1 = {}",
        count_nice_strings("./data/input.txt", &is_nice)
    );
    println!(
        "Part 2 = {}",
        count_nice_strings("./data/input.txt", &is_nice2)
    );
}
