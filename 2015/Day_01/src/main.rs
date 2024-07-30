/*
 * --- Day 1: Not Quite Lisp ---
 *
 * Santa is trying to deliver presents in a large apartment
 * building, but he can't find the right floor - the directions he
 * got are a little confusing. He starts on the ground floor
 * (floor 0) and then follows the instructions one character at a
 * time.
 *
 * An opening parenthesis, (, means he should go up one floor,
 * and a closing parenthesis, ), means he should go down one
 * floor.
 *
 * The apartment building is very tall, and the basement is very
 * deep; he will never find the top or bottom floors.
 *
 * PART 1: To what floor do the instructions take Santa?
 */

use std::fs;

/// Read the file with the building directions and return them
pub fn read_building_directions(fp: &str) -> String {
    return fs::read_to_string(fp).unwrap().trim().to_string();
}

/// Find the final floor that Santa ends up on due to the building
/// directions or when he first enters the basement.
pub fn find_final_floor(directions: &String, find_basement: bool) -> i32 {
    let mut floor: i32 = 0;

    /* Interate over the instructions */
    for (idx, direct) in directions.chars().enumerate() {
        match direct {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Unknown direction!"),
        }

        /* Return the index of the instruction that first takes Santa
        into the basement. Plus one is due to differnt indexing offset for
        the answer.*/
        if find_basement && floor == -1 {
            return (idx + 1) as i32;
        }
    }
    return floor;
}

fn main() {
    let directions = read_building_directions("./data/input.txt");
    println!("Part 1 answer = {}", find_final_floor(&directions, false));
    println!("Part 1 answer = {}", find_final_floor(&directions, true));
}
