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

pub fn md5_idx_hash(seed: String, index: u32) -> String {
    String::from("")
}

pub fn is_char_hash(hash: String) -> bool {
    false
}

pub fn decipher_password(seed: String, len: usize) -> String {
    String::from("")
}

fn main() {}
