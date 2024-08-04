/*
 * --- Day 3: Perfectly Spherical Houses in a Vacuum ---
 *
 * Santa is delivering presents to an infinite two-dimensional
 * grid of houses.
 *
 * He begins by delivering a present to the house at his
 * starting location, and then an elf at the North Pole calls
 * him via radio and tells him where to move next. Moves are
 * always exactly one house to the north (^), south (v), east
 * (>), or west (<). After each move, he delivers another
 * present to the house at his new location.
 *
 * However, the elf back at the north pole has had a little too
 * much eggnog, and so his directions are a little off, and
 * Santa ends up visiting some houses more than once.
 *
 * PART 1: How many houses receive at least one present?
 *
 * The next year, to speed up the process, Santa creates a robot
 * version of himself, Robo-Santa, to deliver presents with him.
 *
 * Santa and Robo-Santa start at the same location (delivering two
 * presents to the same starting house), then take turns moving
 * based on instructions from the elf, who is eggnoggedly reading
 * from the same script as the previous year.
 *
 * PART 2:  This year, how many houses receive at least one present?
 */

use std::collections::HashSet;
use std::fs;

#[derive(PartialEq, Debug)]
pub enum GridDir {
    North,
    South,
    West,
    East,
}

/// Read a text file and parse into grid directions
pub fn read_directions(file_path: &str) -> Vec<GridDir> {
    let mut parsed_dirs = Vec::new();

    /* Open the file and put everything into a String. */
    let file_contents = fs::read_to_string(file_path).unwrap();

    /* Iterate over every char and parse the direction */
    for f_char in file_contents.chars() {
        match f_char {
            '^' => parsed_dirs.push(GridDir::North),
            'v' => parsed_dirs.push(GridDir::South),
            '<' => parsed_dirs.push(GridDir::West),
            '>' => parsed_dirs.push(GridDir::East),
            _ => continue,
        }
    }

    return parsed_dirs;
}

/// Count the number of houses visited by Santa at least
/// once.
pub fn count_visited_houses(directions: &Vec<GridDir>, robot: bool) -> usize {
    let mut visited_houses = HashSet::new();

    /* Santa & Robot Santa always start at the same location. */
    let mut s_loc = (0, 0);
    let mut r_loc = s_loc;

    /* The starting house is always visited. */
    visited_houses.insert(s_loc);

    /* For each direction move the location then save it*/
    for (idx, dir) in directions.iter().enumerate() {
        /* Robot Santa moves on even directions */
        if robot && idx % 2 == 0 {
            move_santa(&mut r_loc, dir);
            visited_houses.insert(r_loc);

        /* Santa moves otherwise */
        } else {
            move_santa(&mut s_loc, dir);
            visited_houses.insert(s_loc);
        }
    }

    /* Only the unique number of houses visited matters. */
    return visited_houses.len();
}

/// Move Santa or his robot counterpart based on a direction
fn move_santa(c_loc: &mut (i32, i32), dir: &GridDir) {
    match dir {
        GridDir::North => *c_loc = (c_loc.0, c_loc.1 + 1),
        GridDir::South => *c_loc = (c_loc.0, c_loc.1 - 1),
        GridDir::West => *c_loc = (c_loc.0 - 1, c_loc.1),
        GridDir::East => *c_loc = (c_loc.0 + 1, c_loc.1),
    };
}

fn main() {
    let dirs = read_directions("./data/input.txt");
    println!(
        "The answer to part 1 = {}",
        count_visited_houses(&dirs, false)
    );
    println!(
        "The answer to part 1 = {}",
        count_visited_houses(&dirs, true)
    );
}
