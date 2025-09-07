/*
 * --- Day 7: The Sum of Its Parts ---
 *
 * You find yourself standing on a snow-covered coastline; apparently, you
 * landed a little off course. The region is too hilly to see the North Pole
 * from here, but you do spot some Elves that seem to be trying to unpack
 * something that washed ashore. It's quite cold out, so you decide to risk
 * creating a paradox by asking them for directions.
 *
 * "Oh, are you the search party?" Somehow, you can understand whatever Elves
 * from the year 1018 speak; you assume it's Ancient Nordic Elvish. Could the
 * device on your wrist also be a translator? "Those clothes don't look very
 * warm; take this." They hand you a heavy coat.
 *
 * "We do need to find our way back to the North Pole, but we have higher
 * priorities at the moment. You see, believe it or not, this box contains
 * something that will solve all of Santa's transportation problems - at least,
 * that's what it looks like from the pictures in the instructions." It doesn't
 * seem like they can read whatever language it's in, but you can: "Sleigh kit.
 * Some assembly required."
 *
 * "'Sleigh'? What a wonderful name! You must help us assemble this 'sleigh'
 * at once!" They start excitedly pulling more parts out of the box.
 *
 * The instructions specify a series of steps and requirements about which steps
 * must be finished before others can begin (your puzzle input). Each step is
 * designated by a single letter. For example, suppose you have the following
 * instructions:
 *
 *      Step C must be finished before step A can begin.
 *      Step C must be finished before step F can begin.
 *      Step A must be finished before step B can begin.
 *      Step A must be finished before step D can begin.
 *      Step B must be finished before step E can begin.
 *      Step D must be finished before step E can begin.
 *      Step F must be finished before step E can begin.
 *
 * Visually, these requirements look like this:
 *
 *        -->A--->B--
 *       /    \      \
 *      C      -->D----->E
 *       \           /
 *        ---->F-----
 *
 * Your first goal is to determine the order in which the steps should be
 * completed. If more than one step is ready, choose the step which is first
 * alphabetically. In this example, the steps would be completed as follows:
 *
 *      -   Only C is available, and so it is done first.
 *
 *      -   Next, both A and F are available. A is first alphabetically, so
 *          it is done next.
 *
 *      -   Then, even though F was available earlier, steps B and D are now
 *          also available, and B is the first alphabetically of the three.
 *
 *      -   After that, only D and F are available. E is not available because
 *          only some of its prerequisites are complete. Therefore, D is
 *          completed next.
 *
 *      -   F is the only choice, so it is done next.
 *
 *      -   Finally, E is completed.
 *
 * So, in this example, the correct order is CABDFE.
 *
 * PART 1:  In what order should the steps in your instructions be completed?
 */

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct WordMaker {
    pub dependencies: HashMap<char, HashSet<char>>,
}

impl WordMaker {
    pub fn new(relations_file: &str) -> Self {
        let mut buffer = String::new();
        let mut dependencies: HashMap<char, HashSet<char>> = HashMap::new();

        /* Open the data file. */
        let file = File::open(relations_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Iterate over the file line by line. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            /* Convert the line to a vector of chars */
            let line: Vec<char> = buffer.chars().collect();

            /* Ensure both letters have entries. */
            if !dependencies.contains_key(&line[5]) {
                dependencies.insert(line[5], HashSet::new());
            };
            if !dependencies.contains_key(&line[36]) {
                dependencies.insert(line[36], HashSet::new());
            };

            /* Codify the letter dependancy */
            dependencies.get_mut(&line[36]).unwrap().insert(line[5]);
            buffer.clear();
        }
        return WordMaker { dependencies };
    }

    /// What will  be the first letter in the word be?
    pub fn first_letters(&self) -> Vec<char> {
        let mut f_letters = Vec::new();

        /* Find the letters with no dependancies. */
        for (letter, depends) in &self.dependencies {
            if depends.is_empty() {
                f_letters.push(*letter);
            }
        }
        f_letters.sort();
        return f_letters;
    }

    /// What would the word be if constructed by a single worker?
    pub fn single_worker(&self) -> String {
        let mut final_word = String::new();
        let mut used_words: HashSet<char> = HashSet::new();

        /* Pick the first letter in the word */
        let first_letter = self.first_letters()[0];
        final_word.push(first_letter);
        used_words.insert(first_letter);

        /* Pick the next letters. */
        while used_words.len() < self.dependencies.len() {
            let mut nxt_letters = Vec::new();

            /* Find the letters that are available. */
            for (letter, depends) in &self.dependencies {
                if !used_words.contains(letter) && depends.is_subset(&used_words) {
                    nxt_letters.push(*letter);
                }
            }

            /* Add the first available word alphabetically. */
            nxt_letters.sort();
            final_word.push(nxt_letters[0]);
            used_words.insert(nxt_letters[0]);
        }
        return final_word;
    }
}

fn main() {
    let sleigh_kit = WordMaker::new("./data/input_0.txt");
    println!("Part 1 = '{}'", sleigh_kit.single_worker());
}
