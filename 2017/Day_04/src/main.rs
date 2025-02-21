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
 *
 * For added security, yet another system policy has been put in place. Now, a
 * valid passphrase must contain no two words that are anagrams of each other -
 * that is, a passphrase is invalid if any word's letters can be rearranged to
 * form any other word in the passphrase.
 *
 * For example:
 *
 *      -   abcde fghij is a valid passphrase.
 *
 *      -   abcde xyz ecdab is not valid - the letters from the third word can
 *          be rearranged to form the first word.
 *
 *      -   a ab abc abd abf abj is a valid passphrase, because all letters need
 *          to be used when forming another word.
 *
 *      -   iiii oiii ooii oooi oooo is valid.
 *
 *      -   oiii ioii iioi iiio is not valid - any of these words can be
 *          rearranged to form any other word.
 *
 * PART 2:  Under this new system policy, how many passphrases are valid?
 */

use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub struct Password {
    pub word_counts: Vec<HashMap<char, u8>>,
    pub words: Vec<String>,
}

impl Password {
    pub fn new(raw_password: &str) -> Self {
        let mut words = Vec::new();
        let mut word_counts = Vec::new();

        /* Analyse each word in the password sequentially. */
        for word in raw_password.split_ascii_whitespace() {
            let mut tmp_wrd_cnt = HashMap::new();

            /* Count the occurance of each character in the word. */
            for wrd_chr in word.chars() {
                *tmp_wrd_cnt.entry(wrd_chr).or_insert(0) += 1
            }
            word_counts.push(tmp_wrd_cnt);
            words.push(String::from(word));
        }

        return Password { word_counts, words };
    }

    /// Detect duplicate words in the password
    pub fn duplicate_words(&self) -> bool {
        for comb in (0..self.words.len()).combinations(2) {
            if self.words[comb[0]] == self.words[comb[1]] {
                return true;
            }
        }
        return false;
    }

    /// Determine if any words are anagrams of each other.
    pub fn anagrams_present(&self) -> bool {
        for comb in (0..self.word_counts.len()).combinations(2) {
            if self.word_counts[comb[0]] == self.word_counts[comb[1]] {
                return true;
            }
        }
        return false;
    }
}

pub fn parse_passwords(data_file: &str) -> Vec<Password> {
    let mut data = Vec::new();
    let mut buffer = String::new();

    /* Open the file. */
    let file = File::open(data_file).unwrap();
    let mut fp = BufReader::new(file);

    /* Read the file line by line. */
    while fp.read_line(&mut buffer).unwrap() > 0 {
        data.push(Password::new(&buffer));
        buffer.clear();
    }
    return data;
}

pub fn count_valid_passwords(all_paswrds: &Vec<Password>, anagrams: bool) -> usize {
    if anagrams {
        return all_paswrds.iter().filter(|x| !x.anagrams_present()).count();
    } else {
        return all_paswrds.iter().filter(|x| !x.duplicate_words()).count();
    }
}

fn main() {
    let sys_pass = parse_passwords("./data/input.txt");
    println!("Part 1 = {}", count_valid_passwords(&sys_pass, false));
    println!("Part 2 = {}", count_valid_passwords(&sys_pass, true));
}
