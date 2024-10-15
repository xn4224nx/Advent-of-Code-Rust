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
 */

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

    pub fn verify(self) -> bool {
        false
    }
}

/// Read and parse room infomation from disk
fn read_rooms(file_path: &str) -> Vec<Room> {
    Vec::new()
}

/// Sum the sector ids of all real rooms
fn sum_real_rooms(rooms: &Vec<Room>) -> u32 {
    0
}

fn main() {}
