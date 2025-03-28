/*
 * --- Day 16: Dragon Checksum ---
 *
 * You're done scanning this part of the network, but you've left traces of
 * your presence. You need to overwrite some disks with random-looking data to
 * cover your tracks and update the local security system with a new checksum
 * for those disks.
 *
 * For the data to not be suspicious, it needs to have certain properties;
 * purely random data will be detected as tampering. To generate appropriate
 * random data, you'll need to use a modified dragon curve.
 *
 * Start with an appropriate initial state (your puzzle input). Then, so long
 * as you don't have enough data yet to fill the disk, repeat the following
 * steps:
 *
 *      -   Call the data you have at this point "a".
 *      -   Make a copy of "a"; call this copy "b".
 *      -   Reverse the order of the characters in "b".
 *      -   In "b", replace all instances of 0 with 1 and all 1s with 0.
 *      -   The resulting data is "a", then a single 0, then "b".
 *
 * Repeat these steps until you have enough data to fill the desired disk.
 *
 * Once the data has been generated, you also need to create a checksum of that
 * data. Calculate the checksum only for the data that fits on the disk, even
 * if you generated more data than that in the previous step.
 *
 * The checksum for some given data is created by considering each non-
 * overlapping pair of characters in the input data. If the two characters
 * match (00 or 11), the next checksum character is a 1. If the characters do
 * not match (01 or 10), the next checksum character is a 0. This should
 * produce a new string which is exactly half as long as the original. If the
 * length of the checksum is even, repeat the process until you end up with a
 * checksum with an odd length.
 *
 * PART 1:  The first disk you have to fill has length 272. Using the initial
 *          state in your puzzle input, what is the correct checksum?
 */

pub struct BinaryBlob {
    pub data: Vec<u8>,
}

impl BinaryBlob {
    pub fn new(seed: &str) -> Self {
        return BinaryBlob {
            data: seed.chars().map(|x| x as u8 - 0x30).collect(),
        };
    }

    /// Reverse the order of the bits in the binary data
    pub fn reverse(&mut self) {
        self.data = self.data.iter().map(|x| *x).rev().collect();
    }

    /// Change all ones to zeros and vice versa in the data
    pub fn invert(&mut self) {
        for idx in 0..self.data.len() {
            self.data[idx] = if self.data[idx] == 0 { 1 } else { 0 };
        }
    }

    /// Expand the data according to the dragon curve algorithm
    pub fn dragon_curve(&mut self) {
        let a = self.data.clone();
        self.reverse();
        self.invert();
        self.data = [a, vec![0], self.data.clone()].concat();
    }

    /// Expand the blob of data to a given size of bits
    pub fn expand(&mut self, size_of_bits: usize) {
        while self.data.len() < size_of_bits {
            self.dragon_curve()
        }

        /* Cut the data down to the required length. */
        self.data = self.data[..size_of_bits].to_vec();
    }

    /// Calculate the checksum for the data in its current state
    pub fn checksum(&self) -> String {
        let mut chsm = self.data.clone();

        /* This only works with even lengths of data. */
        assert_eq!(self.data.len() % 2, 0);

        loop {
            let mut new_chsm = Vec::new();

            /* Examine each pair of bits in the data. */
            for idx in 0..chsm.len() {
                if idx % 2 == 0 {
                    new_chsm.push((chsm[idx] == chsm[idx + 1]) as u8)
                }
            }

            /* Overwrite the old with the new. */
            chsm = new_chsm;

            /* Iterrate until the checksum has an odd length. */
            if chsm.len() % 2 != 0 {
                break;
            }
        }
        return self.show(&chsm);
    }

    /// Return a human readable form of the binary blob.
    pub fn show(&self, sh_data: &Vec<u8>) -> String {
        sh_data
            .iter()
            .map(|x| match *x {
                0 => '0',
                1 => '1',
                _ => panic!("Character Not Recognised!"),
            })
            .collect::<String>()
    }

    /// Expand the data and then determine its checksum
    pub fn expanded_check(&mut self, size_of_bits: usize) -> String {
        self.expand(size_of_bits);
        return self.checksum();
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        BinaryBlob::new("10111011111001111").expanded_check(272)
    );
    println!(
        "Part 2 = {}",
        BinaryBlob::new("10111011111001111").expanded_check(35651584)
    );
}
