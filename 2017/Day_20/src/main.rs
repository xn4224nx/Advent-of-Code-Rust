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
 */

use regex::Regex;
use std::cmp::Ordering;
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

    pub fn dist_from_origin(&mut self) -> i32 {
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
        for _ in 0..1000 {
            for p_idx in 0..self.contents.len() {
                self.contents[p_idx].step()
            }
        }

        /* Find the particle closest to the origin. */
        return self
            .contents
            .iter_mut()
            .map(|x| x.dist_from_origin())
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap();
    }
}

fn main() {}
