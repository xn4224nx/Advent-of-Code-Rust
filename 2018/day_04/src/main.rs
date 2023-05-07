/************************************************************************
*									*
*	--- Day 4: Repose Record ---					*
*									*
*	Part 1 - Find the guard that has the most minutes asleep. 	*
* 		What minute does that guard spend asleep the most? 	*
* 		What is the ID of the guard you chose multiplied by 	*
* 		the minute you chose?					*
*									*
* 	Part 2 - Of all guards, which guard is most frequently asleep 	*
* 		on the same minute? What is the ID of the guard you 	*
* 		chose multiplied by the minute you chose?		*
* 									*
************************************************************************/

/* Accessing The Files */
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/* Parse The Data */
use regex::Regex;
use std::collections::HashMap;

/* Message Structure */
#[derive(Debug)]
struct Msg {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    txt: String,
}

impl Msg {
    
    fn print(&self) {
	println!("[{:04}-{:02}-{:02} {:02}:{:02}] {}", self.year, self.month, 
	    self.day, 
	    self.hour, 
	    self.min, 
	    self.txt);
    }
    
    fn sleep_time(&self, falls: &Msg) -> Vec<u32> {
	
	/* Calculate the time the guard is asleep,
	 * only in the hour after midnight.  */
	let mut fall_min = 0;
	let mut wake_min = 0;
	let mut sleep_mins = vec![];
	
	/* Catch the guard waking up after 1am. */
	if self.hour > 1 {
	    wake_min = 60;
	} else {
	    wake_min = self.min;
	}
	
	/* Catch if the guard starting sleeping before midnight. */
	if falls.hour != 0 {
	    fall_min = 0;
	} else {
	    fall_min = falls.min;
	}
	
	/* Create a vector of the minutes the guard was asleep. */
	for i in fall_min..wake_min {
	    sleep_mins.push(i)
	}
	
	return sleep_mins;
    }
}


fn vector_mode(numbers: Vec<i32>) -> (i32, i32) {
    /* Calculate the mode of a vector of numbers and 
     * return the mode and frequency. */
    
    /* If the vector is empty return set values of 0,0 */
    if numbers.len() < 1 {
	return (0,0)
    }
    
    
    /* Put the elements of the vector into a hash map */
    let mut numbers_freq = HashMap::new();
    
    for num in &numbers {
        let count = numbers_freq.entry(num).or_insert(1);
        *count += 1;
    }
    
    /* Extract the Mode */
    let mode = numbers_freq.iter().max_by_key(|entry | entry.1).unwrap();
    return (**mode.0, *mode.1);
}


fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    
    /* Open the file as read only. */
    let file = File::open(filename)?;
    
    /* An iterator to the reader of the lines of the file. */
    return Ok(io::BufReader::new(file).lines());
}

fn main() {
        
    /* Storage for the claim data */
    let mut msg_rec: Vec<Msg> = vec![];
    
    /* Regex to extract the message data */
    let re_msg = Regex::new(
	    r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\]\s(.*)").unwrap();
    
    /* Access the data file. */
    if let Ok(lines) = read_lines("./data/input.txt") {
	
	/* Read the data file line by line. */
	for line in lines {
	    if let Ok(parsed_line) = line {
		
		/* Show the raw line */    
		//println!("{}", parsed_line);
		
		/* Parse the message text. */
		let cap_groups = re_msg.captures(&parsed_line).unwrap();
		
		/* Put the parsed data into a structure */
		let tmp_msg = Msg {
		    year: cap_groups.get(1)
				    .unwrap()
				    .as_str()
				    .parse::<u32>()
				    .unwrap(),

		    month: cap_groups.get(2)
				    .unwrap()
				    .as_str()
				    .parse::<u32>()
				    .unwrap(),		    

		    day: cap_groups.get(3)
				    .unwrap()
				    .as_str()
				    .parse::<u32>()
				    .unwrap(),
		    
		    hour: cap_groups.get(4)
				    .unwrap()
				    .as_str()
				    .parse::<u32>()
				    .unwrap(),

		    min: cap_groups.get(5)
				    .unwrap()
				    .as_str()
				    .parse::<u32>()
				    .unwrap(),

		    txt: cap_groups.get(6)
				    .unwrap()
				    .as_str()
				    .to_string(),
		};
		
		/* Show the parsed message */
		//println!("{:?}", tmp_msg);

		/* Save the message data*/
		msg_rec.push(tmp_msg)
	    }
	}
    }

    
    /* Order the vector of message structures */
    msg_rec.sort_by(|a, b| if a.year != b.year {
	    
	    a.year.partial_cmp(&b.year).unwrap()
	    
	} else if a.month != b.month {
	    
	    a.month.partial_cmp(&b.month).unwrap()
	    
	} else if a.day != b.day {
	    
	    a.day.partial_cmp(&b.day).unwrap()
	    
	} else if a.hour != b.hour {
	    
	    a.hour.partial_cmp(&b.hour).unwrap()
	    
	} else if a.min != b.min {
	    
	    a.min.partial_cmp(&b.min).unwrap()
	
	} else {
	    
	    a.txt.partial_cmp(&b.txt).unwrap()
    });

    /* Storage for when the guards are asleep. */
    let mut sleep_rec: HashMap<i32, Vec<i32>> = HashMap::new();
    
    /* Storage for the total time asleep per guard */
    let mut guard_sleep_rec: HashMap<i32, i32> = HashMap::new();
    
    
    /* Add every minute in the hour as a key in the map */
    for i in 0..60 {
	sleep_rec.insert(i, vec![]);
    };

    
    /* Active Guard Statistics */
    let mut active_guard = 0;
    let mut sleep_msg = 0;
    
    let re_guard = Regex::new(r"\d+").unwrap();

    for i in 0..msg_rec.len() {
	
	/* Detect a new guard on shift. */
	if msg_rec[i].txt.contains("Guard") {

	    /* Extract the active guard id */
	    let guard = re_guard.find(&msg_rec[i].txt).unwrap().as_str();
	    
	    /* Set the current guard */
	    active_guard = guard.parse::<i32>().unwrap();
	    
	    //print!("{} ", active_guard.unwrap());
	    
	} else if msg_rec[i].txt.contains("wakes") {
	    
	    /* Calculate the minutes the guard was asleep */
	    let sleep_mins = msg_rec[i].sleep_time(&msg_rec[sleep_msg]);

	    /* Add in the sleeping hours to the hashmap. */
	    for min in &sleep_mins {
		
		sleep_rec.entry(*min as i32)
			.or_insert(Vec::new())
			.push(active_guard); 
	    }
	    
	    /* Save the total time asleep. */
	    guard_sleep_rec.entry(active_guard)
			.and_modify(|guard| *guard += sleep_mins.len() as i32)
			.or_insert(sleep_mins.len() as i32);
	    
	} else if msg_rec[i].txt.contains("falls") {
	    
	    /* save the location of the message of falling asleep. */
	    sleep_msg = i;
	}

    }
    
    
    /* Find the guard that has the most minutes asleep. */
    let sleepiest_guard = guard_sleep_rec
			    .iter().max_by_key(|entry | entry.1).unwrap().0;
    
    println!("Part 1 - The Guard that sleeps the most is: #{}", 
		    sleepiest_guard);
    
    
    /* Find the minute that they are most likely to be asleep. */
    let mut max_times_asleep = 0;
    let mut best_min = 0;

    /* For each minute count the guard that is asleep the most. */
    let mut max_times_asleep_in_a_min = 0;
    let mut cons_asleep_guard = 0;
    let mut best_min_p2 = 0;	
    
    for min in sleep_rec {
	
	/* Count how many times the sleepy guard was asleep in this minute */
	let times_asleep = min.1.iter()
				.filter(|&n| n == sleepiest_guard).count();
	
	if times_asleep > max_times_asleep {
	    max_times_asleep = times_asleep;
	    best_min = min.0;
	}
	
	/* For each minute count the guard that is asleep the most. */
	let sleep_mode = vector_mode(min.1);
	
	if sleep_mode.1 > max_times_asleep_in_a_min {
	    max_times_asleep_in_a_min = sleep_mode.1;
	    cons_asleep_guard = sleep_mode.0;
	    best_min_p2 = min.0;
	}  
		
    }
    
    println!("Part 1 - The minute the guard sleeps the most is {}", best_min);
    println!("Part 1 - The product is {}", best_min * sleepiest_guard);
    
    println!("Part 2- The most consistent guard is: #{}", cons_asleep_guard);
    println!("Part 2- The minute the guard sleeps the is {}", best_min_p2);
    println!("Part 2 - The product is {}", best_min_p2 * cons_asleep_guard);
    
}
