/*
 * --- Day 10: Pipe Maze ---
 *
 * Scanning the area, you discover that the entire field you're standing
 * on is densely packed with pipes; it was hard to tell at first
 * because they're the same metallic silver color as the "ground". You
 * make a quick sketch of all of the surface pipes you can see (your
 * puzzle input).
 *
 * The pipes are arranged in a two-dimensional grid of tiles. Based on
 * the acoustics of the animal's scurrying, you're confident the pipe
 * that contains the animal is one large, continuous loop.
 *
 * Unfortunately, there are also many pipes that aren't connected to the
 * loop! Every pipe in the main loop connects to its two neighbors
 * (including S, which will have exactly two pipes connecting to it, and
 * which is assumed to connect back to those two pipes).
 *
 * If you want to get out ahead of the animal, you should find the tile
 * in the loop that is farthest from the starting position. Because the
 * animal is in the pipe, it doesn't make sense to measure this by
 * direct distance. Instead, you need to find the tile that would take
 * the longest number of steps along the loop to reach from the starting
 * point - regardless of which way around the loop the animal went.
 *
 * Part 1 - Find the single giant loop starting at S. How many steps
 *          along the loop does it take to get from the starting
 *          position to the point farthest from the starting position?
 */

use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
pub struct Maze {
    pub pipes: Vec<Vec<char>>,
    pub start_pnt: (usize, usize),
    pub maze_size: (usize, usize),
}

impl Maze {
    /// Read a file with a pipe maze and populate a structure
    pub fn new(filepath: &str) -> Self {
        let mut row_idx = 0;
        let mut col_idx = 0;
        let mut start_point = (0, 0);
        let mut start_found = false;
        let mut last_col_len = 0;

        /* Read the file. */
        let file = File::open(filepath).expect("Unable to read file!");
        let reader = BufReader::new(file);
        let mut pipe_maze: Vec<Vec<char>> = Vec::new();

        /* Iterate over the file, parse the data and collect stats */
        for raw_line in reader.lines() {
            let mut row: Vec<char> = Vec::new();
            col_idx = 0;

            /* Check the line can be read otherwise skip it. */
            let Ok(clean_line) = raw_line else {
                continue;
            };

            /* Iterate over the characters in the row. */
            for row_ele in clean_line.chars() {
                if row_ele == 'S' {
                    start_point = (row_idx, col_idx);
                    start_found = true;
                }

                row.push(row_ele);
                col_idx += 1;
            }

            /* Check the current row is the same length as the last. */
            if row_idx > 0 && col_idx != last_col_len {
                panic!("The maze rows need to be consistent length!");
            }

            last_col_len = col_idx;
            pipe_maze.push(row);
            row_idx += 1;
        }

        /* Check the start point was found. */
        if !start_found {
            panic!("Start point not found in the maze!");
        }

        return Maze {
            pipes: pipe_maze,
            start_pnt: start_point,
            maze_size: (row_idx, col_idx),
        };
    }

    /// Determine the adjacent maze squares for a point
    pub fn adj_squares(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
        let mut adj_sqrs: Vec<(usize, usize)> = Vec::new();
        let y = point.0;
        let x = point.1;

        if y >= self.maze_size.0 {
            panic!("y-coord is too large!");
        } else if x >= self.maze_size.1 {
            panic!("x-coord is too large!");
        }

        if y == 0 {
            adj_sqrs.push((1, x));
        } else if y >= self.maze_size.0 - 1 {
            adj_sqrs.push((self.maze_size.0 - 2, x));
        } else {
            adj_sqrs.push((y + 1, x));
            adj_sqrs.push((y - 1, x));
        }

        if x == 0 {
            adj_sqrs.push((y, 1));
        } else if x >= self.maze_size.1 - 1 {
            adj_sqrs.push((y, self.maze_size.1 - 2));
        } else {
            adj_sqrs.push((y, x + 1));
            adj_sqrs.push((y, x - 1));
        }

        return adj_sqrs;
    }

    /// Determine the valid adjacent next steps
    pub fn next_steps(&self, curr_point: (usize, usize)) -> Vec<(usize, usize)> {
        let mut next_pnts: Vec<(usize, usize)> = Vec::new();

        /* Gather all the squares next to the current point. */
        let all_adj = &self.adj_squares(curr_point);

        /* Iterate over each adjacent point and check its validity. */
        for pnt in all_adj.iter() {
            let pnt_type = self.pipes[pnt.0][pnt.1];

            if pnt_type == '.' {
                continue;
            }
            /* Points above the current point. */
            else if (pnt_type == 'F' || pnt_type == '7' || pnt_type == '|')
                && pnt.0 < curr_point.0
            {
                next_pnts.push(*pnt);
            }
            /* Points below the current point. */
            else if (pnt_type == 'L' || pnt_type == 'J' || pnt_type == '|')
                && pnt.0 > curr_point.0
            {
                next_pnts.push(*pnt);
            }
            /* Points to the left of the current point. */
            else if (pnt_type == 'F' || pnt_type == 'L' || pnt_type == '-')
                && pnt.1 < curr_point.1
            {
                next_pnts.push(*pnt);
            }
            /* Points to the right of the current point. */
            else if (pnt_type == 'J' || pnt_type == '7' || pnt_type == '-')
                && pnt.1 > curr_point.1
            {
                next_pnts.push(*pnt);
            }
        }

        return next_pnts;
    }

    /// Retrieve the pipe that is at a certain point in the maze
    pub fn pnt_2_pipe(&self, point: (usize, usize)) -> char {
        return self.pipes[point.0][point.1];
    }

    /// Determine the next square based on the currently occupied square
    pub fn generate_maze_loop(&self) -> Vec<(usize, usize)> {
        let mut seen_points = vec![self.start_pnt];

        /* Determine the viable next step from the start point. */
        let mut pnt = self.next_steps(self.start_pnt)[0];

        loop {
            let pipe = self.pnt_2_pipe(pnt);

            /* The loop is completed when the start point is seen again. */
            if pipe == 'S' {
                break;
            }

            /* Keep a record of the points travelled. */
            seen_points.push(pnt);

            /* Determine the two possible next steps in the maze */
            let pos_next_steps = match pipe {
                '-' => vec![(pnt.0, pnt.1 + 1), (pnt.0, pnt.1 - 1)],
                '|' => vec![(pnt.0 + 1, pnt.1), (pnt.0 - 1, pnt.1)],
                'F' => vec![(pnt.0 + 1, pnt.1), (pnt.0, pnt.1 + 1)],
                'L' => vec![(pnt.0 - 1, pnt.1), (pnt.0, pnt.1 + 1)],
                'J' => vec![(pnt.0 - 1, pnt.1), (pnt.0, pnt.1 - 1)],
                '7' => vec![(pnt.0 + 1, pnt.1), (pnt.0, pnt.1 - 1)],
                _ => panic!("Unknown pipe encountered!"),
            };

            /* Work out which of the two points was the previous one. */
            if seen_points[seen_points.len() - 2] == pos_next_steps[0] {
                pnt = pos_next_steps[1]
            } else {
                pnt = pos_next_steps[0]
            }
        }
        return seen_points;
    }
}

fn main() {
    let pmaze = Maze::new("./data/input.txt");
    let maze_loop = pmaze.generate_maze_loop();

    println!("Part 1 answer = {}", maze_loop.len() / 2);
}
