/************************************************************************
*									*
*	--- Day 5: Alchemical Reduction ---				*
*									*
*	Part 1 - How many units remain after fully reacting the polymer *
* 		you scanned?						*
* 									*
************************************************************************/


use std::fs::read_to_string;


fn unit_react(unit_a: char, unit_b: char) -> bool {
    
    /* Test if two units react and annihilate themselves */
    
    /* Convert the chars to numerical values */
    let diff = unit_a as i32 - unit_b as i32;
    
    return i32::abs(diff) == 32;
}


fn is_react_possible(polymer: &str) -> bool {
    
    /* Check if the polymer could react further. */
    
    /* Convert the polymer to a vector of chars */
    let poly: Vec<char> = polymer.chars().collect();
    
    /* Iterate over the polymer */
    for i in 1..poly.len() {
	
	/* If the last char and current one react the polymer can be reduced */
	if unit_react(poly[i-1], poly[i]) {
	    return true
	}
    }
    
    return false
}


fn poly_react(polymer: &str) -> String {
    
    /* Run a polymer reaction where units cancel out if they are the capital
    and lowercase of the same letter. */
    
    /* Convert the polymer to a vector of chars */
    let mut old_poly = polymer.chars().peekable();
    
    let mut new_poly = String::from("");
    
    while let Some(char) = old_poly.next() {
	
	let next_char = old_poly.peek();
	
	/* Test if the current string and the next string annihilate */
	if !next_char.is_none() && unit_react(char, *next_char.unwrap()) {
	    old_poly.nth(0);
	    continue;
	} 
	
	/* Add the unreacted chr to the new polymer */
	new_poly.push(char);
    }

    return new_poly;
}


fn multi_poly_react(polymer: &str) -> String {
    
    /* Reduce a polymer to its most basic form. */
    
    let mut new_poly = String::from(polymer);
    
    while is_react_possible(&new_poly) {
	
	new_poly = poly_react(&new_poly);
    }
    
    return new_poly;
}


fn main() {
    
    /* Read the file and extract the contents as one string. */
    let file_buf = read_to_string("./data/input.txt").unwrap();
    let poly_chain = file_buf.trim();
    
    /* Fully react the polymer */ 
    let reacted_poly = multi_poly_react(poly_chain);
    
    println!("The final polymer is {}", reacted_poly);
    println!("The new polymer length is {}", reacted_poly.len());
}
