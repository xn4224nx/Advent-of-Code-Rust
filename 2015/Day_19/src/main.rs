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

/// Read the replacement molecules and initial chemical data.
pub fn read_molc_replacements(datafile: &str) -> (Vec<(String, String)>, String) {
    return (Vec::new(), String::new());
}

/// Count the number of distinct chemicals that can be made.
pub fn cnt_distinct_chems(molc_reps: Vec<(String, String)>, chem: String) -> usize {
    0
}

fn main() {}
