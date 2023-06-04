/************************************************************************
*									*
*	--- Day 8: Memory Maneuver ---					*
*									*
*	Part 1 - What is the sum of all metadata entries?		*
*									*	
************************************************************************/

use std::path::Path;
use std::fs::File;
use std::io::Read;


/// Read the licence file and return a vector of the integer values
fn read_licence_file<P>(filename: P) -> Vec<i32>
where P: AsRef<Path>{
    
    /* Open the file */
    let mut file = File::open(filename).expect("Unable to open licence.");
    
    /* Read the file into a string */
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Unable to read licence.");
    
    /* Parse the licence numbers into a Vector */
    let licence: Vec<i32> = buffer.split_whitespace()
					    .map(|s| s.parse().unwrap())
					    .collect();
    return licence;
}


fn main() {
    let raw_licence = read_licence_file("./data/sample.txt");
    
    println!("{:?}", raw_licence);
}
