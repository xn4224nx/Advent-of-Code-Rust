/*
 * --- Day 16: Aunt Sue ---
 *
 * Your Aunt Sue has given you a wonderful gift, and you'd like to send her a
 * thank you card. However, there's a small problem: she signed it "From, Aunt
 * Sue".
 *
 * You have 500 Aunts named "Sue".
 *
 * So, to avoid sending the card to the wrong person, you need to figure out
 * which Aunt Sue (which you conveniently number 1 to 500, for sanity) gave you
 * the gift. You open the present and, as luck would have it, good ol' Aunt Sue
 * got you a My First Crime Scene Analysis Machine! Just what you wanted. Or
 * needed, as the case may be.
 *
 * The My First Crime Scene Analysis Machine (MFCSAM for short) can detect a few
 * specific compounds in a given sample, as well as how many distinct kinds of
 * those compounds there are. According to the instructions, these are what the
 * MFCSAM can detect:
 *
 *  -   children, by human DNA age analysis.
 *
 *  -   cats. It doesn't differentiate individual breeds.
 *
 *  -   Several seemingly random breeds of dog: samoyeds, pomeranians, akitas,
 *      and vizslas.
 *
 *  -   goldfish. No other kinds of fish.
 *
 *  -   trees, all in one group.
 *
 *  -   cars, presumably by exhaust or gasoline or something.
 *
 *  -   perfumes, which is handy, since many of your Aunts Sue wear a few kinds.
 *
 * In fact, many of your Aunts Sue have many of these. You put the wrapping from
 * the gift into the MFCSAM. It beeps inquisitively at you a few times and then
 * prints out a message on ticker tape:
 *
 *      children: 3
 *      cats: 7
 *      samoyeds: 2
 *      pomeranians: 3
 *      akitas: 0
 *      vizslas: 0
 *      goldfish: 5
 *      trees: 3
 *      cars: 2
 *      perfumes: 1
 *
 * You make a list of the things you can remember about each Aunt Sue. Things
 * missing from your list aren't zero - you simply don't remember the value.
 *
 * PART 1:  What is the number of the Sue that got you the gift?
 */

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Read the data file and return a vector of aunt characteristics
pub fn read_aunt_data(data_file: &str) -> Vec<HashMap<String, u32>> {
    let mut all_aunt_data = Vec::new();

    /* Open the data file. */
    let file = File::open(data_file).unwrap();
    let mut fp = BufReader::new(file);

    let re_comp = Regex::new(r"([a-z]+): (\d+)").unwrap();

    /* Extract the data on each line to a HashMap for just that line. */
    let mut buffer = String::new();
    while fp.read_line(&mut buffer).unwrap() > 0 {
        let mut aunts = HashMap::new();

        /* Extract the compound and value pairs in the line. */
        for (_, [comp, val]) in re_comp.captures_iter(&buffer).map(|x| x.extract()) {
            aunts.insert(String::from(comp), val.parse::<u32>().unwrap());
        }

        all_aunt_data.push(aunts);
        buffer.clear();
    }

    return all_aunt_data;
}

/// Determine if an aunt could match another aunt.
pub fn could_aunt_match(
    true_aunt: &HashMap<String, u32>,
    posib_aunt: &HashMap<String, u32>,
) -> bool {
    for (compound, value) in posib_aunt.iter() {
        if true_aunt.contains_key(compound) && true_aunt[compound] != *value {
            return false;
        }
    }

    return true;
}

/// Find the index of the most likely match to the true aunt
pub fn find_true_aunt_index(
    true_aunt: &HashMap<String, u32>,
    aunt_data: &Vec<HashMap<String, u32>>,
) -> usize {
    for (idx, aunt) in aunt_data.iter().enumerate() {
        if could_aunt_match(true_aunt, aunt) {
            return idx + 1;
        }
    }
    panic!("Valid aunt not found!");
}

fn main() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    let all_aunts = read_aunt_data("./data/input.txt");

    println!("Part 1 = {}", find_true_aunt_index(true_aunt, &all_aunts));
}
