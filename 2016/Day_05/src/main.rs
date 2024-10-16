/*
 * --- Day 5: How About a Nice Game of Chess? ---
 *
 * You are faced with a security door designed by Easter Bunny engineers that
 * seem to have acquired most of their security knowledge by watching hacking
 * movies.
 *
 * The eight-character password for the door is generated one character at a
 * time by finding the MD5 hash of some Door ID (your puzzle input) and an
 * increasing integer index (starting with 0).
 *
 * A hash indicates the next character in the password if its hexadecimal
 * representation starts with five zeroes. If it does, the sixth character in
 * the hash is the next character of the password.
 *
 * PART 1:  Given the actual Door ID, what is the password?
 */

use md5;

pub fn md5_idx_hash(seed: &String, index: u32) -> String {
    let digest = md5::compute(format!("{}{}", seed, index).as_bytes());
    return format!("{:x}", digest);
}

pub fn is_char_hash(hash: &String) -> bool {
    return &hash[..5] == "00000";
}

pub fn decipher_password(seed: &String, password_len: usize) -> String {
    let mut index = 0;
    let mut password = String::new();

    /* Increment the index until all the chars are found. */
    while password.len() < password_len {
        let tmp_hash = md5_idx_hash(&seed, index);

        /* Test to see if a new password char has been found. */
        if is_char_hash(&tmp_hash) {
            password.push(tmp_hash.chars().nth(5).unwrap())
        }
        index += 1;
    }
    return password;
}

fn main() {}
