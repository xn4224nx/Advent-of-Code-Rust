/*
 * --- Day 21: RPG Simulator 20XX ---
 *
 * Little Henry Case got a new video game for Christmas. It's an RPG, and he's
 * stuck on a boss. He needs to know what equipment to buy at the shop. He
 * hands you the controller.
 *
 * In this game, the player (you) and the enemy (the boss) take turns
 * attacking. The player always goes first. Each attack reduces the opponent's
 * hit points by at least 1. The first character at or below 0 hit points
 * loses.
 *
 * Damage dealt by an attacker each turn is equal to the attacker's damage
 * score minus the defender's armor score. An attacker always does at least 1
 * damage. So, if the attacker has a damage score of 8, and the defender has an
 * armor score of 3, the defender loses 5 hit points. If the defender had an
 * armor score of 300, the defender would still lose 1 hit point.
 *
 * Your damage score and armor score both start at zero. They can be increased
 * by buying items in exchange for gold. You start with no items and have as
 * much gold as you need. Your total damage or armor is equal to the sum of
 * those stats from all of your items. You have 100 hit points.
 *
 * Here is what the item shop is selling:
 *
 * Weapons:    Cost  Damage  Armor
 * Dagger        8     4       0
 * Shortsword   10     5       0
 * Warhammer    25     6       0
 * Longsword    40     7       0
 * Greataxe     74     8       0
 *
 * Armor:      Cost  Damage  Armor
 * Leather      13     0       1
 * Chainmail    31     0       2
 * Splintmail   53     0       3
 * Bandedmail   75     0       4
 * Platemail   102     0       5
 *
 * Rings:      Cost  Damage  Armor
 * Damage +1    25     1       0
 * Damage +2    50     2       0
 * Damage +3   100     3       0
 * Defense +1   20     0       1
 * Defense +2   40     0       2
 * Defense +3   80     0       3
 *
 * You must buy exactly one weapon; no dual-wielding. Armor is optional, but
 * you can't use more than one. You can buy 0-2 rings (at most one for each
 * hand). You must use any items you buy. The shop only has one of each item,
 * so you can't buy, for example, two rings of Damage +3.
 *
 * PART 1:  You have 100 hit points. The boss's actual stats are in your puzzle
 *          input. What is the least amount of gold you can spend and still win
 *          the fight?
 */

use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Item {
    Weapon,
    Armour,
    Ring,
}

#[derive(Clone)]
#[derive(PartialEq, Debug)]
pub struct ShopItem {
    pub name: String,
    pub cost: u32,
    pub damage: u32,
    pub armour: u32,
    pub stype: Item,
}

#[derive(PartialEq, Debug)]
pub struct Stats {
    pub hit_points: u32,
    pub damage: u32,
    pub armour: u32,
}

/// Parse the shop data and convert it into a hashmap format.
pub fn read_shop_data(file_path: &str) -> HashMap<Item, Vec<ShopItem>> {
    let mut shop = HashMap::from([
        (Item::Weapon, Vec::new()),
        (Item::Armour, Vec::new()),
        (Item::Ring, Vec::new()),
    ]);
    let mut active_cata: Option<Item> = None;

    /* Data search patterns. */
    let cata_re = Regex::new(r"^([A-Za-z ]+):").unwrap();
    let stats_re = Regex::new(r"^([A-Za-z0-9/+ ]+)\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();

    /* Open the file */
    let file = File::open(file_path).unwrap();
    let mut file_ptr = BufReader::new(file);

    /* Process the file line by line. */
    let mut buffer = String::new();
    while file_ptr.read_line(&mut buffer).unwrap() > 0 {
        /* Detect the start of a new shop catagory. */
        if buffer.contains("Cost") && buffer.contains("Damage") && buffer.contains("Armor") {
            let Some(captured_cata) = cata_re.captures(&buffer) else {
                buffer.clear();
                continue;
            };

            active_cata = match &captured_cata[1] {
                "Weapons" => Some(Item::Weapon),
                "Armor" => Some(Item::Armour),
                "Rings" => Some(Item::Ring),
                _ => panic!("Catagory could not be matched!"),
            };
        } else {
            /* Extract the shop contents. */
            let Some(item_stats) = stats_re.captures(&buffer) else {
                buffer.clear();
                continue;
            };

            shop.entry(active_cata.clone().unwrap())
                .or_insert_with(Vec::new)
                .push(ShopItem {
                    name: String::from(item_stats[1].trim()),
                    cost: item_stats[2].parse::<u32>().unwrap(),
                    damage: item_stats[3].parse::<u32>().unwrap(),
                    armour: item_stats[4].parse::<u32>().unwrap(),
                    stype: active_cata.clone().unwrap(),
                });
        }
        buffer.clear();
    }

    return shop;
}

/// Calculate the cost of a set of equipment
pub fn calc_equip_cost(equip: &Vec<&ShopItem>) -> u32 {
    return equip.iter().map(|x| x.cost).sum::<u32>();
}

/// Determine the player statistics based on their equipment
pub fn calc_player_stats(equip: &Vec<&ShopItem>) -> Stats {
    let mut sum_damage = 0;
    let mut sum_armour = 0;

    for s_item in equip.iter() {
        sum_damage += s_item.damage;
        sum_armour += s_item.armour;
    }

    return Stats {
        hit_points: 100,
        damage: sum_damage,
        armour: sum_armour,
    };
}

/// Work out if the player beats the boss
pub fn does_player_win(player: &Stats, boss: &Stats) -> bool {
    let mut player_health = player.hit_points;
    let mut boss_health = boss.hit_points;

    loop {
        /* The player attacks. */
        boss_health = boss_health.saturating_sub(max(1, player.damage.saturating_sub(boss.armour)));

        if boss_health <= 0 {
            return true;
        }

        /* The boss attacks. */
        player_health = player_health.saturating_sub(max(1, boss.damage.saturating_sub(player.armour)));

        if player_health <= 0 {
            return false;
        }
    }
}

/// Search for the cheapest way to beat the boss
pub fn find_cheapest_win(store: &HashMap<Item, Vec<ShopItem>>, boss: &Stats) -> u32 {
    let mut min_cost = u32::MAX;

    /* Iterate over each weapon. */
    for weapon in store[&Item::Weapon].iter() {

        /* Iterate over each armour type and no armour */
        for num_armour in 0..=1 {
            for armour_comb in store[&Item::Armour].iter().combinations(num_armour) {


                /* Iterate over the rings. */
                for num_rings in 0..=2 {
                    for ring_comb in store[&Item::Ring].iter().combinations(num_rings) {

                        let equip_comb =
                            [vec![weapon], armour_comb.clone(), ring_comb.clone()].concat();
                        let comb_stats = calc_player_stats(&equip_comb);

                        /* Does this equipment combination beat the boss? */
                        if does_player_win(&comb_stats, boss) {
                            let comb_cost = calc_equip_cost(&equip_comb);

                            if comb_cost < min_cost {
                                min_cost = comb_cost;
                            }
                        }
                    }
                }
            }
        }
    }

    return min_cost;
}

fn main() {
    let merchant = read_shop_data("./data/shop.txt");
    let boss_stats = Stats {
        hit_points: 100,
        damage: 8,
        armour: 2,
    };

    println!("Part 1 = {}", find_cheapest_win(&merchant, &boss_stats));
}
