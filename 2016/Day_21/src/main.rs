/*
 * --- Day 21: Scrambled Letters and Hash ---
 *
 * The computer system you're breaking into uses a weird scrambling function to
 * store its passwords. It shouldn't be much trouble to create your own
 * scrambled password so you can add it to the system; you just have to
 * implement the scrambler.
 *
 * The scrambling function is a series of operations (the exact list is provided
 * in your puzzle input). Starting with the password to be scrambled, apply each
 * operation in succession to the string. The individual operations behave as
 * follows:
 *
 *      -   swap position X with position Y means that the letters at indexes X
 *          and Y (counting from 0) should be swapped.
 *
 *      -   swap letter X with letter Y means that the letters X and Y should be
 *          swapped (regardless of where they appear in the string).
 *
 *      -   rotate left/right X steps means that the whole string should be
 *          rotated; for example, one right rotation would turn abcd into dabc.
 *
 *      -   rotate based on position of letter X means that the whole string
 *          should be rotated to the right based on the index of letter X
 *          (counting from 0) as determined before this instruction does any
 *          rotations. Once the index is determined, rotate the string to the
 *          right one time, plus a number of times equal to that index, plus one
 *          additional time if the index was at least 4.
 *
 *      -   reverse positions X through Y means that the span of letters at
 *          indexes X through Y (including the letters at X and Y) should be
 *          reversed in order.
 *
 *      -   move position X to position Y means that the letter which is at
 *          index X should be removed from the string, then inserted such that
 *          it ends up at index Y.
 *
 *  PART 1: Now, you just need to generate a new scrambled password and you can
 *          access the system. Given the list of scrambling operations in your
 *          puzzle input, what is the result of scrambling abcdefgh?
 */

use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Debug)]
pub enum Command {
    SwapIndex(usize, usize),
    SwapLetter(char, char),
    Rotate(i32),
    RotateLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

pub struct SecretHasher {
    pub initial_letters: Vec<char>,
    pub curr_letters: VecDeque<char>,
    pub instructs: Vec<Command>,
}

impl SecretHasher {
    pub fn new(starting_letters: &str, instruc_file: &str) -> Self {
        let initial_letters: Vec<char> = starting_letters.chars().collect();
        let curr_letters = starting_letters.chars().collect::<VecDeque<char>>();
        let mut instructs = Vec::new();

        let re_pats = vec![
            Regex::new(r"swap position (\d+) with position (\d+)").unwrap(),
            Regex::new(r"swap letter ([a-z]) with letter ([a-z])").unwrap(),
            Regex::new(r"rotate right (\d+) steps?").unwrap(),
            Regex::new(r"rotate left (\d+) steps?").unwrap(),
            Regex::new(r"rotate based on position of letter ([a-z])").unwrap(),
            Regex::new(r"reverse positions (\d+) through (\d+)").unwrap(),
            Regex::new(r"move position (\d+) to position (\d+)").unwrap(),
        ];

        /* Open the file */
        let file = File::open(instruc_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Iterate over the file line by line. */
        let mut buffer = String::new();
        while fp.read_line(&mut buffer).unwrap() > 0 {
            let prev_num_comms = instructs.len();

            /* Test each regex pattern and extract the values. */
            for (idx, comm_re) in re_pats.iter().enumerate() {
                if let Some(caps) = comm_re.captures(&buffer) {
                    instructs.push(match idx {
                        0 => Command::SwapIndex(
                            caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                            caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                        ),
                        1 => Command::SwapLetter(
                            caps.get(1).unwrap().as_str().chars().nth(0).unwrap(),
                            caps.get(2).unwrap().as_str().chars().nth(0).unwrap(),
                        ),
                        2 => Command::Rotate(caps.get(1).unwrap().as_str().parse::<i32>().unwrap()),
                        3 => Command::Rotate(
                            -1 * caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                        ),
                        4 => Command::RotateLetter(
                            caps.get(1).unwrap().as_str().chars().nth(0).unwrap(),
                        ),
                        5 => Command::Reverse(
                            caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                            caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                        ),
                        6 => Command::Move(
                            caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                            caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                        ),
                        _ => panic!("Unknown pattern encountered!"),
                    });

                    /* Once a match is found attempt no more matching. */
                    break;
                }
            }

            /* Check for a line matching no patterns at all */
            if instructs.len() == prev_num_comms {
                buffer.truncate(buffer.len() - 1);
                println!("WARNING: '{}' not a recognised command!", buffer);
            }
            buffer.clear();
        }

        return SecretHasher {
            initial_letters,
            curr_letters,
            instructs,
        };
    }

    /// Take a command and mutate the current letters according to the command
    pub fn impl_command(&mut self, comm_idx: usize) {
        match self.instructs[comm_idx] {
            Command::SwapIndex(idx0, idx1) => {
                self.curr_letters.swap(idx0, idx1);
            }
            Command::SwapLetter(chr0, chr1) => {
                let mut idx_chr0 = 0;
                let mut idx_chr1 = 0;

                /* Find the index of each of the letters. */
                for (idx, val) in self.curr_letters.iter().enumerate() {
                    if *val == chr0 {
                        idx_chr0 = idx;
                    } else if *val == chr1 {
                        idx_chr1 = idx;
                    };

                    /* If both letters have been found stop looking. */
                    if idx_chr0 != 0 && idx_chr1 != 0 {
                        break;
                    };
                }
                self.curr_letters.swap(idx_chr0, idx_chr1);
            }
            Command::Rotate(shf) => {
                if shf >= 0 {
                    self.curr_letters.rotate_right(shf as usize)
                } else {
                    self.curr_letters.rotate_left(shf.abs() as usize)
                };
            }
            Command::RotateLetter(chr0) => {
                let mut idx_chr0 = self.curr_letters.iter().position(|x| *x == chr0).unwrap();

                /* Enlarge the rotation if its four or over. */
                if idx_chr0 >= 4 {
                    idx_chr0 += 2;
                } else {
                    idx_chr0 += 1;
                };

                /* Handle the case of multiple cycles of rotation. */
                idx_chr0 %= self.curr_letters.len();

                self.curr_letters.rotate_right(idx_chr0);
            }
            Command::Reverse(idx0, idx1) => {
                let loop_size = (idx1 - idx0) / 2;

                /* Swap pairs, moving outer to inner.  */
                for offset in 0..loop_size {
                    self.curr_letters.swap(idx0 + offset, idx1 - offset)
                }
            }
            Command::Move(idx0, idx1) => {
                let chr0 = self.curr_letters.remove(idx0).unwrap();
                self.curr_letters.insert(idx1, chr0);
            }
        }
    }

    /// Show the letters in there current state
    pub fn show(&self) -> String {
        return self.curr_letters.iter().collect();
    }

    /// Find the final form of the letters after all commands
    pub fn final_state(&mut self) -> String {
        for cmd_idx in 0..self.instructs.len() {
            self.impl_command(cmd_idx);
        }
        return self.show();
    }
}

fn main() {}
