/*
 * --- Day 20: Particle Swarm ---
 *
 * Suddenly, the GPU contacts you, asking for help. Someone has asked it to
 * simulate too many particles, and it won't be able to finish them all in time
 * to render the next frame at this rate.
 *
 * It transmits to you a buffer (your puzzle input) listing each particle in
 * order (starting with particle 0, then particle 1, particle 2, and so on). For
 * each particle, it provides the X, Y, and Z coordinates for the particle's
 * position (p), velocity (v), and acceleration (a), each in the format <X,Y,Z>.
 *
 * Each tick, all particles are updated simultaneously. A particle's properties
 * are updated in the following order:
 *
 *      -   Increase the X velocity by the X acceleration.
 *      -   Increase the Y velocity by the Y acceleration.
 *      -   Increase the Z velocity by the Z acceleration.
 *      -   Increase the X position by the X velocity.
 *      -   Increase the Y position by the Y velocity.
 *      -   Increase the Z position by the Z velocity.
 *
 * Because of seemingly tenuous rationale involving z-buffering, the GPU would
 * like to know which particle will stay closest to position <0,0,0> in the long
 * term. Measure this using the Manhattan distance, which in this situation is
 * simply the sum of the absolute values of a particle's X, Y, and Z position.
 *
 * For example, suppose you are only given two particles, both of which stay
 * entirely on the X-axis (for simplicity). Drawing the current states of
 * particles 0 and 1 (in that order) with an adjacent a number line and diagram
 * of current X positions (marked in parentheses), the following would take
 * place:
 *
 * p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
 * p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>                         (0)(1)
 *
 * p=< 4,0,0>, v=< 1,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
 * p=< 2,0,0>, v=<-2,0,0>, a=<-2,0,0>                      (1)   (0)
 *
 * p=< 4,0,0>, v=< 0,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
 * p=<-2,0,0>, v=<-4,0,0>, a=<-2,0,0>          (1)               (0)
 *
 * p=< 3,0,0>, v=<-1,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
 * p=<-8,0,0>, v=<-6,0,0>, a=<-2,0,0>                         (0)
 *
 * At this point, particle 1 will never be closer to <0,0,0> than particle 0,
 * and so, in the long run, particle 0 will stay closest.
 *
 * PART 1:  Which particle will stay closest to position <0,0,0> in the long
 *          term?
 *
 * To simplify the problem further, the GPU would like to remove any particles
 * that collide. Particles collide if their positions ever exactly match.
 * Because particles are updated simultaneously, more than two particles can
 * collide at the same time and place. Once particles collide, they are removed
 * and cannot collide with anything else after that tick.
 *
 * PART 2:  How many particles are left after all collisions are resolved?
 */

use itertools::Itertools;
use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
pub struct Particle {
    pub position: Vec<i32>,
    pub velocity: Vec<i32>,
    pub acceleration: Vec<i32>,
}

#[derive(Debug, PartialEq)]
pub struct Throng {
    pub contents: Vec<Particle>,
}

impl Particle {
    pub fn new(raw_details: &str) -> Self {
        let re_num = Regex::new(r"-?[0-9]+").unwrap();
        let raw_nums: Vec<i32> = re_num
            .find_iter(raw_details)
            .map(|x| x.as_str().parse::<i32>().unwrap())
            .collect();

        return Particle {
            position: vec![raw_nums[0], raw_nums[1], raw_nums[2]],
            velocity: vec![raw_nums[3], raw_nums[4], raw_nums[5]],
            acceleration: vec![raw_nums[6], raw_nums[7], raw_nums[8]],
        };
    }

    pub fn step(&mut self) {
        for idx in 0..self.velocity.len() {
            self.velocity[idx] += self.acceleration[idx];
            self.position[idx] += self.velocity[idx]
        }
    }

    pub fn dist_from_origin(&self) -> i32 {
        return self.position.iter().map(|x| x.abs()).sum::<i32>();
    }
}

impl Throng {
    pub fn new(data_file: &str) -> Self {
        Throng {
            contents: read_to_string(data_file)
                .unwrap()
                .lines()
                .map(|x| Particle::new(x))
                .collect(),
        }
    }

    pub fn long_term_closest_to_origin(&mut self) -> usize {
        /* Advance the particles. */
        for _ in 0..1000 {
            for p_idx in 0..self.contents.len() {
                self.contents[p_idx].step()
            }
        }

        /* Find the particle closest to the origin. */
        let mut min_val = i32::MAX;
        let mut min_idx = 0;
        for (p_idx, dist) in self
            .contents
            .iter()
            .map(|x| x.dist_from_origin())
            .enumerate()
        {
            if dist < min_val {
                min_val = dist;
                min_idx = p_idx;
            }
        }
        return min_idx;
    }

    pub fn remaining_particles(&mut self) -> usize {
        let mut collided = vec![false; self.contents.len()];

        /* Advance the particles and determine ones that have collided. */
        for _ in 0..1000 {
            for p_idx in 0..self.contents.len() {
                if !collided[p_idx] {
                    self.contents[p_idx].step();
                }
            }

            /* Find the indexes of uncollided particles. */
            let uncollided_idxs: Vec<usize> = collided
                .iter()
                .enumerate()
                .filter(|x| !x.1)
                .map(|x| x.0)
                .collect();

            /* Check for collisions amoung uncollided particles. */
            for part_idxs in uncollided_idxs.into_iter().combinations(2) {
                if self.contents[part_idxs[0]].position == self.contents[part_idxs[1]].position {
                    collided[part_idxs[0]] = true;
                    collided[part_idxs[1]] = true;
                }
            }
        }
        return collided.into_iter().filter(|x| !x).count();
    }
}

fn main() {
    println!(
        "Part 1 = {}\nPart 2 = {}\n",
        Throng::new("./data/input.txt").long_term_closest_to_origin(),
        Throng::new("./data/input.txt").remaining_particles()
    );
}
