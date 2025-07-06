/*
 * --- Day 3: No Matter How You Slice It ---
 *
 * The Elves managed to locate the chimney-squeeze prototype fabric for Santa's
 * suit (thanks to someone who helpfully wrote its box IDs on the wall of the
 * warehouse in the middle of the night). Unfortunately, anomalies are still
 * affecting them - nobody can even agree on how to cut the fabric.
 *
 * The whole piece of fabric they're working on is a very large square - at
 * least 1000 inches on each side.
 *
 * Each Elf has made a claim about which area of fabric would be ideal for
 * Santa's suit. All claims have an ID and consist of a single rectangle with
 * edges parallel to the edges of the fabric. Each claim's rectangle is defined
 * as follows:
 *
 *      -   The number of inches between the left edge of the fabric and the
 *          left edge of the rectangle.
 *
 *      -   The number of inches between the top edge of the fabric and the top
 *          edge of the rectangle.
 *
 *      -   The width of the rectangle in inches.
 *
 *      -   The height of the rectangle in inches.
 *
 * A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3
 * inches from the left edge, 2 inches from the top edge, 5 inches wide, and 4
 * inches tall. Visually, it claims the square inches of fabric represented by #
 * (and ignores the square inches of fabric represented by .) in the diagram
 * below:
 *
 *      ...........
 *      ...........
 *      ...#####...
 *      ...#####...
 *      ...#####...
 *      ...#####...
 *      ...........
 *      ...........
 *      ...........
 *
 * The problem is that many of the claims overlap, causing two or more claims to
 * cover part of the same areas. For example, consider the following claims:
 *
 *      #1 @ 1,3: 4x4
 *      #2 @ 3,1: 4x4
 *      #3 @ 5,5: 2x2
 *
 * Visually, these claim the following areas:
 *
 *      ........
 *      ...2222.
 *      ...2222.
 *      .11XX22.
 *      .11XX22.
 *      .111133.
 *      .111133.
 *      ........
 *
 * The four square inches marked with X are claimed by both 1 and 2. (Claim 3,
 * while adjacent to the others, does not overlap either of them.)
 *
 * PART 1:  If the Elves all proceed with their own plans, none of them will
 *          have enough fabric. How many square inches of fabric are within two
 *          or more claims?
 *
 * Amidst the chaos, you notice that exactly one claim doesn't overlap by even
 * a single square inch of fabric with any other claim. If you can somehow draw
 * attention to it, maybe the Elves will be able to make Santa's suit after
 * all!
 *
 * For example, in the claims above, only claim 3 is intact after all claims
 * are made.
 *
 * PART 2:  What is the ID of the only claim that doesn't overlap?
 */

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Fabric {
    pub claims: HashMap<(usize, usize), HashSet<usize>>,
    pub claim_idxs: HashSet<usize>,
}

impl Fabric {
    pub fn new(claims_data: &str) -> Self {
        let re = Regex::new(r"#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
        let mut claims = HashMap::new();
        let mut claim_idxs = HashSet::new();
        let mut buffer = String::new();

        /* Open the file */
        let file = File::open(claims_data).unwrap();
        let mut fs = BufReader::new(file);

        /* Read the file line by line. */
        while fs.read_line(&mut buffer).unwrap() > 0 {
            let raw_claim = re.captures(&buffer);

            /* Extract the claim details or skip the line. */
            let Some(claim) = raw_claim else {
                println!("Line could not be extracted: '{}'", buffer);
                buffer.clear();
                continue;
            };

            /* Parse the numbers in the claim. */
            let cl_dat: Vec<usize> = (1..claim.len())
                .map(|x| claim[x].parse::<usize>().unwrap())
                .collect();

            /* Add every sq covered by this claim to the claims record. */
            for x_idx in cl_dat[1]..cl_dat[1] + cl_dat[3] {
                for y_idx in cl_dat[2]..cl_dat[2] + cl_dat[4] {
                    claims
                        .entry((x_idx, y_idx))
                        .and_modify(|x: &mut HashSet<usize>| {
                            x.insert(cl_dat[0]);
                        })
                        .or_insert(HashSet::from([cl_dat[0]]));
                }
            }

            /* Make a record of the claim indexs. */
            claim_idxs.insert(cl_dat[0]);

            buffer.clear();
        }
        return Fabric { claims, claim_idxs };
    }

    /// How many square inches of  fabric are overlapping from all the claims.
    pub fn overlapping_sqrs(&self) -> usize {
        return self.claims.values().filter(|x| x.len() > 1).count();
    }

    /// Find the index of the claim the doesn't overlap with any other.
    pub fn find_non_overlapping_claim(&self) -> usize {
        let mut overlapping: HashSet<usize> = HashSet::new();

        /* Make a record of each index that overlaps with another.  */
        for clm_idxes in self.claims.values() {
            if clm_idxes.len() > 1 {
                overlapping.extend(clm_idxes);
            }
        }

        /* Compare the overlaping indexs with all indexes. */
        return *self.claim_idxs.difference(&overlapping).next().unwrap();
    }
}

fn main() {
    let santas_suit = Fabric::new("./data/input_0.txt");
    println!("Part 1 = {}", santas_suit.overlapping_sqrs());
    println!("Part 2 = {}", santas_suit.find_non_overlapping_claim());
}
