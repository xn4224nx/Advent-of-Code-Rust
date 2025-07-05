/*
 * --- Day 2: Inventory Management System ---
 *
 * You stop falling through time, catch your breath, and check the screen on
 * the device. "Destination reached. Current Year: 1518. Current Location:
 * North Pole Utility Closet 83N10." You made it! Now, to find those anomalies.
 *
 * Outside the utility closet, you hear footsteps and a voice. "...I'm not sure
 * either. But now that so many people have chimneys, maybe he could sneak in
 * that way?" Another voice responds, "Actually, we've been working on a new
 * kind of suit that would let him fit through tight spaces like that. But, I
 * heard that a few days ago, they lost the prototype fabric, the design plans,
 * everything! Nobody on the team can even seem to remember important details
 * of the project!"
 *
 * "Wouldn't they have had enough fabric to fill several boxes in the
 * warehouse? They'd be stored together, so the box IDs should be similar. Too
 * bad it would take forever to search the warehouse for two similar box
 * IDs..." They walk too far away to hear any more.
 *
 * Late at night, you sneak to the warehouse - who knows what kinds of
 * paradoxes you could cause if you were discovered - and use your fancy wrist
 * device to quickly scan every box and produce a list of the likely candidates
 * (your puzzle input).
 *
 * To make sure you didn't miss any, you scan the likely candidate boxes again,
 * counting the number that have an ID containing exactly two of any letter and
 * then separately counting those with exactly three of any letter. You can
 * multiply those two counts together to get a rudimentary checksum and compare
 * it to what your device predicts.
 *
 * For example, if you see the following box IDs:
 *
 *      -   abcdef contains no letters that appear exactly two or three times.
 *
 *      -   bababc contains two a and three b, so it counts for both.
 *
 *      -   abbcde contains two b, but no letter appears exactly three times.
 *
 *      -   abcccd contains three c, but no letter appears exactly two times.
 *
 *      -   aabcdd contains two a and two d, but it only counts once.
 *
 *      -   abcdee contains two e.
 *
 *      -   ababab contains three a and three b, but it only counts once.
 *
 * Of these box IDs, four of them contain a letter which appears exactly twice,
 * and three of them contain a letter which appears exactly three times.
 * Multiplying these together produces a checksum of 4 * 3 = 12.
 *
 * PART 1:  What is the checksum for your list of box IDs?
 */

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct IMS {
    pub boxes: Vec<Vec<char>>,
}

#[derive(PartialEq, Debug)]
pub enum Multiples {
    Nothing,
    Duplicate,
    Triplicate,
    Both,
}

impl IMS {
    pub fn new(box_file: &str) -> Self {
        let mut boxes = Vec::new();
        let mut buffer = String::new();

        /* Load the file */
        let file = File::open(box_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Read the file line by line. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            boxes.push(buffer.trim().chars().collect());
            buffer.clear();
        }
        return IMS { boxes };
    }

    /// What kind of multiples are inside the box
    pub fn box_check(&self, box_idx: usize) -> Multiples {
        let mut letter_cnt: HashMap<char, usize> = HashMap::new();

        /* Count the occurances of each char in the box */
        for letter in self.boxes[box_idx].iter() {
            letter_cnt
                .entry(*letter)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        let counts: Vec<usize> = letter_cnt.drain().map(|(_, y)| y).collect();
        let triples = counts.contains(&3);
        let doubles = counts.contains(&2);

        return if triples && doubles {
            Multiples::Both
        } else if doubles {
            Multiples::Duplicate
        } else if triples {
            Multiples::Triplicate
        } else {
            Multiples::Nothing
        };
    }

    /// Count the number of boxes with duplicates & triplicates and return
    /// the product as a checksum.
    pub fn checksum(&self) -> usize {
        let mut duplicates = 0;
        let mut triplicates = 0;

        for box_idx in 0..self.boxes.len() {
            match self.box_check(box_idx) {
                Multiples::Nothing => {}
                Multiples::Duplicate => duplicates += 1,
                Multiples::Triplicate => triplicates += 1,
                Multiples::Both => {
                    duplicates += 1;
                    triplicates += 1;
                }
            }
        }
        return duplicates * triplicates;
    }
}

fn main() {
    let utility_closet = IMS::new("./data/input_0.txt");
    println!("Part 1 = {}", utility_closet.checksum());
}
