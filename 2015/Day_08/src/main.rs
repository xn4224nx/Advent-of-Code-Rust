/*
 * --- Day 8: Matchsticks ---
 *
 * Space on the sleigh is limited this year, and so Santa will be bringing his
 * list as a digital copy. He needs to know how much space it will take up when
 * stored.
 *
 * It is common in many programming languages to provide a way to escape special
 * characters in strings. For example, C, JavaScript, Perl, Python, and even PHP
 * handle special characters in very similar ways.
 *
 * However, it is important to realize the difference between the number of
 * characters in the code representation of the string literal and the number of
 * characters in the in-memory string itself.
 *
 * Santa's list is a file that contains many double-quoted string literals, one
 * on each line. The only escape sequences used are \\ (which represents a
 * single backslash), \" (which represents a lone double-quote character), and
 * \x plus two hexadecimal characters (which represents a single character with
 * that ASCII code).
 *
 * PART 1:  Disregarding the whitespace in the file, what is the number of
 *          characters of code for string literals minus the number of
 *          characters in memory for the values of the strings in total for the
 *          entire file?
 *
 * Now, let's go the other way. In addition to finding the number of characters
 * of code, you should now encode each code representation as a new string and
 * find the number of characters of the new encoded representation, including the
 * surrounding double quotes.
 *
 * PART 2:  Your task is to find the total number of characters to represent the
 *          newly encoded strings minus the number of characters of code in each
 *          original string literal.
 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Open the data file and iterate over it line by line
pub fn parse_data(file_path: &str) -> (usize, usize) {
    let file = File::open(file_path).unwrap();
    let mut fp = BufReader::new(file);

    let mut str_cnt = 0;
    let mut raw_str_cnt = 0;
    let mut enc_str_cnt = 0;

    let mut buffer = String::new();

    while fp.read_line(&mut buffer).unwrap() > 0 {
        /* Remove the end line char */
        buffer.truncate(buffer.len() - 1);

        /* Count the length of various string forms. */
        str_cnt += str_len(&buffer);
        raw_str_cnt += raw_str_len(&buffer);
        enc_str_cnt += encoded_str_len(&buffer);

        buffer.clear()
    }

    println!("{} {} {}", str_cnt, raw_str_cnt, enc_str_cnt);

    return (raw_str_cnt - str_cnt, enc_str_cnt - raw_str_cnt);
}

/// Determine the length of the String
pub fn str_len(value: &String) -> usize {
    let hex_re = Regex::new(r#"\\x[0-9,a-f][0-9,a-f]"#).unwrap();
    let test = value.replace(r#"\""#, "B");
    let test = test.replace(r#"\\"#, "C");
    let test = hex_re.replace_all(&test, "A");

    return test.len() - 2;
}

/// Determine the raw String length
pub fn raw_str_len(value: &String) -> usize {
    return value.len();
}

pub fn encoded_str_len(value: &String) -> usize {
    let raw_len = raw_str_len(value) + 2;
    let new_quotes = value.matches(r#"""#).count();
    let new_back = value.matches(r#"\"#).count();

    return raw_len + new_quotes + new_back;
}

fn main() {
    let res = parse_data("./data/input.txt");
    println!("Part 1 = {}\nPart 2 = {}", res.0, res.1);
}
