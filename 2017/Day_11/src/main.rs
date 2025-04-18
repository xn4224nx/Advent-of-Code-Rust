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
 *
 * PART 2:  How many steps away is the furthest he ever got from his starting
 *          position?
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
    pub north_displace: i32,
    pub sou_eas_displace: i32,
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
            north_displace: 0,
            sou_eas_displace: 0,
        }
    }

    /// Using the displacement to the North and to the South East to calculate
    /// the distance the path has taken from the center of the hex grid.
    pub fn distance_from_origin(&self) -> u32 {
        return ((self.north_displace.abs()
            + self.sou_eas_displace.abs()
            + (self.sou_eas_displace - self.north_displace).abs())
            / 2) as u32;
    }

    /// Determine the distance from the centre the path would take
    pub fn max_and_final_distance(&mut self) -> (u32, u32) {
        let mut max_dist: u32 = 0;

        /* Follow the path in the hex grid and calculate the distance each time. */
        for direc in &self.path {
            match direc {
                Dir::North => {
                    self.north_displace += 1;
                }
                Dir::NorthEast => {
                    self.north_displace += 1;
                    self.sou_eas_displace += 1;
                }
                Dir::NorthWest => {
                    self.sou_eas_displace -= 1;
                }
                Dir::South => {
                    self.north_displace -= 1;
                }
                Dir::SouthEast => {
                    self.sou_eas_displace += 1;
                }
                Dir::SouthWest => {
                    self.north_displace -= 1;
                    self.sou_eas_displace -= 1;
                }
            };

            /* Test to see if the path has reached a point further that any other. */
            let curr_dist = self.distance_from_origin();
            if curr_dist > max_dist {
                max_dist = curr_dist;
            };
        }
        return (max_dist, self.distance_from_origin());
    }
}

fn main() {
    let (m_dist, f_dist) = HexGrid::new("./data/input.txt").max_and_final_distance();
    println!("Part 1 = {}\nPart 2 = {}\n", f_dist, m_dist);
}
