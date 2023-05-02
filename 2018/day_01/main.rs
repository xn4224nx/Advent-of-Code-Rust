/************************************************************************
*									*
*	--- Day 1: Chronal Calibration ---				*
*									*
*	Part 1 - Starting with a frequency of zero, what is the 	*
*		resulting frequency after all of the changes in 	*
*		frequency have been applied?				*
*									*
*	Part 2	- What is the first frequency your device reaches 	*
*		twice?		  					*
*									*
************************************************************************/

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines <P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

fn main() {
	
	/* Record the sum of frequencies */
	let mut freq_sum = 0;
	
	/* Record the seen frequencies */
	let mut seen_freq: HashSet<i32> = HashSet::new();
	let mut duplicate_not_found = true;
	let mut first_dup_freq: Option<i32> = None;
	
	/* Vector to store the read frequencies */
	let mut frequencies: Vec<i32> = vec![];
	
	/* Read the data file from disk. */
	if let Ok(lines) = read_lines("data/input.txt") {	
		
		/* Read the file in line by line */
		for line in lines {
			if let Ok(ip) = line {
				
				/* Parse the string */
				let tmp_freq = ip.parse().unwrap();
				frequencies.push(tmp_freq);
				
				/* Calculate the sum of all the frequencies */
				freq_sum += tmp_freq;
			}
		}
	}
	
	/* Show the answer to the first part */
	println!("Part 1: The sum of the frequencies are: {}", freq_sum);
	
	/* Loop over the vector of frequecies until a duplicate is seen */
	freq_sum = 0;
	
	while duplicate_not_found {
		
		/* Loop over the frequencies */
		for freq in frequencies.iter() {
			
			freq_sum += freq;
			
			/* Check if the frequency has been seen */
			if seen_freq.contains(&freq_sum) {
				
				first_dup_freq = Some(freq_sum);
				duplicate_not_found = false;
				break;
				
			} else {
				seen_freq.insert(freq_sum);
			}
		}
	}
	
	println!("Part 2: The first duplicate frequency is: {}", 
				first_dup_freq.unwrap()); 
}
