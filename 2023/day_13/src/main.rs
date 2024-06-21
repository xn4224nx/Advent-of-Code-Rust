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
 *
 * You resume walking through the valley of mirrors and - SMACK! - run
 * directly into one. Hopefully nobody was watching, because that must
 * have been pretty embarrassing.
 *
 * Upon closer inspection, you discover that every mirror has exactly
 * one smudge: exactly one . or # should be the opposite type.
 *
 * In each pattern, you'll need to locate and fix the smudge that causes
 * a different reflection line to be valid. (The old reflection line
 * won't necessarily continue being valid after the smudge is fixed.)
 *
 * Part 2 - In each pattern, fix the smudge and find the different line
 *          of reflection. What number do you get after summarizing the
 *          new reflection line in each pattern in your notes?
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
pub fn is_horiz_reflection(ashfield: &Vec<Vec<char>>, row: usize, smudges: usize) -> bool {
    if row >= ashfield.len() - 1 {
        return false;
    }

    let mut errors = 0;

    let (mut u_row, mut d_row) = (row, row + 1);

    /* working outwards until the edge check that every element matches. */
    loop {
        /* Check if the current rows are identical */
        for idx in 0..ashfield[d_row].len() {
            if ashfield[u_row][idx] != ashfield[d_row][idx] {
                errors += 1;
            }

            if errors > smudges {
                return false;
            }
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
pub fn is_verti_reflection(ashfield: &Vec<Vec<char>>, col: usize, smudges: usize) -> bool {
    if col >= ashfield[0].len() - 1 {
        return false;
    }

    let mut errors = 0;

    let (mut u_col, mut d_col) = (col, col + 1);

    /* working outwards until the edge check that every element matches. */
    loop {
        /* Check if the current cols are identical */
        for idx in 0..ashfield.len() - 1 {
            if ashfield[idx][u_col] != ashfield[idx][d_col] {
                errors += 1;
            }

            if errors > smudges {
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

/// Determine the score for an ashfield based on its mirror
pub fn ashfield_score(ashfield: &Vec<Vec<char>>, smudges: usize) -> usize {
    /* Determine if there is a horizontal reflection. */
    for col in 0..ashfield.len() {
        if is_horiz_reflection(ashfield, col, smudges) {
            return (col + 1) * 100;
        }
    }

    /* Determine if there is a vertical reflection. */
    for row in 0..ashfield[0].len() {
        if is_verti_reflection(ashfield, row, smudges) {
            return row + 1;
        }
    }

    panic!("No refection found");
}

fn main() {
    let raw_notes = read_raw_notes("./data/input.txt");

    println!(
        "Part 1 answer = {}",
        raw_notes
            .iter()
            .map(|x| ashfield_score(x, 0))
            .sum::<usize>()
    );

    // 33807 too low
    // 48494 too high

    println!(
        "Part 2 answer = {}",
        raw_notes
            .iter()
            .map(|x| ashfield_score(x, 1))
            .sum::<usize>()
    );
}
