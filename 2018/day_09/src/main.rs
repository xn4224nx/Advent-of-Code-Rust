/************************************************************************
*									                                    *
*	--- Day 9: Marble Mania --- 					                    *
*									                                    *
*	Part 1 - What is the winning Elf's score?       		            *
*									                                    *
************************************************************************/


fn main() {
    
    let players = 419;
    let last_marble = 71052;
    
    /* Define the vector to hold the circle of marbles. */
    let mut circle = vec![0];
    
    /* Keep a record of the elves scores */
    let mut scores = vec![0; players];
    
    let mut curr_marb = 1;
    let mut curr_player = 1;
    
    /* Place the marbles */
    for i in 1..last_marble+1 {

        let circum = circle.len() as usize;
        
        /* Test if the current elf gets a score */
        if i % 23 == 0 {
            
            /* Add the marble to the current players score */
            scores[curr_player] += i;
            
            /* Remove the marble 7 back and add it to the score */
            let rem_idx = (curr_marb + circum - 6) % circum; 
            scores[curr_player] += circle.remove(rem_idx);
            
            /* Change the current marble to the one after */
            curr_marb = (rem_idx-1).try_into().unwrap();
            
            //print!("{:?}",  circle);
            //println!(" {:?}",  circle[curr_marb+1]);
            
            continue;
        }
        
        /* Work out where the new marble gets inserted */
        curr_marb = (curr_marb + 2) % circum;
        circle.insert((curr_marb + 1).try_into().unwrap(), i);
        
        /* Move onto the next player */
        curr_player = (curr_player + 1) % players;
        
        //print!("{:?}",  circle);
        //println!(" {:?}",  circle[curr_marb+1]);
    }

    println!("{}", scores.iter().max().unwrap());
}
