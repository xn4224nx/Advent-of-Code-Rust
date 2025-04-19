/*
 * Knot Hash from day 10
 */

#[derive(Debug)]
pub struct KnotHash {
    pub nums: Vec<u32>,
    pub twists: Vec<usize>,
    pub skip_size: usize,
    pub curr_pos: usize,
}

impl KnotHash {
    pub fn new(message: &str) -> Self {
        let mut twists: Vec<usize> = message.as_bytes().iter().map(|x| *x as usize).collect();
        twists.extend(vec![17, 31, 73, 47, 23]);

        return KnotHash {
            nums: (0..=255).collect(),
            twists,
            skip_size: 0,
            curr_pos: 0,
        };
    }

    /// Reverse a section of the numbers
    pub fn reverse(&mut self, rev_len: usize) {
        let arr_len = self.nums.len();

        /* Select the numbers and reverse them. */
        let extract = (0..rev_len)
            .map(|x| self.nums[(self.curr_pos + x) % arr_len])
            .rev()
            .collect::<Vec<u32>>();

        /* Re-insert the numbers back into the vector */
        for (idx, rev_num) in extract.into_iter().enumerate() {
            self.nums[(self.curr_pos + idx) % arr_len] = rev_num;
        }

        /* Calculate the new skip size & position */
        self.curr_pos = (self.curr_pos + rev_len + self.skip_size) % arr_len;
        self.skip_size += 1;
    }

    /// Perform a single round of the hash
    pub fn run_round(&mut self) {
        for idx in 0..self.twists.len() {
            self.reverse(self.twists[idx]);
        }
    }

    /// Calculate a round and return the product of the first two numbers
    pub fn verify_round_hash(&mut self) -> u32 {
        self.run_round();
        return self.nums[0] * self.nums[1];
    }

    /// Determine the final digest of the message
    pub fn digest(&mut self) -> String {
        let num_rounds = 64;
        let block_size = 16;
        let mut sparse_digest: Vec<String> = Vec::new();

        /* Run rounds and presereve the skip and position. */
        for _ in 0..num_rounds {
            self.run_round();
        }

        /* XOR each block of numbers into one number. */
        for blk_idx in 0..block_size {
            let mut tmp_blck = 0;
            for num_idx in 0..block_size {
                tmp_blck ^= self.nums[blk_idx * block_size + num_idx];
            }
            sparse_digest.push(format!("{:02x}", tmp_blck)[..2].to_string())
        }
        return sparse_digest.join("");
    }
}
