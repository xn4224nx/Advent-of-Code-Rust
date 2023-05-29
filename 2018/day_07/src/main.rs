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
use std::collections::HashSet;

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



fn create_lookup(instructions: Vec<(char, char)>) -> 
    HashMap<char, HashSet<char>> {
    /* Create a HashMap of the required Nodes for each Node */
    
    let mut req_nodes: HashMap<char, HashSet<char>> = HashMap::new();
    
    /* Iterate over the vector of instructions */
    for inst in instructions {
	
	/* If a key doesn't exist create it. */
	req_nodes.entry(inst.0).or_insert(HashSet::new());
	
	/* Add the requred nodes to the hashmap */
	req_nodes.entry(inst.1).and_modify(|node| {node.insert(inst.0);})
				.or_insert(HashSet::from([inst.0]));
    }
    
    return req_nodes
}


fn find_node_order(node_lookup: HashMap<char, HashSet<char>>) -> String {
    /* Create a string that indicates the node order. */
    
    
    /* Copy the contents of the node_lookup into another HashMap */
    let mut lookup: HashMap<char, HashSet<char>> = HashMap::new();
    
    for (key, value) in node_lookup.iter() {
	lookup.insert(*key, value.clone());
    }
    
    
    /* String that will store the answers */
    let mut node_order = String::from("");
    
    
    /* Iterate over the hash map while the string has less chars than nodes */
    while lookup.len() > 0 {
	
	/* Find nodes that have no requirements */
	let mut all_next_nodes = vec![];
	
	for (key, value) in lookup.iter_mut() {
	    
	    /* If the node has no required nodes save it */
	    if value.len() == 0 {
		all_next_nodes.push(key.clone());
	    }
	}
	
	
	/* Order the next_node vector */
	all_next_nodes.sort();
	
	/* Select the First Node */
	let next_node = &all_next_nodes[0];
	
	/* Remove the next node from  the hashmap */
	lookup.remove(next_node);
	
	/* Remove the next node from the values */
	for value in lookup.values_mut() {

	    value.remove(next_node);
	}


	/* Add the Letters to the node order */
	node_order.push(*next_node);

	println!("{:?}", all_next_nodes);
	println!("{:?}", lookup);
	println!("{}", node_order);

    }
    
    return node_order;
}

fn main() {
    
    /* Load the instructions from disk. */
    let instruc = load_instructions("./data/sample.txt");

    /* Add the chars into a hashmap. */
    let node_dep = create_lookup(instruc);

    
    println!("{:?}", node_dep);
    
    /* Find the Node  */
    find_node_order(node_dep);
    
}
