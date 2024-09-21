#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_22;
use day_22::WizardBattle;

#[test]
fn new_turn_1() {
    let mut test = WizardBattle::new(20, 100, 20, 3);

    test.poison_turns = 3;
    test.shield_turns = 2;
    test.recharge_turns = 1;

    test.impl_active_effects();
    test.impl_active_effects();

    assert_eq!(test.poison_turns, 0);
    assert_eq!(test.shield_turns, 0);
    assert_eq!(test.recharge_turns, 1);
}

#[test]
fn new_turn_2() {
    let mut test = WizardBattle::new(20, 100, 20, 3);

    test.poison_turns = 3;
    test.shield_turns = 2;
    test.recharge_turns = 1;

    test.impl_active_effects();
    test.impl_active_effects();

    assert_eq!(test.poison_turns, 0);
    assert_eq!(test.shield_turns, 0);
    assert_eq!(test.recharge_turns, 1);
}

#[test]
fn new_turn_3() {
    let mut test = WizardBattle::new(20, 100, 20, 3);

    test.poison_turns = 7;
    test.shield_turns = 0;
    test.recharge_turns = 1;

    test.impl_active_effects();
    test.impl_active_effects();

    assert_eq!(test.poison_turns, 5);
    assert_eq!(test.shield_turns, 0);
    assert_eq!(test.recharge_turns, 0);
}

#[test]
fn boss_attack_raw_1() {
    let mut test = WizardBattle::new(20, 100, 20, 3);
    test.boss_attacks();
    test.boss_attacks();
    test.boss_attacks();
    assert_eq!(test.wiz_health, 11);
}

#[test]
fn boss_attack_raw_2() {
    let mut test = WizardBattle::new(21, 100, 20, 4);
    test.boss_attacks();
    test.boss_attacks();
    assert_eq!(test.wiz_health, 13);
}

#[test]
fn boss_attack_raw_3() {
    let mut test = WizardBattle::new(35, 100, 20, 5);
    test.boss_attacks();
    assert_eq!(test.wiz_health, 30);
}

#[test]
fn boss_attack_while_shield_1() {
    let mut test = WizardBattle::new(100, 120, 20, 4);
    test.cast_shield();

    for _ in 0..10 {
        test.boss_attacks();
    }

    assert_eq!(test.wiz_mana, 7);
    assert_eq!(test.wiz_health, 75);
}

#[test]
fn boss_attack_while_shield_2() {
    let mut test = WizardBattle::new(21, 113, 20, 7);
    test.cast_shield();
    test.boss_attacks();
    test.boss_attacks();
    assert_eq!(test.wiz_health, 19);
    assert_eq!(test.wiz_mana, 0);
}

#[test]
fn boss_attack_while_shield_3() {
    let mut test = WizardBattle::new(21, 120, 20, 8);
    test.cast_shield();
    test.boss_attacks();
    test.boss_attacks();
    assert_eq!(test.wiz_health, 19);
    assert_eq!(test.wiz_mana, 7);
}

#[test]
fn boss_attack_while_shield_4() {
    let mut test = WizardBattle::new(21, 120, 20, 10);
    test.cast_shield();
    test.boss_attacks();
    test.boss_attacks();
    assert_eq!(test.wiz_health, 15);
    assert_eq!(test.wiz_mana, 7);
}

#[test]
fn magic_missile_1() {
    let mut test = WizardBattle::new(21, 500, 20, 10);
    test.cast_magic_missile();
    test.cast_magic_missile();
    test.cast_magic_missile();
    assert_eq!(test.bos_health, 8);
    assert_eq!(test.wiz_mana, 341);
}

#[test]
fn magic_missile_2() {
    let mut test = WizardBattle::new(21, 53, 20, 10);
    test.cast_magic_missile();
    assert_eq!(test.bos_health, 16);
    assert_eq!(test.wiz_mana, 0);
}

#[test]
fn magic_missile_3() {
    let mut test = WizardBattle::new(21, 250, 20, 10);
    test.cast_magic_missile();
    test.cast_magic_missile();
    assert_eq!(test.bos_health, 12);
    assert_eq!(test.wiz_mana, 144);
}

#[test]
fn drain_1() {
    let mut test = WizardBattle::new(21, 250, 20, 10);
    test.cast_drain();

    assert_eq!(test.bos_health, 18);
    assert_eq!(test.wiz_health, 23);
    assert_eq!(test.wiz_mana, 177);
}

#[test]
fn drain_2() {
    let mut test = WizardBattle::new(21, 250, 20, 10);
    test.cast_drain();
    test.cast_drain();

    assert_eq!(test.bos_health, 16);
    assert_eq!(test.wiz_health, 25);
    assert_eq!(test.wiz_mana, 104);
}

#[test]
fn drain_3() {
    let mut test = WizardBattle::new(21, 250, 20, 10);
    test.cast_drain();
    test.cast_drain();
    test.cast_drain();

    assert_eq!(test.bos_health, 14);
    assert_eq!(test.wiz_health, 27);
    assert_eq!(test.wiz_mana, 31);
}

#[test]
fn shield() {
    let mut test = WizardBattle::new(100, 1000, 100, 4);

    test.cast_shield();
    assert_eq!(test.wiz_mana, 887);

    test.boss_attacks();
    assert_eq!(test.wiz_health, 99);
    test.cast_magic_missile();

    test.boss_attacks();
    assert_eq!(test.wiz_health, 98);
    test.cast_magic_missile();

    test.boss_attacks();
    assert_eq!(test.wiz_health, 97);

    test.boss_attacks();
    assert_eq!(test.wiz_health, 93);

    test.boss_attacks();
    assert_eq!(test.wiz_health, 89);
}

#[test]
fn poison() {
    let mut test = WizardBattle::new(100, 1000, 100, 4);

    test.cast_poison();
    assert_eq!(test.wiz_mana, 827);

    test.boss_attacks();
    assert_eq!(test.bos_health, 97);
    test.cast_magic_missile();

    test.boss_attacks();
    assert_eq!(test.bos_health, 87);
    test.cast_magic_missile();

    test.boss_attacks();
    assert_eq!(test.bos_health, 77);

    test.boss_attacks();
    assert_eq!(test.bos_health, 74);

    test.boss_attacks();
    assert_eq!(test.bos_health, 74);
}

#[test]
fn recharge() {
    let mut test = WizardBattle::new(100, 1000, 100, 4);
    test.cast_recharge();
    assert_eq!(test.wiz_mana, 771);

    test.boss_attacks();
    assert_eq!(test.wiz_mana, 872);

    test.cast_magic_missile();
    assert_eq!(test.wiz_mana, 920);

    test.boss_attacks();
    assert_eq!(test.wiz_mana, 1021);

    test.cast_magic_missile();
    assert_eq!(test.wiz_mana, 1069);

    test.boss_attacks();
    assert_eq!(test.wiz_mana, 1170);

    test.cast_magic_missile();
    assert_eq!(test.wiz_mana, 1117);

    test.boss_attacks();
    assert_eq!(test.wiz_mana, 1117);
}

#[test]
fn test_battle_1() {
    let mut test = WizardBattle::new(10, 250, 13, 8);
    test.cast_poison();

    assert_eq!(test.bos_health, 13);
    assert_eq!(test.wiz_health, 10);
    assert_eq!(test.wiz_mana, 77);

    test.boss_attacks();

    assert_eq!(test.bos_health, 10);
    assert_eq!(test.wiz_health, 2);
    assert_eq!(test.wiz_mana, 77);

    test.cast_magic_missile();

    assert_eq!(test.bos_health, 3);
    assert_eq!(test.wiz_health, 2);
    assert_eq!(test.wiz_mana, 24);
}

#[test]
fn test_battle_2() {
    let mut test = WizardBattle::new(10, 250, 14, 8);
    test.cast_recharge();

    assert_eq!(test.wiz_health, 10);
    assert_eq!(test.wiz_mana, 21);
    assert_eq!(test.bos_health, 14);
    test.boss_attacks();

    assert_eq!(test.wiz_health, 2);
    assert_eq!(test.wiz_mana, 122);
    assert_eq!(test.bos_health, 14);
    test.cast_shield();

    assert_eq!(test.wiz_health, 2);
    assert_eq!(test.wiz_mana, 110);
    assert_eq!(test.bos_health, 14);
    test.boss_attacks();

    assert_eq!(test.wiz_health, 1);
    assert_eq!(test.wiz_mana, 211);
    assert_eq!(test.bos_health, 14);
    test.cast_drain();

    assert_eq!(test.wiz_health, 3);
    assert_eq!(test.wiz_mana, 239);
    assert_eq!(test.bos_health, 12);
    test.boss_attacks();

    assert_eq!(test.wiz_health, 2);
    assert_eq!(test.wiz_mana, 340);
    assert_eq!(test.bos_health, 12);
    test.cast_poison();

    assert_eq!(test.wiz_health, 2);
    assert_eq!(test.wiz_mana, 167);
    assert_eq!(test.bos_health, 12);
    test.boss_attacks();

    assert_eq!(test.wiz_health, 1);
    assert_eq!(test.wiz_mana, 167);
    assert_eq!(test.bos_health, 9);
    test.cast_magic_missile();

    assert_eq!(test.wiz_health, 1);
    assert_eq!(test.wiz_mana, 114);
    assert_eq!(test.bos_health, 2);
}
