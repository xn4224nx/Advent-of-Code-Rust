/*
 * --- Day 19: An Elephant Named Joseph ---
 *
 * The Elves contact you over a highly secure emergency channel. Back at the
 * North Pole, the Elves are busy misunderstanding White Elephant parties.
 *
 * Each Elf brings a present. They all sit in a circle, numbered starting with
 * position 1. Then, starting with the first Elf, they take turns stealing all
 * the presents from the Elf to their left. An Elf with no presents is removed
 * from the circle and does not take turns.
 *
 * PART 1:  With the number of Elves given in your puzzle input, which Elf gets
 *          all the presents?
 *
 * Realizing the folly of their present-exchange rules, the Elves agree to
 * instead steal presents from the Elf directly across the circle. If two Elves
 * are across the circle, the one on the left (from the perspective of the
 * stealer) is stolen from. The other rules remain unchanged: Elves with no
 * presents are removed from the circle entirely, and the other elves move in
 * slightly to keep the circle evenly spaced.
 *
 * PART 2:  With the number of Elves given in your puzzle input, which Elf now
 *          gets all the presents?
 */

use std::cmp::max;

/// Determine the lucky elf that gets all the presents
pub fn find_final_elf(num_elves: u32) -> u32 {
    /* Convert to a binary form. */
    let num = &format!("{:b}", num_elves);

    /* Move the first bit to the end. */
    let num = [&num[1..], &num[0..1]].concat();

    /* Convert back to an integer. */
    return u32::from_str_radix(&num, 2).unwrap();
}

/// Find the lucky elf under another the cross steal system
pub fn find_final_elf_cross(num_elves: u32) -> u32 {
    let p = 3_u32.pow((num_elves - 1).ilog(3));
    return num_elves + num_elves.saturating_sub(2 * p) - p;
}

fn main() {
    println!("Part 1 = {}", find_final_elf(3018458));
    println!("Part 2 = {}", find_final_elf_cross(3018458));
}
