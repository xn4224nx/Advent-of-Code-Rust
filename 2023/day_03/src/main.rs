/*
 * --- Day 3: Gear Ratios ---
 *
 * The engineer explains that an engine part seems to be missing from the
 * engine, but nobody can figure out which one. If you can add up all the part
 * numbers in the engine schematic, it should be easy to work out which part is
 * missing.
 *
 * The engine schematic (your puzzle input) consists of a visual representation
 * of the engine. There are lots of numbers and symbols you don't really
 * understand, but apparently any number adjacent to a symbol, even diagonally,
 * is a "part number" and should be included in your sum. (Periods (.) do not
 * count as a symbol.)
 *
 * Part 1: What is the sum of all of the part numbers in the engine schematic?
 
 * Part 2: A gear is any * symbol that is adjacent to exactly two part numbers. 
 * Its gear ratio is the result of multiplying those two numbers together. What 
 * is the sum of all of the gear ratios in your engine schematic?
 */

use regex::Regex;
use std::fs;

/// Load the data into a vector of strings.
fn file_to_str_vec(file_pth: &str) -> Vec<String> {
    return fs::read_to_string(file_pth)
        .expect("Unable to read file")
        .split("\n")
        .map(|line| line.to_string())
        .collect();
}

/// For a line extract the number and its start position.
fn extract_num_pos(raw_line: &String) -> Vec<(u32, u32)> {
    let re_pat = Regex::new(r"[0-9]+").unwrap();
    let mut numbers_positions = Vec::new();

    for reg_res in re_pat.find_iter(&raw_line) {
        let num = reg_res.as_str().parse::<u32>().unwrap();
        let pos = reg_res.start() as u32;
        numbers_positions.push((num, pos));
    }

    return numbers_positions;
}

/// Do two points in a grid overlap?
fn coord_overlap(pnt_0: (u32, u32), pnt_1: (u32, u32)) -> bool {
    let x_diff = pnt_0.0 as i32 - pnt_1.0 as i32;
    let y_diff = pnt_0.1 as i32 - pnt_1.1 as i32;

    return if x_diff.abs() < 2 && y_diff.abs() < 2 {
        true
    } else {
        false
    };
}

/// Get all the coordinates covered by a number
fn number_coverage(num_pos: (u32, u32), row: u32) -> Vec<(u32, u32)> {
    let mut number_coverage = Vec::new();
    let num_digits = num_pos.0.checked_ilog10().unwrap_or(0) + 1;
    let pos = num_pos.1;

    for n in 0..num_digits {
        number_coverage.push((pos + n, row))
    }

    return number_coverage;
}

/// Take the grid of positons and numbers and determine the sum of the numbers
/// in the engine schematic.
fn part_num_sum(raw_data: &Vec<String>, grid: &Vec<Vec<(u32, u32)>>) -> u32 {
    let re_pat = Regex::new(r#"[!#$%&'()*/+/,\-:@/[/]^_{|}~\?\\=]"#).unwrap();
    let mut sum = 0;

    for (row_idx, line) in raw_data.iter().enumerate() {
        /* Find the symbol positions */
        let sym_pos: Vec<u32> = re_pat.find_iter(&line).map(|x| x.start() as u32).collect();

        for col_idx in &sym_pos {
            /* Check for the numbers in-line. */
            for num in &grid[row_idx] {
                for point in number_coverage(*num, row_idx as u32) {
                    if coord_overlap(point, (*col_idx, row_idx as u32)) {
                        sum += num.0;
                        break;
                    }
                }
            }

            /* Check for numbers above. */
            if row_idx != 0 {
                for num in &grid[row_idx - 1] {
                    for point in number_coverage(*num, row_idx as u32) {
                        if coord_overlap(point, (*col_idx, row_idx as u32)) {
                            sum += num.0;
                            break;
                        }
                    }
                }
            }

            /* Check for numbers below. */
            if row_idx != grid.len() - 1 {
                for num in &grid[row_idx + 1] {
                    for point in number_coverage(*num, row_idx as u32) {
                        if coord_overlap(point, (*col_idx, row_idx as u32)) {
                            sum += num.0;
                            break;
                        }
                    }
                }
            }
        }
    }

    return sum;
}

/// Find the gears and if there are only two numbers next to it, multiply them.
/// Sum all those values and return the total.
fn part_gear_sum(raw_data: &Vec<String>, grid: &Vec<Vec<(u32, u32)>>) -> u32 {
    let re_pat = Regex::new(r#"[*]"#).unwrap();
    let mut sum = 0;
    
    
    for (row_idx, line) in raw_data.iter().enumerate() {
        /* Find every gear. */
        let gear_pos: Vec<u32> = re_pat.find_iter(&line).map(|x| x.start() as u32).collect();
        
        /* Iterate over every gear in the row. */
        for col_idx in &gear_pos {
            
            let mut found_nums:Vec<u32> =  Vec::new();
            
            /* Check for the numbers in-line. */
            for num in &grid[row_idx] {
                for point in number_coverage(*num, row_idx as u32) {
                    if coord_overlap(point, (*col_idx, row_idx as u32)) {
                        found_nums.push(num.0);
                        break;
                    }
                }
            }

            /* Check for numbers above. */
            if row_idx != 0 {
                for num in &grid[row_idx - 1] {
                    for point in number_coverage(*num, row_idx as u32) {
                        if coord_overlap(point, (*col_idx, row_idx as u32)) {
                            found_nums.push(num.0);
                            break;
                        }
                    }
                }
            }

            /* Check for numbers below. */
            if row_idx != grid.len() - 1 {
                for num in &grid[row_idx + 1] {
                    for point in number_coverage(*num, row_idx as u32) {
                        if coord_overlap(point, (*col_idx, row_idx as u32)) {
                            found_nums.push(num.0);
                            break;
                        }
                    }
                }
            }
            
            /* If the gear has exactly two numbers multiply and sum them. */
            if found_nums.len() == 2 {
                sum += found_nums[0] * found_nums[1];
            }
                    
        }
    }
    
    return sum;
}

fn main() {
    let data = file_to_str_vec("./data/input.txt");
    let data_postions: Vec<_> = data.iter().map(|x| extract_num_pos(&x)).collect();
    
    let p1_ans = part_num_sum(&data, &data_postions);
    println!("The answer to part one = {}", p1_ans);
    
    let p2_ans = part_gear_sum(&data, &data_postions);
    println!("The answer to part two = {}", p2_ans);
    
}
