/************************************************************************
*									*
*	--- Day 7: The Sum of Its Parts ---				*
*									*
*	Part 1 - Using the supplied step instructions determine what 	*
* 		order they should be completed in. 			*
*									*
************************************************************************/

/* Load and read the file */
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn load_instructions<P>(filename: P) -> Vec<(char, char)> 
where P: AsRef<Path>,{
    /* Load the instructions from file and parse into a vector. */
    
    let instructs = vec![];
    
    /* Open the instructions file as read only. */
    let file = File::open(filename).unwrap();
    
    /* Load the file and iterate over it */
    if let lines = io::BufReader::new(file).lines() {
	    
	for line in lines {
	    if let Ok(raw_line) = line {
		println!("{}", raw_line);
	    }
	}
    }
	
    return instructs
}


fn main() {
    
    /* Load the instructions from disk. */
    let var = load_instructions("./data/sample.txt");
    
    
    /* Parse each line into a tuple of chars. */
    
    /* Add the chars into a hashmap. */
    
    /* Loop over the data structure. */
    
    /* Find the step that has no prerequisites.  */
    
    /* Iterate over the datastrucutre working out the next steps. */
    
}
