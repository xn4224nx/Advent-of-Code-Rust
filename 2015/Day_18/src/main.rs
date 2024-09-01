/*
 * --- Day 18: Like a GIF For Your Yard ---
 *
 * After the million lights incident, the fire code has gotten stricter: now, at
 * most ten thousand lights are allowed. You arrange them in a 100x100 grid.
 *
 * Never one to let you down, Santa again mails you instructions on the ideal
 * lighting configuration. With so few lights, he says, you'll have to resort to
 * animation.
 *
 * Start by setting your lights to the included initial configuration (your
 * puzzle input). A # means "on", and a . means "off".
 *
 * Then, animate your grid in steps, where each step decides the next
 * configuration based on the current one. Each light's next state (either on or
 * off) depends on its current state and the current states of the eight lights
 * adjacent to it (including diagonals). Lights on the edge of the grid might
 * have fewer than eight neighbors; the missing ones always count as "off".
 *
 * The state a light should have next is based on its current state (on or off)
 * plus the number of neighbors that are on:
 *
 *      -   A light which is on stays on when 2 or 3 neighbors are on, and turns
 *          off otherwise.
 *
 *      -   A light which is off turns on if exactly 3 neighbors are on, and
 *          stays off otherwise.
 *
 * All of the lights update simultaneously; they all consider the same current
 * state before moving to the next.
 *
 * PART 1:  In your grid of 100x100 lights, given your initial configuration, how
 *          many lights are on after 100 steps?
 */

use ndarray::{arr2, Array2};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub fn read_light_grid(light_file: &str) -> Array2<bool> {
    let mut raw: Vec<bool> = Vec::new();

    /* Open the file */
    let file = File::open(light_file).unwrap();
    let mut fp = BufReader::new(file);

    /* Iterate over the file line by line. */
    let mut buffer = String::new();
    let mut line_idx = 0;
    while fp.read_line(&mut buffer).unwrap() > 0 {
        line_idx += 1;

        /* Parse the line as either true or falses, ignoring everything else. */
        for li_char in buffer.chars() {
            match li_char {
                '.' => raw.push(false),
                '#' => raw.push(true),
                _ => (),
            }
        }
        buffer.clear();
    }

    /* Convert the read data into a ndarray. */
    return Array2::from_shape_vec((line_idx, raw.len() / line_idx), raw).unwrap();
}

pub fn find_adj_lights(light: &Point, grid_size: &[usize]) -> Vec<Point> {
    let mut adj_points = Vec::new();

    /* Points above the current one. */
    if light.y > 0 {
        adj_points.push(Point {
            x: light.x,
            y: light.y - 1,
        });

        /* Points to the left. */
        if light.x > 0 {
            adj_points.push(Point {
                x: light.x - 1,
                y: light.y - 1,
            });
        }

        /* Points to the right. */
        if light.x < grid_size[1] - 1 {
            adj_points.push(Point {
                x: light.x + 1,
                y: light.y - 1,
            });
        }
    }

    /* Points to the left. */
    if light.x > 0 {
        adj_points.push(Point {
            x: light.x - 1,
            y: light.y,
        });
    }

    /* Points to the right. */
    if light.x < grid_size[1] - 1 {
        adj_points.push(Point {
            x: light.x + 1,
            y: light.y,
        });
    }

    /* Points below the current one. */
    if light.y < grid_size[0] - 1 {
        adj_points.push(Point {
            x: light.x,
            y: light.y + 1,
        });

        /* Points to the left. */
        if light.x > 0 {
            adj_points.push(Point {
                x: light.x - 1,
                y: light.y + 1,
            });
        }

        /* Points to the right. */
        if light.x < grid_size[1] - 1 {
            adj_points.push(Point {
                x: light.x + 1,
                y: light.y + 1,
            });
        }
    }

    return adj_points;
}

pub fn new_light_state(light_grid: &Array2<bool>, light: &Point) -> bool {
    false
}

pub fn incre_light_grid(light_grid: &Array2<bool>, steps: usize) -> Array2<bool> {
    arr2(&[
        [false, false, false],
        [false, false, false],
        [false, false, false],
    ])
}

fn main() {}
