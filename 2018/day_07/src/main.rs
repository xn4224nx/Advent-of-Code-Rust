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

/* Parse The Data */
use regex::Regex;

fn load_instructions<P>(filename: P) -> Vec<(char, char)> 
where P: AsRef<Path>,{

    /* Regex to extract the coordinate data */
    let re_coords = Regex::new(
	    r"([A-Z]) must be finished before step ([A-Z])").unwrap();
    
    /* Storage for the parsed instructions */
    let mut instructs = vec![];
    
    /* Open the instructions file as read only. */
    let file = File::open(filename).unwrap();
    
    /* Load the file and iterate over it */
    if let lines = io::BufReader::new(file).lines() {
	    
	for line in lines {
	    if let Ok(raw_line) = line {
		
		/* Parse the values */
		match re_coords.captures(&raw_line) {
		    Some(caps) => {
		    
			/* Insert into a vector */
			instructs.push(( 
			    caps.get(1).unwrap().as_str()
					.chars().next().unwrap(),
			    caps.get(2).unwrap().as_str()
					.chars().next().unwrap(),
			));
		    }
		    
		    None => break
		}
	    }
	}
    }
	
    return instructs
}


fn main() {
    
    /* Load the instructions from disk. */
    let var = load_instructions("./data/sample.txt");
    
    for grp in var {
	
	println!("{} {}", grp.0, grp.1);
    }
    
    /* Parse each line into a tuple of chars. */
    
    /* Add the chars into a hashmap. */
    
    /* Loop over the data structure. */
    
    /* Find the step that has no prerequisites.  */
    
    /* Iterate over the datastrucutre working out the next steps. */
    
}
