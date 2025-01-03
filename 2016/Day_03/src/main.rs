/*
 * --- Day 3: Squares With Three Sides ---
 *
 * Now that you can think clearly, you move deeper into the labyrinth of
 * hallways and office furniture that makes up this part of Easter Bunny HQ.
 * This must be a graphic design department; the walls are covered in
 * specifications for triangles.
 *
 * Or are they?
 *
 * The design document gives the side lengths of each triangle it describes,
 * but... 5 10 25? Some of these aren't triangles. You can't help but mark the
 * impossible ones.
 *
 * In a valid triangle, the sum of any two sides must be larger than the
 * remaining side. For example, the "triangle" given above is impossible,
 * because 5 + 10 is not larger than 25.
 *
 * PART 1:  In your puzzle input, how many of the listed triangles are possible?
 *
 * Now that you've helpfully marked up their design documents, it occurs to you
 * that triangles are specified in groups of three vertically. Each set of three
 * numbers in a column specifies a triangle. Rows are unrelated.
 *
 * PART 2:  In your puzzle input, and instead reading by columns, how many of
 *          the listed triangles are possible?
 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Read the triangles sizes from file
pub fn read_triangles(file_path: &str, vert: bool) -> Vec<(u32, u32, u32)> {
    let re_tri = Regex::new(r"(\d+)\s+(\d+)\s+(\d+)").unwrap();
    let mut triangles = Vec::new();
    let mut buffer = String::new();

    /* Read the file. */
    let file = File::open(file_path).unwrap();
    let mut fp = BufReader::new(file);

    /* Iterate over the file line by line. */
    while fp.read_line(&mut buffer).unwrap() > 0 {
        let raw_line = re_tri.captures(&buffer);

        /* Either capture the three numbers or skip the line. */
        let Some(raw_nums) = raw_line else {
            continue;
        };

        /* Convert the captured side values to integers. */
        triangles.push((
            raw_nums[1].parse::<u32>().unwrap(),
            raw_nums[2].parse::<u32>().unwrap(),
            raw_nums[3].parse::<u32>().unwrap(),
        ));

        buffer.clear();
    }

    /* If the triangles are vertical change the data ordering. */
    if vert && triangles.len() % 3 == 0 {
        let mut v_tri = Vec::new();

        for row_idx in (0..triangles.len()).step_by(3) {
            v_tri.push((
                triangles[row_idx].0,
                triangles[row_idx + 1].0,
                triangles[row_idx + 2].0,
            ));
            v_tri.push((
                triangles[row_idx].1,
                triangles[row_idx + 1].1,
                triangles[row_idx + 2].1,
            ));
            v_tri.push((
                triangles[row_idx].2,
                triangles[row_idx + 1].2,
                triangles[row_idx + 2].2,
            ));
        }
        return v_tri;
    } else {
        return triangles;
    };
}

/// Determine if a triangle is valid.
pub fn is_valid_triangle(triangle: (u32, u32, u32)) -> bool {
    return if triangle.0 >= triangle.1 + triangle.2 {
        false
    } else if triangle.1 >= triangle.0 + triangle.2 {
        false
    } else if triangle.2 >= triangle.0 + triangle.1 {
        false
    } else {
        true
    };
}

/// Count the number of valid triangles in a list
pub fn count_valid_triangles(all_tri: &Vec<(u32, u32, u32)>) -> usize {
    return all_tri.iter().filter(|x| is_valid_triangle(**x)).count();
}

fn main() {
    println!(
        "Part 1 = {}",
        count_valid_triangles(&read_triangles("./data/input.txt", false))
    );

    println!(
        "Part 2 = {}",
        count_valid_triangles(&read_triangles("./data/input.txt", true))
    );
}
