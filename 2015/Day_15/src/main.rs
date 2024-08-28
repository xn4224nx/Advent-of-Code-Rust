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
 */

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
    Vec::new()
}

/// Score a particular combination of ingredient ammounts and return the score.
pub fn score_cookie_comb(ingr_data: &Vec<Cookie>, weights: &Vec<i32>) -> i32 {
    0
}

/// Find the combination of ingredients that give the highest score
/// and return that score.
pub fn highest_cookie_score(ingr_data: &Vec<Cookie>, total_ingr: i32) -> i32 {
    0
}

fn main() {}
