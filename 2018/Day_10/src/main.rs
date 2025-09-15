/*
 * --- Day 10: The Stars Align ---
 *
 * It's no use; your navigation system simply isn't capable of providing walking
 * directions in the arctic circle, and certainly not in 1018.
 *
 * The Elves suggest an alternative. In times like these, North Pole rescue
 * operations will arrange points of light in the sky to guide missing Elves
 * back to base. Unfortunately, the message is easy to miss: the points move
 * slowly enough that it takes hours to align them, but have so much momentum
 * that they only stay aligned for a second. If you blink at the wrong time, it
 * might be hours before another message appears.
 *
 * You can see these points of light floating in the distance, and record their
 * position in the sky and their velocity, the relative change in position per
 * second (your puzzle input). The coordinates are all given from your
 * perspective; given enough time, those positions and velocities will move the
 * points into a cohesive message!
 *
 * Rather than wait, you decide to fast-forward the process and calculate what
 * the points will eventually spell.
 *
 * For example, suppose you note the following points:
 *
 *      position=< 9,  1> velocity=< 0,  2>
 *      position=< 7,  0> velocity=<-1,  0>
 *      position=< 3, -2> velocity=<-1,  1>
 *      position=< 6, 10> velocity=<-2, -1>
 *      position=< 2, -4> velocity=< 2,  2>
 *      position=<-6, 10> velocity=< 2, -2>
 *      position=< 1,  8> velocity=< 1, -1>
 *      position=< 1,  7> velocity=< 1,  0>
 *      position=<-3, 11> velocity=< 1, -2>
 *      position=< 7,  6> velocity=<-1, -1>
 *      position=<-2,  3> velocity=< 1,  0>
 *      position=<-4,  3> velocity=< 2,  0>
 *      position=<10, -3> velocity=<-1,  1>
 *      position=< 5, 11> velocity=< 1, -2>
 *      position=< 4,  7> velocity=< 0, -1>
 *      position=< 8, -2> velocity=< 0,  1>
 *      position=<15,  0> velocity=<-2,  0>
 *      position=< 1,  6> velocity=< 1,  0>
 *      position=< 8,  9> velocity=< 0, -1>
 *      position=< 3,  3> velocity=<-1,  1>
 *      position=< 0,  5> velocity=< 0, -1>
 *      position=<-2,  2> velocity=< 2,  0>
 *      position=< 5, -2> velocity=< 1,  2>
 *      position=< 1,  4> velocity=< 2,  1>
 *      position=<-2,  7> velocity=< 2, -2>
 *      position=< 3,  6> velocity=<-1, -1>
 *      position=< 5,  0> velocity=< 1,  0>
 *      position=<-6,  0> velocity=< 2,  0>
 *      position=< 5,  9> velocity=< 1, -2>
 *      position=<14,  7> velocity=<-2,  0>
 *      position=<-3,  6> velocity=< 2, -1>
 *
 * Each line represents one point. Positions are given as <X, Y> pairs: X
 * represents how far left (negative) or right (positive) the point appears,
 * while Y represents how far up (negative) or down (positive) the point
 * appears.
 *
 * At 0 seconds, each point has the position given. Each second, each point's
 * velocity is added to its position. So, a point with velocity <1, -2> is
 * moving to the right, but is moving upward twice as quickly. If this point's
 * initial position were <3, 9>, after 3 seconds, its position would become
 * <6, 3>.
 *
 * After 3 seconds, the message appeared briefly: HI. Of course, your message
 * will be much longer and will take many more seconds to appear.
 *
 * PART 1:  What message will eventually appear in the sky?
 *
 * Good thing you didn't have to wait, because that would have taken a long
 * time - much longer than the 3 seconds in the example above.
 *
 * PART 2:  Impressed by your sub-hour communication capabilities, the Elves are
 *          curious: exactly how many seconds would they have needed to wait for
 *          that message to appear?
 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct NightSky {
    pub star_position: Vec<(i32, i32)>,
    pub star_velocity: Vec<(i32, i32)>,
}

impl NightSky {
    pub fn new(data_file: &str) -> Self {
        let mut buffer = String::new();
        let mut star_position = Vec::new();
        let mut star_velocity = Vec::new();
        let re_num = Regex::new(r"\-?\d+").unwrap();

        /* Open the datafile. */
        let file = File::open(data_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Each line will describe one star. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            let nums = re_num
                .captures_iter(&buffer)
                .map(|x| x[0].parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            /* Extract the star position and velocity. */
            star_position.push((nums[0], nums[1]));
            star_velocity.push((nums[2], nums[3]));
            buffer.clear();
        }

        return NightSky {
            star_position,
            star_velocity,
        };
    }

    /// Calculate a measure of how close all the stars are to each other at a
    /// specific point int time
    pub fn star_cluster_size(&self, time: i32) -> (i32, i32, i32, i32) {
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;

        /* Find the min and max x & y values for stars  */
        for star_idx in 0..self.star_position.len() {
            let x_pos = self.star_position[star_idx].0 + time * self.star_velocity[star_idx].0;
            let y_pos = self.star_position[star_idx].1 + time * self.star_velocity[star_idx].1;

            if x_pos > x_max {
                x_max = x_pos;
            }

            if x_pos < x_min {
                x_min = x_pos;
            }

            if y_pos > y_max {
                y_max = y_pos;
            }

            if y_pos < y_min {
                y_min = y_pos;
            }
        }

        return (x_min, x_max, y_min, y_max);
    }

    /// Create a visual representation of the night sky at a point in time.
    pub fn show(&self, time: i32) -> String {
        let (min_x, max_x, min_y, max_y) = self.star_cluster_size(time);
        let mut nsky = vec![vec!['.'; (1 + max_x - min_x) as usize]; (1 + max_y - min_y) as usize];

        /* Add the stars into the empty space. */
        for star_idx in 0..self.star_position.len() {
            let x = self.star_position[star_idx].0 + time * self.star_velocity[star_idx].0 - min_x;
            let y = self.star_position[star_idx].1 + time * self.star_velocity[star_idx].1 - min_y;
            nsky[y as usize][x as usize] = '#';
        }

        /* Convert the vector to a string. */
        let mut out_n_sky = String::new();

        for row in 0..(1 + max_y - min_y) {
            for col in 0..(1 + max_x - min_x) {
                out_n_sky.push(nsky[row as usize][col as usize]);
            }
            out_n_sky.push('\n');
        }

        return out_n_sky;
    }

    /// Show what the night sky looks like when its stars are closest together.
    /// Then return the time it takes for that pattern to appear.
    pub fn message(&self) -> i32 {
        let mut time_idx = 0;
        let mut star_spread = i32::MAX - 1;
        let mut old_spread = i32::MAX;

        /* Find the time index when the stars are closest together. */
        while star_spread < old_spread {
            old_spread = star_spread;
            let rn = self.star_cluster_size(time_idx);
            star_spread = rn.1 - rn.0 + rn.3 - rn.2;
            time_idx += 1;
        }
        time_idx -= 2;

        /* Show what message the stars form. */
        self.show(time_idx);
        return time_idx;
    }
}

fn main() {}
