/*
 * --- Day 4: High-Entropy Passphrases ---
 *
 * A new system policy has been put in place that requires all accounts to use a
 * passphrase instead of simply a password. A passphrase consists of a series of
 * words (lowercase letters) separated by spaces.
 *
 * To ensure security, a valid passphrase must contain no duplicate words. For
 * example:
 *
 *      -   aa bb cc dd ee is valid.
 *      -   aa bb cc dd aa is not valid - the word aa appears more than once.
 *      -   aa bb cc dd aaa is valid - aa and aaa count as different words.
 *
 * PART 1:  The system's full passphrase list is available as your puzzle input.
 *          How many passphrases are valid?
 */

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Password {
    pub word_counts: Vec<HashMap<char, u8>>,
    pub words: Vec<String>,
}

impl Password {
    pub fn new(raw_password: &str) -> Self {
        Password {
            word_counts: Vec::new(),
            words: Vec::new(),
        }
    }

    pub fn duplicate_words(&self) -> bool {
        false
    }
}

pub fn parse_passwords(data_file: &str) -> Vec<Password> {
    Vec::new()
}

pub fn count_valid_passwords(all_paswrds: &Vec<Password>) -> u32 {
    0
}

fn main() {}
