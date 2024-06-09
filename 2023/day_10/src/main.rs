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
        let x = point.0;
        let y = point.1;

        if x >= self.maze_size.0 {
            panic!("X-coord is too large!");
        } else if y >= self.maze_size.1 {
            panic!("Y-coord is too large!");
        }

        if x == 0 {
            adj_sqrs.push((1, y));
        } else if x >= self.maze_size.0 - 1 {
            adj_sqrs.push((self.maze_size.0 - 2, y));
        } else {
            adj_sqrs.push((x + 1, y));
            adj_sqrs.push((x - 1, y));
        }

        if y == 0 {
            adj_sqrs.push((x, 1));
        } else if y >= self.maze_size.1 - 1 {
            adj_sqrs.push((x, self.maze_size.1 - 2));
        } else {
            adj_sqrs.push((x, y + 1));
            adj_sqrs.push((x, y - 1));
        }

        return adj_sqrs;
    }
}

fn main() {}
