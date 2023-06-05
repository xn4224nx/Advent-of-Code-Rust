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
use std::collections::HashMap;


/// The Licence Node Structure
struct Node {
    values: Vec<i32>,
    children: HashMap<i32, Node>,
}


impl Node {
    
    
    /// Create a new Licence Node
    fn new(values: Vec<i32>) -> Self {
	
	/* Define the new node */
	Node {
	    values:  values,
	    children: HashMap::new(),
	}
    }
    
    /// Add a child Node to an already existing Node
    fn add_child(&mut self, node_key: i32, values: Vec<i32>) {
	
	/* Create the new child node */
	let mut child_node = Node::new(values);
	
	/* Insert the new child node into the current node */
	self.children.insert(node_key.clone(), child_node);
    }
}


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
    
    /* Read the licence file */
    let raw_licence = read_licence_file("./data/sample.txt");
    
    /* Put the licence data into a tree structure */
    
    println!("{:?}", raw_licence);
}
