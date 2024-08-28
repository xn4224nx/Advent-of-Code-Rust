/*
 * --- Day 15: Science for Hungry People ---
 *
 * Today, you set out on the task of perfecting your milk-dunking cookie recipe.
 * All you have to do is find the right balance of ingredients.
 *
 * Your recipe leaves room for exactly 100 teaspoons of ingredients. You make a
 * list of the remaining ingredients you could use to finish the recipe (your
 * puzzle input) and their properties per teaspoon:
 *
 *      - capacity (how well it helps the cookie absorb milk)
 *
 *      - durability (how well it keeps the cookie intact when full of milk)
 *
 *      - flavor (how tasty it makes the cookie)
 *
 *      - texture (how it improves the feel of the cookie)
 *
 *      - calories (how many calories it adds to the cookie)
 *
 * You can only measure ingredients in whole-teaspoon amounts accurately, and
 * you have to be accurate so you can reproduce your results in the future. The
 * total score of a cookie can be found by adding up each of the properties
 * (negative totals become 0) and then multiplying together everything except
 * calories.
 *
 * If any properties had produced a negative total, it would have instead become
 * zero, causing the whole score to multiply to zero.
 *
 * PART 1:  Given the ingredients in your kitchen and their properties, what is
 *          the total score of the highest-scoring cookie you can make?
 *
 * Your cookie recipe becomes wildly popular! Someone asks if you can make
 * another recipe that has exactly 500 calories per cookie (so they can use it
 * as a meal replacement). Keep the rest of your award-winning process the same
 * (100 teaspoons, same ingredients, same scoring system).
 *
 * PART 2:  Given the ingredients in your kitchen and their properties, what is
 *          the total score of the highest-scoring cookie you can make with a
 *          calorie total of 500?
 */

use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Debug)]
pub struct Cookie {
    pub capacity: i32,
    pub durability: i32,
    pub flavor: i32,
    pub texture: i32,
    pub calories: i32,
}

/// Read the ingredients file and return it in a structured format
pub fn read_cookie_data(file_path: &str) -> Vec<Cookie> {
    let mut data = Vec::new();

    /* Open the file. */
    let file = File::open(file_path).unwrap();
    let mut fp = BufReader::new(file);

    /* Extract all the integers in the line. */
    let re_int = Regex::new(r"-?\d+").unwrap();

    /* Read the file line by line into a buffer. */
    let mut buffer = String::new();
    while fp.read_line(&mut buffer).unwrap() > 0 {
        let nums: Vec<i32> = re_int
            .find_iter(&buffer)
            .map(|x| x.as_str().parse::<i32>().unwrap())
            .collect();

        data.push(Cookie {
            capacity: nums[0],
            durability: nums[1],
            flavor: nums[2],
            texture: nums[3],
            calories: nums[4],
        });
        buffer.clear();
    }
    return data;
}

/// Score a particular combination of ingredient ammounts and return the score.
pub fn score_cookie_comb(data: &Vec<Cookie>, weights: &Vec<i32>) -> i32 {
    let mut scores = vec![0; 4];

    /* For each property and ingredient, sum the scores for each property. */
    for s_idx in 0..scores.len() {
        for d_idx in 0..data.len() {
            match s_idx {
                0 => scores[s_idx] += weights[d_idx] * data[d_idx].capacity,
                1 => scores[s_idx] += weights[d_idx] * data[d_idx].durability,
                2 => scores[s_idx] += weights[d_idx] * data[d_idx].flavor,
                3 => scores[s_idx] += weights[d_idx] * data[d_idx].texture,
                _ => panic!("Unknown score index."),
            }
        }
    }

    /* Determine the product of the combined ingredient scores. */
    let mut final_scr = 1;
    for scr in scores {
        final_scr *= max(scr, 0);
    }
    return final_scr;
}

/// Calculate the calories of a Cookie combination
pub fn cookie_calories(data: &Vec<Cookie>, weights: &Vec<i32>) -> i32 {
    let mut cals = 0;
    for d_idx in 0..data.len() {
        cals += weights[d_idx] * data[d_idx].calories;
    }
    return cals;
}

/// Find the combination of ingredients that give the highest score
/// and return that score.
pub fn highest_cookie_score(ingr_data: &Vec<Cookie>, total_weight: i32, cal_cnt: bool) -> i32 {
    let mut high_score = 0;

    /* select a combination (with replacement) of the different weights .*/
    for weight_comb in (0..=total_weight).combinations_with_replacement(ingr_data.len()) {
        /* Ensure the weight combination can actually be used. */
        if weight_comb.iter().sum::<i32>() != total_weight {
            continue;
        }

        /* Iterate over the possible combinations of the weight vector. */
        for weight_perm in weight_comb.into_iter().permutations(ingr_data.len()) {
            let curr_score = score_cookie_comb(&ingr_data, &weight_perm);

            /* If the calories need to be 500, reject all failures */
            if cal_cnt && cookie_calories(&ingr_data, &weight_perm) != 500 {
                continue;
            }

            /* Record if a new high score has been found. */
            if curr_score > high_score {
                high_score = curr_score;
            }
        }
    }
    return high_score;
}

fn main() {
    let data = read_cookie_data("./data/input.txt");
    println!("Part 1 = {}", highest_cookie_score(&data, 100, false));
    println!("Part 2 = {}", highest_cookie_score(&data, 100, true));
}
