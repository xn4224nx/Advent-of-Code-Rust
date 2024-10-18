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
 *
 * As the door slides open, you are presented with a second door that uses a
 * slightly more inspired security mechanism. Clearly unimpressed by the last
 * version (in what movie is the password decrypted in order?!), the Easter
 * Bunny engineers have worked out a better solution.
 *
 * Instead of simply filling in the password from left to right, the hash now
 * also indicates the position within the password to fill. You still look for
 * hashes that begin with five zeroes; however, now, the sixth character
 * represents the position (0-7), and the seventh character is the character to
 * put in that position.
 *
 * A hash result of 000001f means that f is the second character in the
 * password. Use only the first result for each position, and ignore invalid
 * positions.
 *
 * PART 2:  Given the actual Door ID and this new method, what is the password?
 *          Be extra proud of your solution if it uses a cinematic "decrypting"
 *          animation.
 */

use md5;

pub fn md5_idx_hash(seed: &String, index: u32) -> String {
    let digest = md5::compute(format!("{}{}", seed, index).as_bytes());
    return format!("{:x}", digest);
}

pub fn is_char_hash(hash: &String) -> bool {
    return &hash[..5] == "00000";
}

pub fn decipher_password(seed: &String, password_len: usize, pos_based: bool) -> String {
    let mut index = 0;
    let mut chars_found = 0;
    let mut password = vec!['*'; password_len];

    /* Increment the index until all the chars are found. */
    while chars_found < password_len {
        let tmp_hash = md5_idx_hash(&seed, index);

        /* Test to see if a new password char has been found. */
        if is_char_hash(&tmp_hash) {
            /* Is the password assigned based on position? */
            if pos_based {
                let new_pos = &tmp_hash[5..6];

                /* Test to see if the position is valid */
                let Ok(parsed_pos) = new_pos.parse::<usize>() else {
                    index += 1;
                    continue;
                };

                /* Test to see if the new position is unused. */
                if parsed_pos >= password_len || password[parsed_pos] != '*' {
                    index += 1;
                    continue;
                };

                /* Save the found character. */
                password[parsed_pos] = tmp_hash.chars().nth(6).unwrap();
                chars_found += 1;
            } else {
                password[chars_found] = tmp_hash.chars().nth(5).unwrap();
                chars_found += 1;
            }
        }
        index += 1;
    }
    return password.into_iter().collect();
}

fn main() {
    let input_seed = String::from("ffykfhsq");
    println!("Part 1 = {}", decipher_password(&input_seed, 8, false));
    println!("Part 2 = {}", decipher_password(&input_seed, 8, true));
}
