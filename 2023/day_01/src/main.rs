/*
 * --- Day 1: Trebuchet?! ---
 *
 * As they're making the final adjustments, they discover that their
 * calibration document (your puzzle input) has been amended by a very
 * young Elf who was apparently just excited to show off her art skills.
 * Consequently, the Elves are having trouble reading the values on the
 * document.
 *
 * The newly-improved calibration document consists of lines of text;
 * each line originally contained a specific calibration value that the
 * Elves now need to recover. On each line, the calibration value can be
 * found by combining the first digit and the last digit (in that order)
 * to form a single two-digit number.
 *
 * Consider your entire calibration document. What is the sum of all of
 * the calibration values?
 * 
 * Your calculation isn't quite right. It looks like some of the digits 
 * are actually spelled out with letters: one, two, three, four, five, 
 * six, seven, eight, and nine also count as valid "digits".
 * 
 * Equipped with this new information, you now need to find the real 
 * first and last digit on each line.
 */

use regex::Regex;
use std::fs;

fn main() {
    let source_fp = "../data/input.txt";

    let mut ex_nums = vec![];

    /* Load the raw text file and split it by new lines. */
    let contents = fs::read_to_string(source_fp).expect("Unable To Read File");
    let instructs: Vec<_> = contents.split('\n').collect();

    /* Iterate over the lines and extract the numbers. */
    let re = Regex::new(r"(\d)").unwrap();

    for lin_ist in &instructs {
        /* Extract all the digits in the string. */
        let digits: Vec<_> = re.captures_iter(lin_ist).collect();
        if digits.len() <= 0 {
            continue;
        };

        /* Create the number from the first and last digit. */
        let num = digits[0][0].to_owned() + &digits[digits.len() - 1][0];
        let fin_num = num.parse::<u32>().unwrap();

        ex_nums.push(fin_num);
    }
    println!("Sum of numbers = {}", ex_nums.iter().sum::<u32>());
}
