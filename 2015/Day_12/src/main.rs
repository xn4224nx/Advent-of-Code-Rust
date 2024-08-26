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
 */

use serde_json::{json, Value, from_str};
use std::fs;

pub fn read_account_data(filepath: &str) -> serde_json::Value {
    let file_conts: String = fs::read_to_string(filepath).unwrap();
    let mem_json = serde_json::from_str(&file_conts).unwrap();

    return mem_json;
}

pub fn sum_all_nums(acc_data: &serde_json::Value) -> i32 {
    0
}

fn main() {}
