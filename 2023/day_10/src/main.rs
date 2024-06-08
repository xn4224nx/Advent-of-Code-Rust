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

/// Read a pipe maze file into a machine readable format
pub fn read_pipe_maze(filepath: &str) -> Vec<Vec<char>> {
    let file = File::open(filepath).expect("Unable to read file!");
    let reader = BufReader::new(file);

    let mut pipe_maze: Vec<Vec<char>> = Vec::new();

    for raw_line in reader.lines() {
        /* Check the line can be read otherwise skip it. */
        let Ok(clean_line) = raw_line else {
            continue;
        };

        pipe_maze.push(clean_line.chars().collect());
    }

    return pipe_maze;
}

/// Find the starting location of the maze
pub fn find_start_coord(pipe_maze: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in pipe_maze.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val == 'L' {
                return (y, x);
            }
        }
    }

    panic!("Start point not found in the maze!");
}

fn main() {}
