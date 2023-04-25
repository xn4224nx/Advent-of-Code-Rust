/******************************************************************************
*																			  *
*					--- Day 1: Chronal Calibration ---						  *
*																			  *
*	Part 1 - 	Starting with a frequency of zero, what is the resulting 	  *
* 				frequency after all of the changes in frequency have been 	  *
* 				applied?													  *
*																			  *
******************************************************************************/

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines <P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

fn main() {
	
	/* Record the sum of frequencies */
	let mut freq_sum = 0;
	
	/* Vector to store the read frequencies */
	let mut freqs: Vec<i32> = vec![];
	
	/* Read the data file from disk. */
	if let Ok(lines) = read_lines("data/input.txt") {
		
		/* Read the file in line by line */
		for line in lines {
			if let Ok(ip) = line {
				
				/* Parse the string */
				let tmp_freq = ip.parse().unwrap();
				
				freqs.push(tmp_freq);
				
				freq_sum += tmp_freq;
				
				//println!("{}", tmp_freq);
			}
		}
	}


	/* Calculate the resultant frequency. */
	println!("Part 1: The sum of the frequencies are: {}", freq_sum);
}




