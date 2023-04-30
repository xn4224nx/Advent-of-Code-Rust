/************************************************************************
*									*
*	--- Day 2: Inventory Management System ---			*
*									*
*	Part 1 - Calculate a checksum for the list. Count the number	*
* 		that have two of any letter and the number with three	*
* 		of any letter and multiply the two counts.		* 
*									*
************************************************************************/


use std::{fs::File, io, io::BufRead, path::Path};
use std::{collections::HashMap};

/* Read Lines From a File */
fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines())
}

/* Create character count in a string */
fn char_count(line: &String) -> HashMap<char, i32> {
    
    /* Storage for the counts */
    let mut char_count: HashMap<char, i32> = HashMap::new();
    
    /* Convert the string to a vector of chars */
    let chars_in_str: Vec<char> = line.to_lowercase().chars().collect();
    
    /* Insert the chars into the hashmap */
    for line_char in chars_in_str {
	
	*char_count.entry(line_char).or_insert(0) += 1;
    }
    
    return char_count
}

/* Check Value is in a HashMap */
fn value_in_map(map: &HashMap<char, i32>, value: i32) -> bool {
    
    for map_value in map.values() {
	if *map_value == value {
	    return true
	}
    }
    return false
}

fn main() {
    
    /* Storage for the lines in the file. */
    let mut file_lines = vec![];
    
    /* Letter counts for each line in the file. */
    let mut letter_counts = vec![];
    
    /* Count of the repeated letters */
    let mut cnt_2_let = 0;
    let mut cnt_3_let = 0;
    
    /* Load the text file in line by line */
    if let Ok(lines) = read_lines("data/input.txt") {
	
	for line in lines {
	    
	    /* Parse the string */
	    if let Ok(parsed_line) = line {
		
		/* Create a hash map of the chars in the string */
		let tmp_map = char_count(&parsed_line);
		
		/* Show the parsed values*/
		println!("{}", parsed_line);
		println!("{:?}", tmp_map);
		
		/* Count the lines that exactly two or three of any letter */
		if value_in_map(&tmp_map, 2) {
		    cnt_2_let += 1
		}
		
		if value_in_map(&tmp_map, 3) {
		    cnt_3_let += 1
		}
		
		println!("2 repeated letters: {}", value_in_map(&tmp_map, 2));
		println!("3 repeated letters: {}", value_in_map(&tmp_map, 3));
		
		
		println!("");
		
		/* Load the strings & maps into vectors */
		file_lines.push(parsed_line);
		letter_counts.push(tmp_map);
		
	    }
	}
    }
    
    /* Part 1 - Calculate the Checksum */
    println!("The Checksum = {}", cnt_2_let * cnt_3_let);
}
