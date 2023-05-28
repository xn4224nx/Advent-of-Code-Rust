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

/* HashMaps */
use  std::collections::HashMap;

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



fn create_lookup(instructions: Vec<(char, char)>) -> HashMap<char, Vec<char>> {
    /* Create a HashMap of the required Nodes for each Node */
    
    let mut req_nodes: HashMap<char, Vec<char>> = HashMap::new();
    
    /* Iterate over the vector of instructions */
    for inst in instructions {
	
	/* If a key doesn't exist create it. */
	req_nodes.entry(inst.0).or_insert(vec![]);
	
	/* Add the requred nodes to the hashmap */
	req_nodes.entry(inst.1).and_modify(|node| node.push(inst.0))
				.or_insert(vec![inst.0]);
    }
    
    return req_nodes
}


fn find_node_order(node_lookup: HashMap<char, Vec<char>>) -> String {
    /* Create a string that indicates the node order. */
    
    /* Find the first Node */
    
    /* Iterate over the Node map and find ones that can be next */
    
    
    return String::from("");
}

fn main() {
    
    /* Load the instructions from disk. */
    let instruc = load_instructions("./data/sample.txt");

    /* Add the chars into a hashmap. */
    let node_dep = create_lookup(instruc);

    
    println!("{:?}", node_dep);
    
    
    /* Loop over the data structure. */
    
    /* Find the step that has no prerequisites.  */
    
    /* Iterate over the datastrucutre working out the next steps. */
    
}
