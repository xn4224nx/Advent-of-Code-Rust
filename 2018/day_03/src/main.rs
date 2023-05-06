/************************************************************************
*									*
*	--- Day 3: No Matter How You Slice It ---			*
*									*
*	Part 1 - How many square inches of fabric are within two or 	*
* 		more claims?						*
*									*
*	Part 2 - What is the ID of the only claim that doesn't 		*
* 		overlap?						*
* 									*			
************************************************************************/


use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;


/// Each claim has an id and a rectangular shape in a 2D grid.
#[derive(Debug)]
struct Claim {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}


impl Claim {
    
    /// Extract all the points in a rectangle
    ///
    fn all_points(&self) -> Vec<(i32, i32)> {
	
	// Storage for all the points in the claim
	let mut points = Vec::new();
	
	for i in self.x..(self.x + self.width) {
	    for j in self.y..(self.y + self.height) {
		points.push((i, j));
	    }
	}
	
	return points
    }
}


/// Reads a file line by line as an iterator
///
fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    
    let file = File::open(file_name)?;
    return Ok(io::BufReader::new(file).lines())
}


fn main() {
   
    /* Storage for the parsed claim data. */
    let mut claim_data = vec![];
    
    /* Regex Group to Extract the Claim Data */
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    
    /* Load the area data from file. */
    if let Ok(lines) = read_lines("./data/input.txt") {
    
	for line in lines {
	    
	    if let Ok(ip_line) = line {
		
		/* Parse the line into the Claim structure */
		let cap = re.captures(&ip_line).unwrap();
		
		/* Put the parsed data into a structure */
		let tmp = Claim {
		    id: 	cap.get(1).unwrap()
					    .as_str()
					    .parse::<i32>()
					    .unwrap(),
		    
		    x: 		cap.get(2).unwrap()
					    .as_str()
					    .parse::<i32>()
					    .unwrap(),
		    
		    y: 		cap.get(3).unwrap()
					    .as_str()
					    .parse::<i32>()
					    .unwrap(),
		    
		    width: 	cap.get(4).unwrap()
					    .as_str()
					    .parse::<i32>()
					    .unwrap(),
		    
		    height: 	cap.get(5).unwrap()
					    .as_str()
					    .parse::<i32>()
					    .unwrap(),
		    
		};
		
		/* Save the claim data */
		claim_data.push(tmp)
	    }
	}  
    }
    
    /* Store details about the claims */
    let mut all_points = HashMap::new();
    let mut claim_ids = HashSet::new();
    
    /* Iterate over the claims */
    for claim in &claim_data {
	
	/* Extract the points that the claim covers */
	let claim_points = claim.all_points();
	
	for point in &claim_points {
	    
	    /* 	If the point doesn't exist create it with count 1
	    otherwise add one to its count. */
	    all_points.entry(point.clone())
			.and_modify(|x| *x += 1)
			.or_insert(1);
	}
	
	claim_ids.insert(claim.id);
    }
    
    /* Count the points that overlap */
    let mut overlaping_points = 0;
    for (_point, count) in &all_points {
	
	if *count > 1 {overlaping_points += 1}
    }
    
    println!("Part 1: the overlaping area is {} inches².", overlaping_points);   
    
    /* Detect overlaps */
    for claim in &claim_data {
	
	let mut overlap = false;
	
	/* Extract the points that the claim covers */
	let claim_points = claim.all_points();
	
	for point in &claim_points {
	    if all_points.get(point) > Some(&1) {
		
		overlap = true;
		break
	    }
	}
	
	if !overlap {
	    println!("Part 2: the non-overlaping claim is {}.", claim.id); 
	    break
	}	
    } 
}
