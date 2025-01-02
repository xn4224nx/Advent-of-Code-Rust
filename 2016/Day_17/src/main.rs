/*
 * --- Day 17: Two Steps Forward ---
 *
 * You're trying to access a secure vault protected by a 4x4 grid of small
 * rooms connected by doors. You start in the top-left room (marked S), and you
 * can access the vault (marked V) once you reach the bottom-right room:
 *
 *      #########
 *      #S| | | #
 *      #-#-#-#-#
 *      # | | | #
 *      #-#-#-#-#
 *      # | | | #
 *      #-#-#-#-#
 *      # | | |
 *      ####### V
 *
 * Fixed walls are marked with #, and doors are marked with - or |.
 *
 * The doors in your current room are either open or closed (and locked) based
 * on the hexadecimal MD5 hash of a passcode (your puzzle input) followed by a
 * sequence of uppercase characters representing the path you have taken so far
 * (U for up, D for down, L for left, and R for right).
 *
 * Only the first four characters of the hash are used; they represent,
 * respectively, the doors up, down, left, and right from your current
 * position. Any b, c, d, e, or f means that the corresponding door is open;
 * any other character (any number or a) means that the corresponding door is
 * closed and locked.
 *
 * To access the vault, all you need to do is reach the bottom-right room;
 * reaching this room opens the vault and all doors in the maze.
 *
 * PART 1:  Given your vault's passcode, what is the shortest path (the actual
 *          path, not just the length) to reach the vault?
 */

use md5;
use std::collections::HashSet;

pub struct Maze {
    pub strt_pnt: (usize, usize),
    pub end_pnt: (usize, usize),
    pub seed: String,
}

impl Maze {
    pub fn new(seed: &str) -> Self {
        Maze {
            strt_pnt: (0, 0),
            end_pnt: (3, 3),
            seed: String::from(seed),
        }
    }

    /// Find out where in the maze a set of instructions will take you
    pub fn directs_2_coords(&self, direcs: &Vec<char>) -> (usize, usize) {
        let mut pnt = self.strt_pnt;

        /* Assume that all move are valid. */
        for d_char in direcs.iter() {
            pnt = match d_char {
                'U' => (pnt.0, pnt.1 - 1),
                'D' => (pnt.0, pnt.1 + 1),
                'L' => (pnt.0 - 1, pnt.1),
                'R' => (pnt.0 + 1, pnt.1),
                _ => panic!("Unknown direction - {}", d_char),
            }
        }
        return pnt;
    }

    /// Determine the viable next steps in the maze based on the previous steps
    pub fn viable_dirs(&self, direcs: &Vec<char>) -> Vec<char> {
        let valid_digest_chars = vec!['b', 'c', 'd', 'e', 'f'];
        let mut new_direcs = Vec::new();

        /* Determine where in the maze you are. */
        let curr_loc = self.directs_2_coords(direcs);

        /* Convert the previous directions to a String. */
        let prev_direcs: String = direcs.iter().collect();

        /* Create the message to be hashed. */
        let msg: String = format!("{}{}", self.seed, prev_direcs);

        /* Calculate the hash digest of the past directions. */
        let digest = format!("{:x}", md5::compute(msg.as_bytes()));

        /* Determine the valid directions based on the first four chars. */
        for (idx, t_char) in digest.chars().enumerate() {
            if idx > 3 {
                break;
            };

            /* Only allow movement if a certain char appears. */
            if valid_digest_chars.contains(&t_char) {
                if idx == 0 && curr_loc.1 > 0 {
                    new_direcs.push('U')
                } else if idx == 1 && curr_loc.1 < self.end_pnt.1 {
                    new_direcs.push('D')
                } else if idx == 2 && curr_loc.0 > 0 {
                    new_direcs.push('L')
                } else if idx == 3 && curr_loc.0 < self.end_pnt.0 {
                    new_direcs.push('R')
                }
            };
        }
        return new_direcs;
    }

    /// Find the shortest path through the maze
    pub fn find_shortest_path(&self) -> String {
        let mut possible_valid_paths = HashSet::from([Vec::new()]);

        loop {
            let mut all_new_paths = HashSet::new();

            /* For each previous paths work out the next possible paths. */
            for old_path in &possible_valid_paths {
                let nxt_pos_dirs = self.viable_dirs(old_path);

                /* Create the new paths for each direction. */
                for nxt_d in nxt_pos_dirs.iter() {
                    let mut new_path = old_path.clone();
                    new_path.push(*nxt_d);

                    /* Check for a solution. */
                    if self.directs_2_coords(&new_path) == self.end_pnt {
                        return new_path.iter().collect();
                    }

                    /* Otherwise save it for the next iteration. */
                    all_new_paths.insert(new_path);
                }
            }

            /* Overwrite the old paths with the new. */
            if all_new_paths.is_empty() {
                panic!("No viable next steps found!");
            } else {
                possible_valid_paths = all_new_paths;
            }
        }
    }
}

fn main() {}
