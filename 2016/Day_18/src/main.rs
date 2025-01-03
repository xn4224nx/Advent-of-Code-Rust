/*
 * --- Day 18: Like a Rogue ---
 *
 * As you enter this room, you hear a loud click! Some of the tiles in the floor
 * here seem to be pressure plates for traps, and the trap you just triggered
 * has run out of... whatever it tried to do to you. You doubt you'll be so
 * lucky next time.
 *
 * Upon closer examination, the traps and safe tiles in this room seem to follow
 * a pattern. The tiles are arranged into rows that are all the same width; you
 * take note of the safe tiles (.) and traps (^) in the first row (your puzzle
 * input).
 *
 * The type of tile (trapped or safe) in each row is based on the types of the
 * tiles in the same position, and to either side of that position, in the
 * previous row. (If either side is off either end of the row, it counts as
 * "safe" because there isn't a trap embedded in the wall.)
 *
 * For example, suppose you know the first row (with tiles marked by letters)
 * and want to determine the next row (with tiles marked by numbers):
 *
 *      ABCDE
 *      12345
 *
 * The type of tile 2 is based on the types of tiles A, B, and C; the type of
 * tile 5 is based on tiles D, E, and an imaginary "safe" tile. Let's call these
 * three tiles from the previous row the left, center, and right tiles,
 * respectively. Then, a new tile is a trap only in one of the following
 * situations:
 *
 *      -   Its left and center tiles are traps, but its right tile is not.
 *      -   Its center and right tiles are traps, but its left tile is not.
 *      -   Only its left tile is a trap.
 *      -   Only its right tile is a trap.
 *
 * In any other situation, the new tile is safe.
 *
 * PART 1:  Starting with the map in your puzzle input, in a total of 40 rows
 *          (including the starting row), how many safe tiles are there?
 *
 * PART 2:  How many safe tiles are there in a total of 400000 rows?
 */

use std::fs::File;
use std::io::{BufRead, BufReader};

const SAFE_TILE: char = '.';
const TRAP_TILE: char = '^';

pub struct TrapRoom {
    pub rows: Vec<Vec<bool>>,
}

impl TrapRoom {
    pub fn new(data_file: &str) -> Self {
        let mut buffer = String::new();
        let mut new_rows = Vec::new();

        /* Open the file with the starting rows. */
        let file = File::open(data_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Iterate over the file line by line. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            new_rows.push(
                buffer
                    .chars()
                    .filter(|x| *x == SAFE_TILE || *x == TRAP_TILE)
                    .map(|x| match x {
                        SAFE_TILE => true,
                        TRAP_TILE => false,
                        _ => panic!("Unknown tile type '{}'", x),
                    })
                    .collect(),
            );
            buffer.clear();
        }
        return TrapRoom { rows: new_rows };
    }

    /// Display the current trap room
    pub fn show(&self) -> String {
        let mut rep = String::new();

        for irow in self.rows.iter() {
            for tile in irow.iter() {
                if *tile {
                    rep.push(SAFE_TILE)
                } else {
                    rep.push(TRAP_TILE)
                }
            }
            rep.push('\n')
        }
        return rep;
    }

    /// Predict the next rows in the room up to a certain row
    pub fn predict_rows(&mut self, max_row: usize) {
        while self.rows.len() < max_row {
            let mut new_row = Vec::new();

            /* Generate new tiles left to right */
            for nt_idx in 0..self.rows[0].len() {
                /* Extract the tile up and to the left of the current. */
                let l_tile = if nt_idx == 0 {
                    true
                } else {
                    self.rows[self.rows.len() - 1][nt_idx - 1]
                };

                /* Extract the tile above the current one. */
                let c_tile = self.rows[self.rows.len() - 1][nt_idx];

                /* Extract the tile up and to the right of the current. */
                let r_tile = if nt_idx == self.rows[0].len() - 1 {
                    true
                } else {
                    self.rows[self.rows.len() - 1][nt_idx + 1]
                };

                /* Determine of its a trap or not. */
                new_row.push(
                    if (!l_tile && !c_tile && r_tile)
                        || (!c_tile && !r_tile && l_tile)
                        || (!l_tile && c_tile && r_tile)
                        || (l_tile && c_tile && !r_tile)
                    {
                        false
                    } else {
                        true
                    },
                );
            }

            /* Add the newly generated row to the rest */
            self.rows.push(new_row)
        }
    }

    /// Count the safe tiles in the room
    pub fn num_safe_tiles(&self) -> usize {
        let mut s_sum = 0;

        /* Count the trues in each row of tiles. */
        for t_row in self.rows.iter() {
            s_sum += t_row.iter().filter(|x| **x).count();
        }
        return s_sum;
    }
}

fn main() {
    let mut rogue = TrapRoom::new("./data/input.txt");
    rogue.predict_rows(40);
    println!("Part 1 = {}", rogue.num_safe_tiles());
    rogue.predict_rows(400000);
    println!("Part 2 = {}", rogue.num_safe_tiles());
}
