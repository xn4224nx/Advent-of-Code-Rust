/*
 * --- Day 19: A Series of Tubes ---
 *
 * Somehow, a network packet got lost and ended up here. It's trying to follow a
 * routing diagram (your puzzle input), but it's confused about where to go.
 *
 * Its starting point is just off the top of the diagram. Lines (drawn with |,
 * -, and +) show the path it needs to take, starting by going down onto the
 * only line connected to the top of the diagram. It needs to follow this path
 * until it reaches the end (located somewhere within the diagram) and stop
 * there.
 *
 * Sometimes, the lines cross over each other; in these cases, it needs to
 * continue going the same direction, and only turn left or right when there's
 * no other option. In addition, someone has left letters on the line; these
 * also don't change its direction, but it can use them to keep track of where
 * it's been. For example:
 *
 *     |
 *     |  +--+
 *     A  |  C
 * F---|----E|--+
 *     |  |  |  D
 *     +B-+  +--+
 *
 * Given this diagram, the packet needs to take the following path:
 *
 *      -   Starting at the only line touching the top of the diagram, it must
 *          go down, pass through A, and continue onward to the first +.
 *
 *      -   Travel right, up, and right, passing through B in the process.
 *
 *      -   Continue down (collecting C), right, and up (collecting D).
 *
 *      -   Finally, go all the way left through E and stopping at F.
 *
 * Following the path to the end, the letters it sees on its path are ABCDEF.
 *
 * PART 1:  The little packet looks up at you, hoping you can help it find the
 *          way. What letters will it see (in the order it would see them) if it
 *          follows the path? (The routing diagram is very wide; make sure you
 *          view it without line wrapping.)
 */

use num::complex::Complex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Network {
    pub pipes: HashMap<(i32, i32), char>,
    pub direction: Complex<i32>,
    pub path: Vec<(i32, i32)>,
}

impl Network {
    pub fn new(datafile: &str) -> Self {
        let mut buffer = String::new();
        let mut pipes = HashMap::new();
        let mut path = Vec::new();

        /* Open the datafile. */
        let file = File::open(datafile).unwrap();
        let mut fp = BufReader::new(file);

        /* Iterate over the file line by line. */
        let mut row_idx = 0;
        while fp.read_line(&mut buffer).unwrap() > 0 {
            for (col_idx, sqr) in buffer.chars().enumerate() {
                if sqr.is_whitespace() {
                    continue;
                }

                /* Save all the visible parts of the network. */
                pipes.insert((col_idx as i32, row_idx), sqr);

                /* Save the first visible element as the start of the path. */
                if path.len() < 1 {
                    path.push((col_idx as i32, row_idx));
                }
            }
            row_idx += 1;
            buffer.clear();
        }

        return Network {
            pipes,
            direction: Complex::new(0, 1),
            path,
        };
    }

    /// Determine the next step along the path through the network.
    pub fn step(&mut self) {
        let curr_loc = self.path.last().unwrap();
        let direct_0 = self.direction * Complex::i();
        let direct_1 = self.direction * -1 * Complex::i();

        /* The location that would be reached by just continuing. */
        let cont_loc = (
            curr_loc.0 + self.direction.re,
            curr_loc.1 + self.direction.im,
        );

        /* The location reached by turning in direction 0. */
        let loc_0 = (curr_loc.0 + direct_0.re, curr_loc.1 + direct_0.im);

        /* The location reached by turning in direction 1. */
        let loc_1 = (curr_loc.0 + direct_1.re, curr_loc.1 + direct_1.im);

        /* Investigate the possible paths. */
        if self.pipes.contains_key(&cont_loc) {
            self.path.push(cont_loc);
        } else if self.pipes.contains_key(&loc_0) {
            self.path.push(loc_0);
            self.direction = direct_0;
        } else if self.pipes.contains_key(&loc_1) {
            self.path.push(loc_1);
            self.direction = direct_1;

        /* Indicate that the path has ended */
        } else {
            self.direction = Complex::new(0, 0);
        };
    }

    /// Follow the path through the network and recover the letters in order
    pub fn find_path_word(&mut self) -> String {
        let mut seen_letters = Vec::new();

        while self.direction != Complex::new(0, 0) {
            let curr_loc = self.path.last().unwrap();
            let curr_sqr = self.pipes.get(&curr_loc).unwrap();

            /* Collect the letters encountered by traversing the path. */
            if curr_sqr.is_uppercase() {
                seen_letters.push(*curr_sqr);
            }
            self.step();
        }
        return seen_letters.iter().collect();
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        Network::new("./data/input.txt").find_path_word()
    );
}
