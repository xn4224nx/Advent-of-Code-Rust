/*
 * --- Day 12: JSAbacusFramework.io ---
 *
 * Santa's Accounting-Elves need help balancing the books after a recent order.
 * Unfortunately, their accounting software uses a peculiar storage format.
 * That's where you come in.
 *
 * They have a JSON document which contains a variety of things: arrays
 * ([1,2,3]), objects ({"a":1, "b":2}), numbers, and strings. Your first job is
 * to simply find all of the numbers throughout the document and add them
 * together.
 *
 * You will not encounter any strings containing numbers.
 *
 * PART 1:  What is the sum of all numbers in the document?
 *
 * Uh oh - the Accounting-Elves have realized that they double-counted
 * everything red.
 *
 * Ignore any object (and all of its children) which has any property with the
 * value "red". Do this only for objects ({...}), not arrays ([...]).
 *
 * PART 2:  What is the sum of all numbers in the document excluding red objects
 *          and the children of red objects?
 */

use regex::Regex;
use std::fs;

/// Read a text file and convert it to a single string value.
pub fn read_account_data(filepath: &str) -> String {
    return fs::read_to_string(filepath).unwrap().trim().to_string();
}

/// Find all numbers in the string and sum them together.
pub fn sum_all_nums(acc_data: &String) -> i32 {
    let num_re = Regex::new(r"-?\d+").unwrap();
    let mut sum = 0;

    for raw_num in num_re.find_iter(acc_data) {
        sum += raw_num.as_str().parse::<i32>().unwrap();
    }

    return sum;
}

/// Sum the account numbers that are not associated with red.
pub fn sum_non_red(acc_data: &String) -> i32 {
    0
}

fn main() {
    let acc_data = read_account_data("./data/input.txt");

    println!("Part 1 = {}", sum_all_nums(&acc_data));
}
