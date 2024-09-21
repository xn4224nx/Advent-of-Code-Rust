/*
 * --- Day 22: Wizard Simulator 20XX ---
 *
 * Little Henry Case decides that defeating bosses with swords and stuff is
 * boring. Now he's playing the game with a wizard. Of course, he gets stuck on
 * another boss and needs your help again.
 *
 * In this version, combat still proceeds with the player and the boss taking
 * alternating turns. The player still goes first. Now, however, you don't get
 * any equipment; instead, you must choose one of your spells to cast. The first
 * character at or below 0 hit points loses.
 *
 * Since you're a wizard, you don't get to wear armor, and you can't attack
 * normally. However, since you do magic damage, your opponent's armor is
 * ignored, and so the boss effectively has zero armor as well. As before, if
 * armor (from a spell, in this case) would reduce damage below 1, it becomes 1
 * instead - that is, the boss' attacks always deal at least 1 damage.
 *
 * On each of your turns, you must select one of your spells to cast. If you
 * cannot afford to cast any spell, you lose. Spells cost mana; you start with
 * 500 mana, but have no maximum limit. You must have enough mana to cast a
 * spell, and its cost is immediately deducted when you cast it. Your spells are
 * Magic Missile, Drain, Shield, Poison, and Recharge.
 *
 *      -   Magic Missile costs 53 mana. It instantly does 4 damage.
 *
 *      -   Drain costs 73 mana. It instantly does 2 damage and heals you for 2
 *          hit points.
 *
 *      -   Shield costs 113 mana. It starts an effect that lasts for 6 turns.
 *          While it is active, your armor is increased by 7.
 *
 *      -   Poison costs 173 mana. It starts an effect that lasts for 6 turns.
 *          At the start of each turn while it is active, it deals the boss 3
 *          damage.
 *
 *      -   Poison costs 173 mana. It starts an effect that lasts for 6 turns.
 *          At the start of each turn while it is active, it deals the boss 3
 *          damage.
 *
 * Effects all work the same way. Effects apply at the start of both the
 * player's turns and the boss' turns. Effects are created with a timer (the
 * number of turns they last); at the start of each turn, after they apply any
 * effect they have, their timer is decreased by one. If this decreases the
 * timer to zero, the effect ends. You cannot cast a spell that would start an
 * effect which is already active. However, effects can be started on the same
 * turn they end.
 *
 * You start with 50 hit points and 500 mana points. The boss's actual stats are
 * in your puzzle input.
 *
 * PART 1:  What is the least amount of mana you can spend and still win the
 *          fight?
 */

pub struct WizardBattle {
    pub wiz_health: u32,
    pub wiz_mana: u32,
    pub bos_health: u32,
    pub bos_damage: u32,
    pub shield_turns: u32,
    pub poison_turns: u32,
    pub recharge_turns: u32,
}

impl WizardBattle {
    pub fn new(wiz_he: u32, wiz_ma: u32, bos_he: u32, bos_da: u32) -> WizardBattle {
        return WizardBattle {
            wiz_health: wiz_he,
            wiz_mana: wiz_ma,
            bos_health: bos_he,
            bos_damage: bos_da,
            shield_turns: 0,
            poison_turns: 0,
            recharge_turns: 0,
        };
    }

    pub fn boss_attacks(&mut self) {}

    pub fn cast_magic_missile(&mut self) {}

    pub fn cast_drain(&mut self) {}

    pub fn cast_shield(&mut self) {}

    pub fn cast_poison(&mut self) {}

    pub fn cast_recharge(&mut self) {}

    pub fn impl_active_effects(&mut self) {}

    pub fn find_lowest_mana_to_win(&mut self) -> u32 {
        0
    }
}

fn main() {}
