/*
 * --- Day 14: One-Time Pad ---
 *
 * In order to communicate securely with Santa while you're on this mission,
 * you've been using a one-time pad that you generate using a pre-agreed
 * algorithm. Unfortunately, you've run out of keys in your one-time pad, and
 * so you need to generate some more.
 *
 * To generate keys, you first get a stream of random data by taking the MD5 of
 * a pre-arranged salt (your puzzle input) and an increasing integer index
 * (starting with 0, and represented in decimal); the resulting MD5 hash should
 * be represented as a string of lowercase hexadecimal digits.
 *
 * However, not all of these MD5 hashes are keys, and you need 64 new keys for
 * your one-time pad. A hash is a key only if:
 *
 *  -   It contains three of the same character in a row, like 777. Only
 *      consider the first such triplet in a hash.
 *
 *  -   One of the next 1000 hashes in the stream contains that same character
 *      five times in a row, like 77777.
 *
 * Considering future hashes for five-of-a-kind sequences does not cause those
 * hashes to be skipped; instead, regardless of whether the current hash is a
 * key, always resume testing for keys starting with the very next hash.
 *
 * PART 1:  Given the actual salt in your puzzle input, what index produces
 *          your 64th one-time pad key?
 *
 * Of course, in order to make this process even more secure, you've also
 * implemented key stretching.
 *
 * Key stretching forces attackers to spend more time generating hashes.
 * Unfortunately, it forces everyone else to spend more time, too.
 *
 * To implement key stretching, whenever you generate a hash, before you use it,
 * you first find the MD5 hash of that hash, then the MD5 hash of that hash, and
 * so on, a total of 2016 additional hashings. Always use lowercase hexadecimal
 * representations of hashes.
 *
 * PART 2:  Given the actual salt in your puzzle input and using 2016 extra MD5
 *          calls of key stretching, what index now produces your 64th one-time
 *          pad key?
 */

use md5;
use std::collections::{HashMap, HashSet};

pub struct KeyGen {
    pub salt: String,
}

impl KeyGen {
    pub fn new(inti_salt: &str) -> Self {
        return KeyGen {
            salt: inti_salt.to_string(),
        };
    }

    /// Create a random data using the salt and an index
    pub fn stream(&self, index: usize) -> Vec<char> {
        let msg = format!("{}{}", self.salt, index.to_string());

        /* Calculate the hash digest. */
        let digest = format!("{:x}", md5::compute(msg.as_bytes()));

        /* Convert to a vector of characters. */
        return digest.chars().collect();
    }

    /// Strenghten the key by multiple hashings
    pub fn stretch(&self, data: &Vec<char>, iters: usize) -> Vec<char> {
        /* Convert the data to a string */
        let mut msg = data.iter().collect::<String>();

        /* Hash it multiple times. */
        for _ in 0..iters {
            msg = format!("{:x}", md5::compute(msg.as_bytes()));
        }

        /* Convert back to a vector of chars. */
        return msg.chars().collect();
    }

    /// Extract the chars of the triples and quintets in the string
    pub fn find_multiples(&self, data: Vec<char>) -> (Vec<char>, Vec<char>) {
        let mut triples = Vec::new();
        let mut quintets = Vec::new();

        for idx in 0..data.len() {
            if idx >= 2
                && triples.len() < 1
                && data[idx] == data[idx - 1]
                && data[idx] == data[idx - 2]
            {
                triples.push(data[idx]);
            }

            if idx >= 4
                && !quintets.contains(&data[idx])
                && data[idx] == data[idx - 1]
                && data[idx] == data[idx - 2]
                && data[idx] == data[idx - 3]
                && data[idx] == data[idx - 4]
            {
                quintets.push(data[idx]);
            }
        }
        return (triples, quintets);
    }

    /// Find a set number of valid keys and return their indexs
    pub fn generate(&self, num_keys: usize, stretch: bool) -> Vec<usize> {
        let mut key_idxs: HashSet<usize> = HashSet::new();
        let mut possible_keys: HashMap<char, HashSet<usize>> = HashMap::new();
        let mut curr_idx = 0;
        let mut post_num_key_cnt = 0;
        let window = 1000;

        /* Iterate until all the keys are found and then continue on for another
         * thousand hashes to ensure no more keys are found. */
        while post_num_key_cnt < window {
            let mut digest = self.stream(curr_idx);

            if stretch {
                digest = self.stretch(&digest, 2016);
            }

            /* Check the next 1000 hashes after the all the expected keys. */
            if key_idxs.len() >= num_keys {
                post_num_key_cnt += 1;
            }

            /* Determine the multiples. */
            let (triples, quintets) = self.find_multiples(digest);

            /* If the hash contains a quintet it may confirm a key. */
            for conf_k in quintets.iter() {
                if possible_keys.contains_key(&conf_k) {
                    for pk_idx in &possible_keys[conf_k] {
                        if curr_idx - pk_idx <= window {
                            key_idxs.insert(*pk_idx);
                        }
                    }
                }
            }

            /* If the hash contains a triple it could be a key. */
            for pos_k in triples.iter() {
                possible_keys
                    .entry(*pos_k)
                    .and_modify(|x: &mut HashSet<usize>| {
                        x.insert(curr_idx);
                    })
                    .or_insert(HashSet::from([curr_idx]));
            }
            curr_idx += 1;
        }

        /* Return the top `num_keys` number of key indexes. */
        let mut keys = key_idxs.into_iter().collect::<Vec<usize>>();
        keys.sort();
        return (&keys[..num_keys]).to_vec();
    }
}

fn main() {
    let otp = KeyGen::new("qzyelonm");
    println!("Part 1 = {}", otp.generate(64, false)[63]);
    println!("Part 2 = {}", otp.generate(64, true)[63]);
}
