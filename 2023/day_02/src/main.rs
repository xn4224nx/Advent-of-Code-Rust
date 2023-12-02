/*
 * --- Day 2: Cube Conundrum ---
 *
 * As you walk, the Elf shows you a small bag and some cubes which are either
 * red, green, or blue. Each time you play this game, he will hide a secret
 * number of cubes of each color in the bag, and your goal is to figure out
 * information about the number of cubes.
 *
 * To get information, once a bag has been loaded with cubes, the Elf will reach
 * into the bag, grab a handful of random cubes, show them to you, and then put
 * them back in the bag. He'll do this a few times per game.
 *
 * The Elf would first like to know which games would have been possible if the
 * bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
 *
 * Determine which games would have been possible if the bag had been loaded
 * with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of
 * the IDs of those games?
 
 As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of cubes of each color that could have been in the bag to make the game possible?
 */

use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let comp_game = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let input_file = "../data/input.txt";

    let mut possible_games = vec![];
    let mut game_powers = vec![];
    
    /* Key Regexes */
    let game_re = Regex::new(r"\d+").unwrap();
    let draw_re = Regex::new(r"(\d+) ([a-z]+)").unwrap();

    /* Read the whole text file in as a raw string. */
    let contents = fs::read_to_string(input_file).unwrap();
    let raw_lines: Vec<_> = contents.split('\n').collect();

    /* Iterate over the vector of the line strings. */
    'line_loop: for line in raw_lines {
        
        /* Extract the game number. */
        let Some(game_num) = game_re.find(line) else {
            continue 'line_loop;
        };
        let game_num = game_num.as_str().parse::<u32>().unwrap();

        /* Extract each draw from the bag into a hashmap. */
        let all_raw_draws = line.split(':').last().unwrap().trim();

        let mut game_cubes = HashMap::<&str, u32>::new();

        for draw in all_raw_draws.split(';') {
            for cube in draw_re.captures_iter(draw) {
                
                /* Parse the cube infomation */
                let cube_cnt = cube.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let cube_col = cube.get(2).unwrap().as_str();

                /* When the cube colour is not represented, add it. */
                if !game_cubes.contains_key(cube_col) {
                    game_cubes.insert(cube_col, cube_cnt);

                /* If the value exists but this count it bigger, replace it. */
                } else if game_cubes[cube_col] < cube_cnt {
                    game_cubes.get_mut(cube_col).map(|val| {
                        *val = cube_cnt;
                    });
                }
            }
        }
        
        /* Calculate the power of the cubes */
        game_powers.push(game_cubes.values().product());
        
        /* Compare each hashmap to comparison game. */
        for (comp_cube_col, comp_cube_cnt) in comp_game.iter() {
            if game_cubes.get(comp_cube_col).unwrap_or(&0) > comp_cube_cnt {
                continue 'line_loop;
            }
        }

        /* If all the draws pass save the game number. */
        possible_games.push(game_num);
    }

    println!("The sum of games = {}", possible_games.iter().sum::<u32>());
    println!("The sum of game powers = {}", game_powers.iter().sum::<u32>());
}
