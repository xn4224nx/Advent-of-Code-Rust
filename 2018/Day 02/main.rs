/************************************************************************
*									*
*	--- Day 2: Inventory Management System ---			*
*									*
*	Part 1 - Calculate a checksum for the list. Count the number	*
* 		that have two of any letter and the number with three	*
* 		of any letter and multiply the two counts.		*
*									*
* 	Part 2 - What letters are common between the two IDs?		*
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


/* Find the two strings that are closest to each other. */
fn two_closest_str(arr: &Vec<String>) -> String {
    
    /* Record the highest ammount of common characters */
    let mut max_common_char = 0;
    
    /* Record the combination of strings that have the highest common chars */
    let mut common_chars = String::from("");
    
    
    /* Compare every every String in the array to every other */
    for i in 0..arr.len() {
	for j in i..arr.len() {
	    
	    /* Don't compare a string to itself. */
	    if i == j {continue};
	    
	    /* Find out how many common characters the two strings have */
	    let tmp_cnt = num_common_chars(&arr[i], &arr[j]);
	    
	    if tmp_cnt > max_common_char {
		
		/* Set the new maximum */
		max_common_char = tmp_cnt;
		
		/* Extract the common characters. */
		common_chars = extract_common_chars(&arr[i], &arr[j]);
	    }
	    
	}
    }
    
    return common_chars;
}


/* Find out how many char in two strings are not the same. */
fn num_common_chars(str1: &String, str2: &String) -> i32 {
    
    /* This will only work with strings of the same length. */
    
    let mut num_common_chars = 0;
    
    /* Convert the strings to arrays of chars. */
    let str_char_1: Vec<char> = str1.chars().collect();
    let str_char_2: Vec<char> = str2.chars().collect();
    
    /* Iterate over the length of the strings and compare chars */
    for i in 0..str1.len() {
	if str_char_1[i] == str_char_2[i] {
	    num_common_chars += 1
	}
    }
    
    return num_common_chars
}


/* Create a string of the common characters in two strings */
fn extract_common_chars(str1: &String, str2: &String) -> String {
    
   let mut com_chars = String::from("");
    
    /* Convert the strings to arrays of chars. */
    let str_char_1: Vec<char> = str1.chars().collect();
    let str_char_2: Vec<char> = str2.chars().collect();
    
    /* Iterate over the length of the strings and compare chars */
    for i in 0..str1.len() {
	if str_char_1[i] == str_char_2[i] {
	    com_chars.push(str_char_1[i])
	}
    }
    
    return com_chars
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
		//println!("{}", parsed_line);
		//println!("{:?}", tmp_map);
		
		/* Count the lines that exactly two or three of any letter */
		if value_in_map(&tmp_map, 2) {cnt_2_let += 1}
		if value_in_map(&tmp_map, 3) {cnt_3_let += 1}
		
		/* Load the strings & maps into vectors */
		file_lines.push(parsed_line);
		letter_counts.push(tmp_map);
	    }
	}
    }
    
    /* Part 1 - Calculate the Checksum */
    println!("The Checksum = {}", cnt_2_let * cnt_3_let);
    
    /* Part 2 - */
    println!("The  Common Letters are \"{}\"", two_closest_str(&file_lines));
}
