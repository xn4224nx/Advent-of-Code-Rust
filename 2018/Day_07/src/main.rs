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
 *
 * As you're about to begin construction, four of the Elves offer to help. "The
 * sun will set soon; it'll go faster if we work together." Now, you need to
 * account for multiple people working on steps simultaneously. If multiple
 * steps are available, workers should still begin them in alphabetical order.
 *
 * Each step takes 60 seconds plus an amount corresponding to its letter: A=1,
 * B=2, C=3, and so on. So, step A takes 60+1=61 seconds, while step Z takes
 * 60+26=86 seconds. No time is required between steps.
 *
 * To simplify things for the example, however, suppose you only have help from
 * one Elf (a total of two workers) and that each step takes 60 fewer seconds
 * (so that step A takes 1 second and step Z takes 26 seconds). Then, using the
 * same instructions as above, this is how each second would be spent:
 *
 * Second   Worker 1   Worker 2   Done
 *   0        C          .
 *   1        C          .
 *   2        C          .
 *   3        A          F       C
 *   4        B          F       CA
 *   5        B          F       CA
 *   6        D          F       CAB
 *   7        D          F       CAB
 *   8        D          F       CAB
 *   9        D          .       CABF
 *  10        E          .       CABFD
 *  11        E          .       CABFD
 *  12        E          .       CABFD
 *  13        E          .       CABFD
 *  14        E          .       CABFD
 *  15        .          .       CABFDE
 *
 * Each row represents one second of time. The Second column identifies how many
 * seconds have passed as of the beginning of that second. Each worker column
 * shows the step that worker is currently doing (or . if they are idle). The
 * Done column shows completed steps.
 *
 * Note that the order of the steps has changed; this is because steps now take
 * time to finish and multiple workers can begin multiple steps simultaneously.
 *
 * In this example, it would take 15 seconds for two workers to complete these
 * steps.
 *
 * PART 2:  With 5 workers and the 60+ second step durations described above,
 *          how long will it take to complete all of the steps?
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

    /// How long would it take to construct a word with multiple workers.
    pub fn multi_worker(&self, num_workers: usize, min_time: usize) -> (String, usize) {
        let mut time_taken = 0;
        let mut final_word = String::new();

        /* Keep track of what letter and time a worker in processing */
        let mut workers = vec![0; num_workers];
        let mut worker_task = vec![' '; num_workers];

        /* Keep track of where each letter is */
        let mut used_letters: HashSet<char> = HashSet::new();
        let mut processing_letters: HashSet<char> = HashSet::new();

        while used_letters.len() < self.dependencies.len() {
            let mut nxt_letters = Vec::new();

            /* Find the available letters. */
            for (letter, depends) in &self.dependencies {
                if !used_letters.contains(letter)
                    && !processing_letters.contains(letter)
                    && depends.is_subset(&used_letters)
                {
                    nxt_letters.push(*letter);
                }
            }

            /* The letters get assigned alphabetically. */
            nxt_letters.sort();
            nxt_letters.reverse();

            /* Give the letters to the free workers.  */
            for wrk_idx in 0..num_workers {
                if workers[wrk_idx] == 0 && !nxt_letters.is_empty() {
                    let assign_letter = nxt_letters.pop().unwrap();
                    processing_letters.insert(assign_letter);
                    workers[wrk_idx] = min_time + (assign_letter as usize) - ('A' as usize) + 1;
                    worker_task[wrk_idx] = assign_letter;
                }
            }

            /* How long will it take a worker to finish next? */
            let mut min_time = usize::MAX;
            for wrk_idx in 0..num_workers {
                if workers[wrk_idx] < min_time && workers[wrk_idx] != 0 {
                    min_time = workers[wrk_idx];
                }
            }

            /* Simulate time running and the letters being processed. */
            time_taken += min_time;
            for wrk_idx in 0..num_workers {
                if workers[wrk_idx] != 0 {
                    workers[wrk_idx] -= min_time;

                    /* A worker finishes the letter and it is added to the word */
                    if workers[wrk_idx] == 0 {
                        processing_letters.remove(&worker_task[wrk_idx]);
                        used_letters.insert(worker_task[wrk_idx]);
                        final_word.push(worker_task[wrk_idx]);
                        worker_task[wrk_idx] = ' ';
                    }
                }
            }
        }

        /* After the last letter has been assigned determine the final time. */
        return (final_word, time_taken);
    }
}

fn main() {
    let sleigh_kit = WordMaker::new("./data/input_0.txt");
    println!(
        "Part 1 = '{}'\nPart 2 = {}",
        sleigh_kit.multi_worker(1, 0).0,
        sleigh_kit.multi_worker(5, 60).1
    );
}
