/*
 * --- Day 5: Alchemical Reduction ---
 *
 * You've managed to sneak in to the prototype suit manufacturing lab. The Elves
 * are making decent progress, but are still struggling with the suit's size
 * reduction capabilities.
 *
 * While the very latest in 1518 alchemical technology might have solved their
 * problem eventually, you can do better. You scan the chemical composition of
 * the suit's material and discover that it is formed by extremely long polymers
 * (one of which is available as your puzzle input).
 *
 * The polymer is formed by smaller units which, when triggered, react with each
 * other such that two adjacent units of the same type and opposite polarity are
 * destroyed. Units' types are represented by letters; units' polarity is
 * represented by capitalization. For instance, r and R are units with the same
 * type but opposite polarity, whereas r and s are entirely different types and
 * do not react.
 *
 * For example:
 *
 *      -   In aA, a and A react, leaving nothing behind.
 *
 *      -   In abBA, bB destroys itself, leaving aA. As above, this then
 *          destroys itself, leaving nothing.
 *
 *      -   In abAB, no two adjacent units are of the same type, and so nothing
 *          happens.
 *
 *      -   In aabAAB, even though aa and AA are of the same type, their
 *          polarities match, and so nothing happens.
 *
 * Now, consider a larger example, dabAcCaCBAcCcaDA:
 *
 *      dabAcCaCBAcCcaDA    The first 'cC' is removed.
 *
 *      dabAaCBAcCcaDA      This creates 'Aa', which is removed.
 *
 *      dabCBAcCcaDA        Either 'cC' or 'Cc' are removed (the result is the
 *                          same).
 *
 *      dabCBAcaDA          No further actions can be taken.
 *
 * After all possible reactions, the resulting polymer contains 10 units.
 *
 * PART 1:  How many units remain after fully reacting the polymer you scanned?
 *          (Note: in this puzzle and others, the input is large; if you copy/
 *          paste your input, make sure you get the whole thing.)
 *
 * Time to improve the polymer.
 *
 * One of the unit types is causing problems; it's preventing the polymer from
 * collapsing as much as it should. Your goal is to figure out which unit type is
 * causing the most problems, remove all instances of it (regardless of
 * polarity), fully react the remaining polymer, and measure its length.
 *
 * For example, again using the polymer dabAcCaCBAcCcaDA from above:
 *
 *      -   Removing all A/a units produces dbcCCBcCcD. Fully reacting this
 *          polymer produces dbCBcD, which has length 6.
 *
 *      -   Removing all B/b units produces daAcCaCAcCcaDA. Fully reacting this
 *          polymer produces daCAcaDA, which has length 8.
 *
 *      -   Removing all C/c units produces dabAaBAaDA. Fully reacting this
 *          polymer produces daDA, which has length 4.
 *
 *      -   Removing all D/d units produces abAcCaCBAcCcaA. Fully reacting this
 *          polymer produces abCBAc, which has length 6.
 *
 * In this example, removing all C/c units was best, producing the answer 4.
 *
 * PART 2:  What is the length of the shortest polymer you can produce by
 *          removing all units of exactly one type and fully reacting the result?
 */

use std::fs::read_to_string;

pub struct Polymer {
    pub units: Vec<char>,
}

impl Polymer {
    pub fn new(element_file: &str) -> Self {
        Polymer {
            units: read_to_string(element_file)
                .unwrap()
                .trim()
                .chars()
                .collect(),
        }
    }

    /// What is the length of the polymer after commpressing it until no more
    /// removals can be made.
    pub fn compressed_len(&self, rm_char: char) -> usize {
        let mut rm_units = vec![true; self.units.len()];

        /* Knock out certain chars. */
        let low = rm_char.to_lowercase().next().unwrap();
        let hig = rm_char.to_uppercase().next().unwrap();

        for u_idx in 0..self.units.len() {
            if low == self.units[u_idx] || hig == self.units[u_idx] {
                rm_units[u_idx] = false;
            }
        }

        /* Find adjacent elements that are the same letter but different case. */
        loop {
            let mut c_idx = 0;
            let mut n_idx = 1;
            let mut change_made = false;

            /* Iterate over the polymer and remove units. */
            while c_idx < self.units.len() && n_idx < self.units.len() {
                /* Move the indexes to find the next chars to compare */
                if !rm_units[c_idx] {
                    c_idx += 1;
                    continue;
                }

                /* Make sure a value is not compared to itself. */
                if c_idx == n_idx || !rm_units[n_idx] {
                    n_idx += 1;
                    continue;
                }

                /* See if the pair of units should be removed. */
                if (self.units[c_idx] as i32 - self.units[n_idx] as i32).abs()
                    == ('A' as i32 - 'a' as i32).abs()
                {
                    change_made = true;

                    /* Mark these units as having been removed. */
                    rm_units[c_idx] = false;
                    rm_units[n_idx] = false;

                    /* Prepare for the next check. */
                    c_idx += 1;
                    n_idx += 1;
                }

                /* Move onto the next pair to check. */
                c_idx += 1;
                n_idx += 1;
            }

            /* If no modifications have been made, stop looking. */
            if !change_made {
                break;
            }
        }

        /* Return the number of remaining units. */
        return rm_units.iter().filter(|x| **x).count();
    }

    /// Remove certain units from the polymer and see the shortest one that can
    /// be found is.
    pub fn compressed_rm_len(&self) -> usize {
        let mut min_found = usize::MAX;

        for rm_char in vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ]
        .into_iter()
        {
            let rm_comp_len = self.compressed_len(rm_char);
            if rm_comp_len < min_found {
                min_found = rm_comp_len;
            }
        }
        return min_found;
    }
}

fn main() {
    let suit_chem = Polymer::new("./data/input_0.txt");
    println!(
        "Part 1 = {}\nPart 2 = {}\n",
        suit_chem.compressed_len(' '),
        suit_chem.compressed_rm_len()
    );
}
