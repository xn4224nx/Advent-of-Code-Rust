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
 */

pub struct Sculpture {
    pub disk_total_positions: Vec<usize>,
    pub disk_start_positions: Vec<usize>,
}

impl Sculpture {
    pub fn new(file_path: &str) -> Self {
        return Sculpture {
            disk_total_positions: Vec::new(),
            disk_start_positions: Vec::new(),
        };
    }

    pub fn can_drop_happen(&self, time: usize) -> bool {
        false
    }

    pub fn find_first_drop_time(&self) -> usize {
        0
    }
}

fn main() {}
