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

pub fn read_light_grid(light_file: &str) -> Array2<bool> {
    arr2(&[[false, false], [false, false]])
}

pub fn find_adj_lights(light: (usize, usize), grid_size: (usize, usize)) -> Vec<(usize, usize)> {
    Vec::new()
}

pub fn new_light_state(light: (usize, usize)) -> bool {
    false
}

pub fn incre_light_grid(light_grid: Array2<bool>, steps: usize) -> Array2<bool> {
    arr2(&[[false, false], [false, false]])
}

fn main() {}
