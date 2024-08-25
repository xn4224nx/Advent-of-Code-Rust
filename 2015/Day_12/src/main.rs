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

use serde_json::{json, Value};

pub fn read_account_data(filepath: &str) -> serde_json::Value {
    return json!({});
}

pub fn sum_all_nums(acc_data: &serde_json::Value) -> i32 {
    0
}

fn main() {}
