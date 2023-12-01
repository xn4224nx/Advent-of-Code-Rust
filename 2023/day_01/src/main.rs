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
use std::collections::HashMap;



fn main() {
    let source_fp = "../data/input.txt";
	
	let num_replace: HashMap<&str, &str> = HashMap::from([
		("zero", "z0o"),
		("one", "o1e"),
		("two", "t2o"),
		("three", "t3e"),
		("four", "f4r"),
		("five", "f5e"),
		("six", "s6x"),
		("seven", "s7n"),
		("eight", "e8t"),
		("nine", "n9e"),
	]);
	
    let mut ex_nums = vec![];

    /* Load the raw text file and split it by new lines. */
    let contents = fs::read_to_string(source_fp).expect("Unable To Read File");
    let instructs: Vec<_> = contents.split('\n').collect();

    /* Iterate over the lines and extract the numbers. */
    let re = Regex::new(r"(\d)").unwrap();

    for lin_ist in instructs {
		
		let mut rep_line = lin_ist.to_owned();
		
        /* Replace the written digits. */
        for (orig_txt, rep_txt) in num_replace.iter() {
			rep_line = rep_line.replace(orig_txt, rep_txt);
		}
		
        /* Extract all the digits in the string. */
        let digits: Vec<_> = re.captures_iter(&rep_line).collect();
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
