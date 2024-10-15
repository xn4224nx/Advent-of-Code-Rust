/*
 * --- Day 4: Security Through Obscurity ---
 *
 * Finally, you come across an information kiosk with a list of rooms. Of
 * course, the list is encrypted and full of decoy data, but the instructions to
 * decode the list are barely hidden nearby. Better remove the decoy data first.
 *
 * Each room consists of an encrypted name (lowercase letters separated by
 * dashes) followed by a dash, a sector ID, and a checksum in square brackets.
 *
 * A room is real (not a decoy) if the checksum is the five most common letters
 * in the encrypted name, in order, with ties broken by alphabetization.
 *
 * PART 1:  What is the sum of the sector IDs of the real rooms?
 *
 * With all the decoy data out of the way, it's time to decrypt this list and
 * get moving.
 *
 * The room names are encrypted by a state-of-the-art shift cipher, which is
 * nearly unbreakable without the right software. However, the information kiosk
 * designers at Easter Bunny HQ were not expecting to deal with a master
 * cryptographer like yourself.
 *
 * To decrypt a room name, rotate each letter forward through the alphabet a
 * number of times equal to the room's sector ID. A becomes B, B becomes C, Z
 * becomes A, and so on. Dashes become spaces.
 *
 * PART 2:  What is the sector ID of the room where North Pole objects are
 *          stored?
 */

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub struct Room {
    pub encr_name: String,
    pub sec_id: u32,
    pub checksum: String,
}

impl Room {
    pub fn new(encr_name: String, sec_id: u32, checksum: String) -> Self {
        return Room {
            encr_name,
            sec_id,
            checksum,
        };
    }

    pub fn verify(&self) -> bool {
        let mut char_cnt: HashMap<char, u32> = HashMap::new();

        /* Count the occurance of characters in the name. */
        for enc_char in self.encr_name.chars() {
            if enc_char == '-' {
                continue;
            };

            char_cnt
                .entry(enc_char)
                .and_modify(|x| *x += 1)
                .or_insert(0);
        }

        /* Re-construct the checksum based on the 5 most populous chars. */
        let mut poss_checksum: Vec<char> = Vec::new();
        while poss_checksum.len() < self.checksum.chars().count() {
            let mut max_cnt = 0;

            /* Find the highest count for a char that is not seen already. */
            for (key, value) in char_cnt.iter() {
                if *value > max_cnt && !poss_checksum.contains(key) {
                    max_cnt = *value;
                };
            }

            /* Collect all the letter that have the highest count. */
            let mut next_chars: Vec<char> = char_cnt
                .iter()
                .filter(|(_, y)| **y == max_cnt)
                .map(|(x, _)| *x)
                .collect();
            next_chars.sort();

            /* Save the new chars. */
            for n_char in next_chars {
                poss_checksum.push(n_char);
            }
        }

        /* Verfiy that the poss_checksum matches the actual one. */
        for (chc_idx, tru_char) in self.checksum.chars().enumerate() {
            if tru_char != poss_checksum[chc_idx] {
                return false;
            }
        }

        return true;
    }

    /// Using shift decryption determine the rooms true name
    pub fn decrypt_name(&self) -> String {
        let mut true_name = String::new();

        for enc_char in self.encr_name.chars() {
            if enc_char == '-' {
                true_name.push(' ');
            } else {
                let enc_char_idx = enc_char as u32 - 'a' as u32;
                let clr_char_idx = (enc_char_idx + self.sec_id) % 26;
                let clr_char = char::from_u32('a' as u32 + clr_char_idx).unwrap();
                true_name.push(clr_char);
            }
        }
        return true_name;
    }
}

/// Read and parse room infomation from disk
pub fn read_rooms(file_path: &str) -> Vec<Room> {
    let room_pat = Regex::new(r"([a-z\-]+)-([0-9]+)\[([a-z]+)\]").unwrap();

    let mut new_rooms = Vec::new();
    let mut buffer = String::new();

    /* Open the file. */
    let file = File::open(file_path).unwrap();
    let mut file_ptr = BufReader::new(file);

    /* Read the file line by line. */
    while file_ptr.read_line(&mut buffer).unwrap() > 0 {
        let Some(raw_data) = room_pat.captures(&buffer) else {
            println!("Line could not be parsed: {}", buffer);
            continue;
        };

        /* Parse and save the elements of the room details. */
        new_rooms.push(Room::new(
            raw_data[1].to_string(),
            raw_data[2].parse::<u32>().unwrap(),
            raw_data[3].to_string(),
        ));
        buffer.clear();
    }
    return new_rooms;
}

/// Sum the sector ids of all real rooms
pub fn sum_real_rooms(rooms: &Vec<Room>) -> u32 {
    return rooms
        .iter()
        .filter(|x| x.verify())
        .map(|y| y.sec_id)
        .sum::<u32>();
}

/// Find the index of the room that contains the north pole object room.
fn find_north_pole_rm(rooms: &Vec<Room>) -> u32 {
    for rm in rooms.iter() {
        let t_name = rm.decrypt_name();

        if t_name.contains("north") && t_name.contains("pole") {
            return rm.sec_id;
        }
    }
    panic!("Room not found!");
}

fn main() {
    let rooms = read_rooms("./data/input.txt");
    println!("Part 1 = {}", sum_real_rooms(&rooms));
    println!("Part 2 = {}", find_north_pole_rm(&rooms));
}
