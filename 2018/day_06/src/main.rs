/************************************************************************
*									*
*	--- Day 6: Chronal Coordinates ---				*
*									*
*	Part 1 - What is the size of the largest area that isn't 	*
* 		infinite?						*
*									*
* 									*
************************************************************************/

use std::collections::HashMap;
use std::collections::HashSet;

/* Load and read the file. */
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/* Parse The Data */
use regex::Regex;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines())
}


fn find_enclose_rect(points: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    /* Find the two points of a rectangle that encloses all the points. */
    
    /* The storage for the max and min values */
    let mut x_max = i32::MIN;
    let mut y_max = i32::MIN;
    let mut x_min = i32::MAX;
    let mut y_min = i32::MAX;
    
    /* Loop over the points. */
    for (x, y) in points.iter() {
	
	/* Find the largest and smallest x values */
	if x > &x_max {
	    x_max = *x;
	    
	} else if x < &x_min {
	    x_min = *x;
	}
	
	/* Find the largest and smallest y values */
	if y > &y_max {
	    y_max = *y;
	    
	} else if y < &y_min {
	    y_min = *y;
	}
    }
    
    /* Return the two coordinates of the rectangle */
    return vec![(x_min - 4, y_min - 4), (x_max + 4, y_max + 4)]
}


fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    /* Find the Manhattan Distance between points a and b. */
    return (a.0 - b.0).abs() + (a.1 - b.1).abs()
}


fn find_closest_point(rectangle: &Vec<(i32, i32)>, points: &Vec<(i32, i32)>) 
-> (Vec<(i32, (i32, i32))>, HashSet<i32>) {
    /* For each coordinate in the rectangle find the closest point */
    
    /* Store the index of the closest coordinate from `points`. */
    let mut close_point: Vec<(i32, (i32, i32))> = vec![];
    
    /* Store details of the points that have infinite area */
    let mut inf_points: HashSet<i32> = HashSet::from([-1]);
    
    /* For each point in the rectangle.*/
    for x in rectangle[0].0..rectangle[1].0 {
	for y in rectangle[0].1..rectangle[1].1 {
	    
	    /* Record details of the closest point. */
	    let mut closest_pnt_idx: i32 = -1;
	    let mut min_dist = i32::MAX;
	    
	    /* for each point */
	    for (i, p) in points.iter().enumerate() {
		
		/* Find the distance between the point & rectangle coordinate*/
		let dist = manhattan_distance(*p, (x,y));
		
		/* This point is closest than all others */
		if dist < min_dist {
		    closest_pnt_idx = i as i32;
		    min_dist = dist;
		
		/* There are two points that are the same distance. */
		} else if dist == min_dist {
		    closest_pnt_idx = -1;
		} 
	    } 
	    
	    /* Test for points on the edge of the rectangle. */
	    if (x == rectangle[0].0) | (y == rectangle[0].1) | 
		(x == (rectangle[1].0 - 1)) | (y == (rectangle[1].1 - 1)) {
		
		inf_points.insert(closest_pnt_idx);
	    } 
	    
	    /* Save details of the closest point. */
	    close_point.push((closest_pnt_idx, (x, y)));
	}
    }
    
    return (close_point, inf_points);
}


fn finite_point_areas(coord_allocation: &Vec<(i32, (i32, i32))>, 
			inf_points: &HashSet<i32>) -> HashMap<i32, i32> {
    /* Create a Hashmap of the points and the areas. */
    
    /* Hashmap of the total finite areas of points */
    let mut pnt_areas: HashMap<i32, i32> = HashMap::new();    
    
    /* Iterate over the coordinates and count the allocations. */
    for (point, rect_coord) in coord_allocation {
	
	if inf_points.contains(point) {
	    continue;
	}
	
	/* Save the points */
	pnt_areas.entry(*point).and_modify(|area| *area += 1).or_insert(1);
    }

    return pnt_areas;
}


fn main() {
    
    /* Storage for the coordinates. */
    let mut points: Vec<(i32, i32)> = vec![];
    
    /* Regex to extract the coordinate data */
    let re_coords = Regex::new(r"(-?\d+), (-?\d+)").unwrap();
    
    /* Load the coordinates from disk. */
    if let Ok(lines) = read_lines("./data/input.txt") {
        
        /* Consume the iterator and return an optional string. */
        for line in lines {
            
            if let Ok(raw_line) = line {
                
		/* Parse the coordinatates into a data structure. */
		let raw_coords = re_coords.captures(&raw_line).unwrap();
		
		let x = raw_coords.get(1).unwrap().as_str()
						    .parse::<i32>().unwrap();
		
		let y = raw_coords.get(2).unwrap().as_str()
						    .parse::<i32>().unwrap();
		
		points.push((x, y));
            }
        }
    }
    
    /* Find the rectangle that encloses all the points. */
    let rectangle = find_enclose_rect(&points);

    /* Determine the closest point for each coordinate in the rectangle.  */
    let (closest_points, inf_points) = find_closest_point(&rectangle, &points);

    /* Determine the non-infinite areas in the rectangle. */
    let areas = finite_point_areas(&closest_points, &inf_points);
    
    /* Find the largest non-infinite area */
    let max = areas.iter().max_by_key(|x| x.1).unwrap();
    
    println!("Part 1:The largest non-infinite area is {}", max.1);
}
