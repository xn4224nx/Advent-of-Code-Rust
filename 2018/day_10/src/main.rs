/************************************************************************
*									                                    *
*	--- Day 10: The Stars Align ---					                    *
*									                                    *
*	Part 1 - What message will eventually appear in the sky?            *
*									                                    *
************************************************************************/

use std::fs::read_to_string;


fn main() {

    /* Read the data into memory. */
    let raw_lines = read_lines_to_vec("./data/sample.txt");
    
    /* Print to screen. */
    for line in raw_lines {
    
        println!("{}", line);
    }
    
}


fn read_lines_to_vec(filename: &str) -> Vec<String> {
    
    let mut file_lines = Vec::new();
    
    /* Read the file into a list. */
    for line in read_to_string(filename).unwrap().lines() {
        file_lines.push(line.to_string());
    }
    
    return file_lines;
}

