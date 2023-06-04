/************************************************************************
*									*
*	--- Day 7: The Sum of Its Parts ---				*
*									*
*	Part 1 - Using the supplied step instructions determine what 	*
* 		order they should be completed in. 			*
*									*
* 	Part 2 - With 5 workers and the 60+ second step durations, how 	*
* 		long will it take to complete all of the steps?		*
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


fn find_node_order(node_lookup: &HashMap<char, HashSet<char>>) -> String {
    /* Create a string that indicates the node order. */
        
    /* Copy the contents of the node_lookup into another HashMap */
    let mut lookup = node_lookup.clone();
    
    /* String that will store the answers */
    let mut node_order = String::from("");
    
    /* Iterate over the hash map while the string has less chars than nodes */
    while lookup.len() > 0 {
	
	/* Find nodes that have no requirements */
	let mut all_next_nodes = find_next_nodes(&lookup);
	    
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
    }
    
    return node_order;
}


fn find_next_nodes(node_lookup: &HashMap<char, HashSet<char>>) -> Vec<char> {
    /* Find all nodes that have no requirements and sort them in order.  */
    
    let mut all_next_nodes = vec![];
    
    for (key, value) in node_lookup.iter() {
	
	/* If the node has no required nodes save it */
	if value.len() == 0 {
	    all_next_nodes.push(key.clone());
	}
    }

    return all_next_nodes
}


fn find_total_step_time(node_lookup: &HashMap<char, HashSet<char>>, 
    num_workers: usize) -> i32 {
    /* Find the total time to process all the steps with a number of workers*/
    let mut total_time = 0;
    
    /* Copy the contents of the node_lookup into another HashMap */
    let mut lookup = node_lookup.clone();
    
    /* Create a object to identify the Worker Allocation */
    let mut workers: Vec<i32> = vec![0; num_workers];
    
    /* Vector to store all the required nodes */
    //let mut all_next_nodes = vec![];
    
    /* Iterate over the hash map while the string has less chars than nodes */
    loop {
	
	/* Find nodes that have no requirements */
	let mut all_next_nodes =  find_next_nodes(&lookup);

	/* Put in reverse order and sort */
	all_next_nodes.sort();
	all_next_nodes.reverse();
	
	/* Allocate the tasks to free workers. */
	for i in 0..workers.len() {
	    
	    /* If the worker is free */
	    if workers[i] == 0 && all_next_nodes.len() > 0 {
		
		let next_node = all_next_nodes.pop().unwrap();
		
		println!("{}", next_node);
		
		/* Assign the task to the free worker */
		workers[i] = alpha_index(next_node);
		
		/* Remove the next node from  the hashmap */
		lookup.remove(&next_node);
		
		/* Remove the next node from the values */
		for value in lookup.values_mut() {
		    value.remove(&next_node);
		}
	    }
	}
	
	println!("{:?}", workers);
	
	/* Work out the minimum time til the next worker is free */
	let mut min_time = 99;
	
	for i in 0..workers.len() {
	    if workers[i] < min_time && workers[i] > 0 {
		min_time = workers[i];
	    }
	}
	
	/* Add the time to the total time */
	total_time += min_time;
	
	/* Take minimum time off every worker */
	for i in 0..workers.len() {
	    if workers[i] > 0 {
		 workers[i] -= min_time;
	    }
	}
	
	println!("{:?}", workers);
	
	/* If every step has been enacted and every worker finished exit. */
	if lookup.len() <= 0 && workers.iter().sum::<i32>() == 0 {break}
    }
    
    return total_time + 1
}


fn alpha_index(alpha: char) -> i32 {
    /* Find the index of the alphabet character A=61 ... Z=86 */ 
    
    return (61 + (alpha as u32) - ('A' as u32)).try_into().unwrap()
}


fn main() {
    
    /* Load the instructions from disk. */
    let instruc = load_instructions("./data/input.txt");

    /* Add the chars into a hashmap. */
    let node_dep = create_lookup(instruc);
    
    /* Find the Node Execution Order */
    let node_order = find_node_order(&node_dep);
    println!("The answer to part 1 is \"{}\".", node_order);
    
    let step_time = find_total_step_time(&node_dep, 5);
    
    println!("{}", step_time);
    
    // 438 too low
}
