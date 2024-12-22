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
    pub fn generate(&self, num_keys: usize) -> Vec<usize> {
        let mut key_idxs: HashSet<usize> = HashSet::new();
        let mut possible_keys: HashMap<char, HashSet<usize>> = HashMap::new();
        let mut curr_idx = 0;
        let mut post_num_key_cnt = 0;
        let window = 1000;

        /* Iterate until all the keys are found and then continue on for another
         * thousand hashes to ensure no more keys are found. */
        while post_num_key_cnt < window {
            let digest = self.stream(curr_idx);

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

fn main() {}
