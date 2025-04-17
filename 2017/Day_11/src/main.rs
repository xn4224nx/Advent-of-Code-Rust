/*
 * --- Day 11: Hex Ed ---
 *
 * Crossing the bridge, you've barely reached the other side of the stream when
 * a program comes up to you, clearly in distress. "It's my child process," she
 * says, "he's gotten lost in an infinite grid!"
 *
 * Fortunately for her, you have plenty of experience with infinite grids.
 *
 * Unfortunately for you, it's a hex grid.
 *
 * The hexagons ("hexes") in this grid are aligned such that adjacent hexes can
 * be found to the north, northeast, southeast, south, southwest, and northwest:
 *
 *   \ n  /
 * nw +--+ ne
 *   /    \
 * -+      +-
 *   \    /
 * sw +--+ se
 *   / s  \
 *
 * You have the path the child process took. Starting where he started, you need
 * to determine the fewest number of steps required to reach him. (A "step"
 * means to move from the hex you are in to any adjacent hex.)
 *
 * For example:
 *
 *      -   ne,ne,ne is 3 steps away.
 *      -   ne,ne,sw,sw is 0 steps away (back where you started).
 *      -   ne,ne,s,s is 2 steps away (se,se).
 *      -   se,sw,se,sw,sw is 3 steps away (s,s,sw).
 *
 * PART 1:  Determine the fewest number of steps required to reach the child
 *          process.
 */

use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
pub enum Dir {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
}

pub struct HexGrid {
    pub path: Vec<Dir>,
}

impl HexGrid {
    pub fn new(path_file: &str) -> Self {
        HexGrid {
            path: read_to_string(path_file)
                .unwrap()
                .split(",")
                .map(|x| match x.trim() {
                    "n" => Dir::North,
                    "ne" => Dir::NorthEast,
                    "nw" => Dir::NorthWest,
                    "s" => Dir::South,
                    "se" => Dir::SouthEast,
                    "sw" => Dir::SouthWest,
                    _ => panic!("'{x}' is not a recognised direction"),
                })
                .collect(),
        }
    }

    /// Determine the distance from the centre the path would take
    pub fn distance(self) -> u32 {
        let mut sum_n_dir: i32 = 0;
        let mut sum_se_dir: i32 = 0;

        for direc in self.path {
            match direc {
                Dir::North => {
                    sum_n_dir += 1;
                }
                Dir::NorthEast => {
                    sum_n_dir += 1;
                    sum_se_dir += 1;
                }
                Dir::NorthWest => {
                    sum_se_dir -= 1;
                }
                Dir::South => {
                    sum_n_dir -= 1;
                }
                Dir::SouthEast => {
                    sum_se_dir += 1;
                }
                Dir::SouthWest => {
                    sum_n_dir -= 1;
                    sum_se_dir -= 1;
                }
            }
        }
        return ((sum_n_dir.abs() + sum_se_dir.abs() + (sum_se_dir - sum_n_dir).abs()) / 2) as u32;
    }
}

fn main() {
    println!("Part 1 = {}", HexGrid::new("./data/input.txt").distance())
}
