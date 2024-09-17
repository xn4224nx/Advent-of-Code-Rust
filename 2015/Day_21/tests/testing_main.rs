#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_21;
use day_21::{
    calc_equip_cost, calc_player_stats, does_player_win, read_shop_data, Item, ShopItem, Stats,
};

use std::collections::HashMap;

#[test]
fn read_small_shop_data() {
    assert_eq!(
        read_shop_data("./data/small_shop.txt"),
        HashMap::from([
            (
                Item::Weapon,
                vec![ShopItem {
                    name: String::from("Dagger"),
                    cost: 8,
                    damage: 4,
                    armour: 0,
                    stype: Item::Weapon
                }]
            ),
            (
                Item::Armour,
                vec![
                    ShopItem {
                        name: String::from("Leather"),
                        cost: 13,
                        damage: 0,
                        armour: 1,
                        stype: Item::Armour
                    },
                    ShopItem {
                        name: String::from("Chainmail"),
                        cost: 31,
                        damage: 0,
                        armour: 2,
                        stype: Item::Armour
                    }
                ]
            ),
            (
                Item::Ring,
                vec![
                    ShopItem {
                        name: String::from("Damage +1"),
                        cost: 25,
                        damage: 1,
                        armour: 0,
                        stype: Item::Ring
                    },
                    ShopItem {
                        name: String::from("Defense +1"),
                        cost: 20,
                        damage: 0,
                        armour: 1,
                        stype: Item::Ring
                    }
                ]
            ),
        ])
    )
}

#[test]
fn test_cost_store_purchases_1() {
    assert_eq!(
        calc_equip_cost(&vec![
            ShopItem {
                name: String::from("Dagger"),
                cost: 8,
                damage: 4,
                armour: 0,
                stype: Item::Weapon
            },
            ShopItem {
                name: String::from("Chainmail"),
                cost: 31,
                damage: 0,
                armour: 2,
                stype: Item::Armour
            },
            ShopItem {
                name: String::from("Damage +1"),
                cost: 25,
                damage: 1,
                armour: 0,
                stype: Item::Ring
            }
        ]),
        64
    )
}

#[test]
fn test_cost_store_purchases_2() {
    assert_eq!(
        calc_equip_cost(&vec![ShopItem {
            name: String::from("Dagger"),
            cost: 8,
            damage: 4,
            armour: 0,
            stype: Item::Weapon
        }]),
        8
    )
}

#[test]
fn test_cost_store_purchases_3() {
    assert_eq!(
        calc_equip_cost(&vec![
            ShopItem {
                name: String::from("Dagger"),
                cost: 8,
                damage: 4,
                armour: 0,
                stype: Item::Weapon
            },
            ShopItem {
                name: String::from("Leather"),
                cost: 13,
                damage: 0,
                armour: 1,
                stype: Item::Armour
            },
            ShopItem {
                name: String::from("Damage +1"),
                cost: 25,
                damage: 1,
                armour: 0,
                stype: Item::Ring
            },
            ShopItem {
                name: String::from("Defense +1"),
                cost: 20,
                damage: 0,
                armour: 1,
                stype: Item::Ring
            }
        ]),
        66
    )
}

#[test]
fn test_determine_stats_1() {
    assert_eq!(
        calc_player_stats(&vec![
            ShopItem {
                name: String::from("Dagger"),
                cost: 8,
                damage: 4,
                armour: 0,
                stype: Item::Weapon
            },
            ShopItem {
                name: String::from("Chainmail"),
                cost: 31,
                damage: 0,
                armour: 2,
                stype: Item::Armour
            },
            ShopItem {
                name: String::from("Damage +1"),
                cost: 25,
                damage: 1,
                armour: 0,
                stype: Item::Ring
            }
        ]),
        Stats {
            hit_points: 100,
            damage: 5,
            armour: 2
        }
    )
}

#[test]
fn test_determine_stats_2() {
    assert_eq!(
        calc_player_stats(&vec![ShopItem {
            name: String::from("Dagger"),
            cost: 8,
            damage: 4,
            armour: 0,
            stype: Item::Weapon
        }]),
        Stats {
            hit_points: 100,
            damage: 4,
            armour: 0
        }
    )
}

#[test]
fn test_determine_stats_3() {
    assert_eq!(
        calc_player_stats(&vec![
            ShopItem {
                name: String::from("Dagger"),
                cost: 8,
                damage: 4,
                armour: 0,
                stype: Item::Weapon
            },
            ShopItem {
                name: String::from("Leather"),
                cost: 13,
                damage: 0,
                armour: 1,
                stype: Item::Armour
            },
            ShopItem {
                name: String::from("Damage +1"),
                cost: 25,
                damage: 1,
                armour: 0,
                stype: Item::Ring
            },
            ShopItem {
                name: String::from("Defense +1"),
                cost: 20,
                damage: 0,
                armour: 1,
                stype: Item::Ring
            }
        ]),
        Stats {
            hit_points: 100,
            damage: 5,
            armour: 2
        }
    )
}

#[test]
fn test_boss_defeated() {
    assert_eq!(
        does_player_win(
            &Stats {
                hit_points: 8,
                damage: 5,
                armour: 5
            },
            &Stats {
                hit_points: 12,
                damage: 7,
                armour: 2
            }
        ),
        true
    )
}
