/*
 * --- Day 19: Medicine for Rudolph ---
 *
 * Rudolph the Red-Nosed Reindeer is sick! His nose isn't shining very
 * brightly, and he needs medicine.
 *
 * Red-Nosed Reindeer biology isn't similar to regular reindeer biology;
 * Rudolph is going to need custom-made medicine. Unfortunately, Red-Nosed
 * Reindeer chemistry isn't similar to regular reindeer chemistry, either.
 *
 * The North Pole is equipped with a Red-Nosed Reindeer nuclear fusion/fission
 * plant, capable of constructing any Red-Nosed Reindeer molecule you need. It
 * works by starting with some input molecule and then doing a series of
 * replacements, one per step, until it has the right molecule.
 *
 * However, the machine has to be calibrated before it can be used. Calibration
 * involves determining the number of molecules that can be generated in one
 * step from a given starting point.
 *
 * Your puzzle input describes all of the possible replacements and, at the
 * bottom, the medicine molecule for which you need to calibrate the machine.
 *
 * PART 1:  How many distinct molecules can be created after all the different
 *          ways you can do one replacement on the medicine molecule?
 */

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Read the replacement molecules and initial chemical data.
pub fn read_molc_replacements(datafile: &str) -> (Vec<(String, String)>, String) {
    let mut molc_reps = Vec::new();
    let mut chem = String::new();

    /* Open the file. */
    let file = File::open(datafile).unwrap();
    let mut fp = BufReader::new(file);

    /* Process the file line by line. */
    let mut buffer = String::new();
    while fp.read_line(&mut buffer).unwrap() > 0 {
        let line = buffer.as_str().trim();

        if line.contains(" => ") {
            let parts: Vec<&str> = line.split(" => ").collect();
            molc_reps.push((parts[0].to_string(), parts[1].to_string()));
        } else if line == "" {
            continue;
        } else {
            chem = line.to_string();
        }
        buffer.clear();
    }
    return (molc_reps, chem);
}

/// Count the number of distinct chemicals that can be made.
pub fn cnt_distinct_chems(molc_reps: Vec<(String, String)>, chem: String) -> usize {
    0
}

fn main() {}
