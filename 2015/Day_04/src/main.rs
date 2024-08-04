/*
 * --- Day 4: The Ideal Stocking Stuffer ---
 *
 * Santa needs help mining some AdventCoins (very similar
 * to bitcoins) to use as gifts for all the economically
 * forward-thinking little girls and boys.
 *
 * To do this, he needs to find MD5 hashes which, in
 * hexadecimal, start with at least five zeroes. The input
 * to the MD5 hash is some secret key (your puzzle input,
 * given below) followed by a number in decimal.
 *
 * PART 1:  To mine AdventCoins, you must find Santa the
 *          lowest positive number (no leading zeroes: 1,
 *          2, 3, ...) that produces such a hash.
 *
 * PART 2:  Now find one that starts with six zeroes.
 */

use md5;

/// Does this key and answer combination produce a digest
/// with a set number of leading zeroes
pub fn valid_md5_hash(secret_key: &String, answer: usize, num_zeros: usize) -> bool {
    /* Creating the hash digest string. */
    let hash_input = format!("{secret_key}{answer}");
    let hash_digest = format!("{:x}", md5::compute(hash_input.as_bytes()));

    /* Verify the hash digest has the required zeroes. */
    for (idx, h_char) in hash_digest.chars().enumerate() {
        if idx >= num_zeros {
            break;
        };

        if h_char != '0' {
            return false;
        }
    }

    return true;
}

/// Find a number to get a certain amount of zeros via a secret key
fn find_valid_hash(secret_key: &String, num_zeros: usize) -> usize {
    let mut answer = 1;

    while !valid_md5_hash(&secret_key, answer, num_zeros) {
        answer += 1;
    }

    return answer;
}

fn main() {
    let key = String::from("iwrupvqb");

    println!("The answer to part 1 = {}", find_valid_hash(&key, 5));
    println!("The answer to part 2 = {}", find_valid_hash(&key, 6));
}
