/*
 * --- Day 15: Timing is Everything ---
 *
 * The halls open into an interior plaza containing a large kinetic sculpture.
 * The sculpture is in a sealed enclosure and seems to involve a set of
 * identical spherical capsules that are carried to the top and allowed to
 * bounce through the maze of spinning pieces.
 *
 * Part of the sculpture is even interactive! When a button is pressed, a
 * capsule is dropped and tries to fall through slots in a set of rotating discs
 * to finally go through a little hole at the bottom and come out of the
 * sculpture. If any of the slots aren't aligned with the capsule as it passes,
 * the capsule bounces off the disc and soars away. You feel compelled to get
 * one of those capsules.
 *
 * The discs pause their motion each second and come in different sizes; they
 * seem to each have a fixed number of positions at which they stop. You decide
 * to call the position with the slot 0, and count up for each position it
 * reaches next.
 *
 * Furthermore, the discs are spaced out so that after you push the button, one
 * second elapses before the first disc is reached, and one second elapses as
 * the capsule passes from one disc to the one below it. So, if you push the
 * button at time=100, then the capsule reaches the top disc at time=101, the
 * second disc at time=102, the third disc at time=103, and so on.
 *
 * The button will only drop a capsule at an integer time - no fractional
 * seconds allowed.
 *
 * PART 1:  However, your situation has more than two discs; you've noted their
 *          positions in your puzzle input. What is the first time you can press
 *          the button to get a capsule?
 *
 * After getting the first capsule (it contained a star! what great fortune!),
 * the machine detects your success and begins to rearrange itself.
 *
 * When it's done, the discs are back in their original configuration as if it
 * were time=0 again, but a new disc with 11 positions and starting at position
 * 0 has appeared exactly one second below the previously-bottom disc.
 *
 * PART 2:  With this new disc, and counting again starting from time=0 with the
 *          configuration in your puzzle input, what is the first time you can
 *          press the button to get another capsule?
 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Sculpture {
    pub dsk_total_pos: Vec<usize>,
    pub dsk_start_pos: Vec<usize>,
}

impl Sculpture {
    pub fn new(file_path: &str, extra_disk: bool) -> Self {
        let mut buffer = String::new();
        let mut dks_total: Vec<usize> = Vec::new();
        let mut dsk_start: Vec<usize> = Vec::new();

        let re_nums = Regex::new(r"\d+").unwrap();

        /* Open the datafile. */
        let file = File::open(file_path).unwrap();
        let mut fp = BufReader::new(file);

        /* Iterate over the file line by line. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            /* Extract the numbers from the line. */
            let nums: Vec<usize> = re_nums
                .find_iter(&buffer)
                .map(|x| x.as_str().parse::<usize>().unwrap())
                .collect();

            /* Save only the ones we require. */
            dks_total.push(nums[1]);
            dsk_start.push(nums[3]);

            /* Prepare to read the next line. */
            buffer.clear();
        }

        /* Account for an extra disk being added at the end. */
        if extra_disk {
            dks_total.push(11);
            dsk_start.push(0);
        }

        return Sculpture {
            dsk_total_pos: dks_total,
            dsk_start_pos: dsk_start,
        };
    }

    /// Determine if a coin was dropped at a particular time if it would fall
    /// through all the sculptures.
    pub fn can_drop_happen(&self, time: usize) -> bool {
        for idx in 0..self.dsk_total_pos.len() {
            if (self.dsk_start_pos[idx] + time + idx + 1) % self.dsk_total_pos[idx] != 0 {
                return false;
            }
        }
        return true;
    }

    pub fn find_first_drop_time(&self) -> usize {
        let mut time = 0;

        loop {
            if self.can_drop_happen(time) {
                return time;
            }
            time += 1;
        }
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        Sculpture::new("./data/input.txt", false).find_first_drop_time()
    );
    println!(
        "Part 2 = {}",
        Sculpture::new("./data/input.txt", true).find_first_drop_time()
    );
}
