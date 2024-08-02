/*
 * --- Day 2: I Was Told There Would Be No Math ---
 *
 * The elves are running low on wrapping paper, and so they
 * need to submit an order for more. They have a list of the
 * dimensions (length l, width w, and height h) of each
 * present, and only want to order exactly as much as they
 * need.
 *
 * Fortunately, every present is a box (a perfect right
 * rectangular prism), which makes calculating the required
 * wrapping paper for each gift a little easier: find the
 * surface area of the box, which is 2*l*w + 2*w*h + 2*h*l. The
 * elves also need a little extra paper for each present: the
 * area of the smallest side.
 *
 * All numbers in the elves' list are in feet.
 *
 * PART 1:  How many total square feet of wrapping paper should
 *          they order?
 *
 * The elves are also running low on ribbon. Ribbon is all the
 * same width, so they only have to worry about the length they
 * need to order, which they would again like to be exact.
 *
 * The ribbon required to wrap a present is the shortest distance
 * around its sides, or the smallest perimeter of any one face.
 * Each present also requires a bow made out of ribbon as well;
 * the feet of ribbon required for the perfect bow is equal to
 * the cubic feet of volume of the present. Don't ask how they
 * tie the bow, though; they'll never tell.
 *
 * PART 2: How many total feet of ribbon should they order?
 */

use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Read the instruction file and parse into a vector of tuples
/// comprised of three integers.
pub fn read_instructions(inst_filepath: &str) -> Vec<(usize, usize, usize)> {
    let mut parsed_inst = Vec::new();
    let re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();

    /* Open the instruction file. */
    let file_ptr = File::open(inst_filepath).unwrap();
    let reader = BufReader::new(file_ptr);

    for raw_line in reader.lines() {
        /* Parse the line or skip it */
        let Ok(line) = raw_line else {
            continue;
        };

        match re.captures(&line) {
            Some(caps) => {
                parsed_inst.push((
                    caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                ));
            }
            None => {
                continue;
            }
        };
    }
    return parsed_inst;
}

/// Calculate the length of ribbon and area of wrapping paper for a box
pub fn calc_ribb_and_wrap(box_dims: (usize, usize, usize)) -> (usize, usize) {
    let (l, w, h) = box_dims;

    /* The bow requires a length of ribbon the same magnitude
     * as the boxes volume */
    let bow_len = l * w * h;

    /* Calculate the area of the 3 surfaces of the box. */
    let sfc_area = 2 * l * w + 2 * w * h + 2 * h * l;

    /* Find the side with the smallest perimeter and the smallest area */
    let small_prei;
    let small_side;

    if l <= h && w <= h {
        small_prei = 2 * l + 2 * w;
        small_side = l * w;
    } else if w <= l && h <= l {
        small_prei = 2 * w + 2 * h;
        small_side = w * h;
    } else {
        small_prei = 2 * h + 2 * l;
        small_side = h * l;
    }

    return (bow_len + small_prei, sfc_area + small_side);
}

fn main() {
    let results: Vec<(usize, usize)> = read_instructions("./data/input.txt")
        .into_iter()
        .map(|x| calc_ribb_and_wrap(x))
        .collect();

    println!(
        "Answer to part 1 = {}",
        results.iter().map(|x| x.1).sum::<usize>()
    );

    println!(
        "Answer to part 2 = {}",
        results.iter().map(|x| x.0).sum::<usize>()
    );
}
