/*
 * --- Day 6: Chronal Coordinates ---
 *
 * The device on your wrist beeps several times, and once again you feel like
 * you're falling.
 *
 * "Situation critical," the device announces. "Destination indeterminate.
 * Chronal interference detected. Please specify new target coordinates."
 *
 * The device then produces a list of coordinates (your puzzle input). Are they
 * places it thinks are safe or dangerous? It recommends you check manual page
 * 729. The Elves did not give you a manual.
 *
 * If they're dangerous, maybe you can minimize the danger by finding the
 * coordinate that gives the largest distance from the other points.
 *
 * Using only the Manhattan distance, determine the area around each coordinate
 * by counting the number of integer X,Y locations that are closest to that
 * coordinate (and aren't tied in distance to any other coordinate).
 *
 * Your goal is to find the size of the largest area that isn't infinite. For
 * example, consider the following list of coordinates:
 *
 *      1, 1
 *      1, 6
 *      8, 3
 *      3, 4
 *      5, 5
 *      8, 9
 *
 * If we name these coordinates A through F, we can draw them on a grid,
 * putting 0,0 at the top left:
 *
 *      ..........
 *      .A........
 *      ..........
 *      ........C.
 *      ...D......
 *      .....E....
 *      .B........
 *      ..........
 *      ..........
 *      ........F.
 *
 * This view is partial - the actual grid extends infinitely in all directions.
 * Using the Manhattan distance, each location's closest coordinate can be
 * determined, shown here in lowercase:
 *
 *      aaaaa.cccc
 *      aAaaa.cccc
 *      aaaddecccc
 *      aadddeccCc
 *      ..dDdeeccc
 *      bb.deEeecc
 *      bBb.eeee..
 *      bbb.eeefff
 *      bbb.eeffff
 *      bbb.ffffFf
 *
 * Locations shown as . are equally far from two or more coordinates, and so
 * they don't count as being closest to any.
 *
 * In this example, the areas of coordinates A, B, C, and F are infinite -
 * while not shown here, their areas extend forever outside the visible grid.
 * However, the areas of coordinates D and E are finite: D is closest to 9
 * locations, and E is closest to 17 (both including the coordinate's location
 * itself). Therefore, in this example, the size of the largest area is 17.
 *
 * PART 1:  What is the size of the largest area that isn't infinite?
 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct MineField {
    pub mine_coords: Vec<(usize, usize)>,
    pub top_left_corner: (usize, usize),
    pub bottom_right_corner: (usize, usize),
}

impl MineField {
    pub fn new(mine_coords_file: &str) -> Self {
        let pnt_re = Regex::new(r"(\d+), (\d+)").unwrap();
        let mut buffer = String::new();
        let mut mine_coords = Vec::new();
        let mut top_left_corner = (usize::MAX, usize::MAX);
        let mut bottom_right_corner = (0, 0);

        /* Open the coordinates file */
        let file = File::open(mine_coords_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Read the file line by line. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            let Some(raw_coord) = pnt_re.captures(&buffer) else {
                buffer.clear();
                continue;
            };

            let coord = (
                raw_coord[1].parse::<usize>().unwrap(),
                raw_coord[2].parse::<usize>().unwrap(),
            );

            /* Determine the reach of the mine field. */
            if coord.0 > bottom_right_corner.0 {
                bottom_right_corner.0 = coord.0;
            }
            if coord.1 > bottom_right_corner.1 {
                bottom_right_corner.1 = coord.1;
            }
            if coord.0 < top_left_corner.0 {
                top_left_corner.0 = coord.0;
            }
            if coord.1 < top_left_corner.1 {
                top_left_corner.1 = coord.1;
            }
            mine_coords.push(coord);
            buffer.clear();
        }

        MineField {
            mine_coords,
            top_left_corner,
            bottom_right_corner,
        }
    }

    pub fn largest_connected_area(self) -> usize {
        let mut assigned_areas = vec![0; self.mine_coords.len()];
        let mut inf_areas = vec![false; self.mine_coords.len()];

        /* Iterate over every coordinate and determine the closest mine. */
        for x in self.top_left_corner.0..=self.bottom_right_corner.0 {
            for y in self.top_left_corner.1..=self.bottom_right_corner.1 {
                let mut closest_dist = usize::MAX;
                let mut closest_mine = 0;
                let mut dist_tie = false;

                for m_idx in 0..self.mine_coords.len() {
                    let dist = self.mine_coords[m_idx].0.abs_diff(x)
                        + self.mine_coords[m_idx].1.abs_diff(y);

                    if dist < closest_dist {
                        closest_dist = dist;
                        closest_mine = m_idx;
                        dist_tie = false;
                    } else if dist == closest_dist {
                        dist_tie = true;
                    }
                }

                /* Points equally close to multiple mines don't count. */
                if dist_tie {
                    continue;
                };

                /* An edge point means an infinite area for the mine. */
                if x == self.top_left_corner.0
                    || x == self.bottom_right_corner.0
                    || y == self.top_left_corner.1
                    || y == self.bottom_right_corner.1
                {
                    inf_areas[closest_mine] = true;
                }

                assigned_areas[closest_mine] += 1;
            }
        }

        /* Return the largest area that is not associated with an infinte mine. */
        let mut max_area = 0;
        for m_idx in 0..self.mine_coords.len() {
            if !inf_areas[m_idx] && assigned_areas[m_idx] > max_area {
                max_area = assigned_areas[m_idx];
            }
        }
        return max_area;
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        MineField::new("./data/input_0.txt").largest_connected_area()
    );
}
