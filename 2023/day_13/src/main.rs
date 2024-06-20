/*
 * --- Day 13: Point of Incidence ---
 *
 * After a while, you make your way to a nearby cluster of mountains
 * only to discover that the valley between them is completely full of
 * large mirrors. Most of the mirrors seem to be aligned in a consistent
 * way; perhaps you should head in that direction?
 *
 * As you move through the valley of mirrors, you find that several of
 * them have fallen from the large metal frames keeping them in place.
 * The mirrors are extremely flat and shiny, and many of the fallen
 * mirrors have lodged into the ash at strange angles. Because the
 * terrain is all one color, it's hard to tell where it's safe to walk
 * or where you're about to run into a mirror.
 *
 * You note down the patterns of ash (.) and rocks (#) that you see as
 * you walk (your puzzle input); perhaps by carefully analyzing these
 * patterns, you can figure out where the mirrors are!
 *
 * To find the reflection in each pattern, you need to find a perfect
 * reflection across either a horizontal line between two rows or across
 * a vertical line between two columns.
 *
 * To summarize your pattern notes, add up the number of columns to the
 * left of each vertical line of reflection; to that, also add 100
 * multiplied by the number of rows above each horizontal line of
 * reflection.
 *
 * Part 1 - Find the line of reflection in each of the patterns in your
 *          notes. What number do you get after summarizing all of your
 *          notes?
 */

use std::fs;

/// Parse the raw notes on the ash field into vectors of chars
pub fn read_raw_notes(notes_file: &str) -> Vec<Vec<Vec<char>>> {
    return fs::read_to_string(notes_file)
        .unwrap()
        .split("\n\n")
        .map(|x| x.to_string())
        .map(|x| {
            x.split('\n')
                .map(|x| x.to_string().chars().collect())
                .collect()
        })
        .collect();
}

/// Check if a horizontal line of reflection in an ashfield is valid
pub fn is_horiz_reflection(ashfield: &Vec<Vec<char>>, row: usize) -> bool {
    if row >= ashfield.len() - 1 {
        return false;
    }

    let (mut u_row, mut d_row) = (row, row + 1);

    /* working outwards until the edge check that every element matches. */
    loop {
        /* Check if the current rows are identical */
        if ashfield[u_row] != ashfield[d_row] {
            return false;
        }

        /* Exit if the checks have reached the edge of the ashfield */
        if u_row <= 0 || d_row >= ashfield.len() - 1 {
            break;
        }

        /* Prepare the next iteration to check the next rows */
        u_row -= 1;
        d_row += 1;
    }

    return true;
}

/// Check if a vertical line of reflection in an ashfield is valid
pub fn is_verti_reflection(ashfield: &Vec<Vec<char>>, col: usize) -> bool {
    if col >= ashfield[0].len() - 1 {
        return false;
    }

    let (mut u_col, mut d_col) = (col, col + 1);

    /* working outwards until the edge check that every element matches. */
    loop {
        /* Check if the current cols are identical */
        for idx in 0..ashfield.len() - 1 {
            if ashfield[idx][u_col] != ashfield[idx][d_col] {
                return false;
            }
        }

        /* Exit if the checks have reached the edge of the ashfield */
        if u_col <= 0 || d_col >= ashfield[0].len() - 1 {
            break;
        }

        /* Prepare the next iteration to check the next rows */
        u_col -= 1;
        d_col += 1;
    }

    return true;
}

fn main() {
    let raw_notes = read_raw_notes("./data/example_00.txt");
    println!("{:?}", is_verti_reflection(&raw_notes[0], 0));
}
