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

use itertools::Itertools;
use std::cmp::max;

pub struct WizardBattle {
    pub wiz_health: u32,
    pub wiz_mana: u32,
    pub bos_health: u32,
    pub bos_damage: u32,
    pub shield_turns: u32,
    pub poison_turns: u32,
    pub recharge_turns: u32,
    pub spent_mana: u32,
}

#[derive(Debug)]
enum Spell {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, PartialEq)]
pub enum BattleStatus {
    BossWin,
    WizardWin,
    Undecided,
}

impl WizardBattle {
    /// Create a new instance of the wizard battle.
    pub fn new(wiz_he: u32, wiz_ma: u32, bos_he: u32, bos_da: u32) -> WizardBattle {
        return WizardBattle {
            wiz_health: wiz_he,
            wiz_mana: wiz_ma,
            bos_health: bos_he,
            bos_damage: bos_da,
            shield_turns: 0,
            poison_turns: 0,
            recharge_turns: 0,
            spent_mana: 0,
        };
    }

    /// Cast a generic spell of a certain cost
    fn spell_econ(&mut self, spell_cost: u32) -> bool {
        let (new_mana, mana_run_out) = self.wiz_mana.overflowing_sub(spell_cost);
        self.wiz_mana = new_mana;
        self.spent_mana += spell_cost;
        return mana_run_out;
    }

    /// Determine the result of a battle based on certain factors
    fn det_battle_outcome(&self, mana_run_out: bool) -> BattleStatus {
        return if mana_run_out || self.wiz_health == 0 {
            BattleStatus::BossWin
        } else if self.bos_health == 0 {
            BattleStatus::WizardWin
        } else {
            BattleStatus::Undecided
        };
    }

    /// Simulate the boss attacking the wizard
    pub fn boss_attacks(&mut self) -> BattleStatus {
        let damage = if self.shield_turns > 0 {
            self.bos_damage.saturating_sub(7)
        } else {
            self.bos_damage
        };

        self.wiz_health = self.wiz_health.saturating_sub(max(damage, 1));
        return self.det_battle_outcome(false);
    }

    /// The wizard casts magic missile against the boss.
    pub fn cast_magic_missile(&mut self) -> BattleStatus {
        let mana_run_out = self.spell_econ(53);
        self.bos_health = self.bos_health.saturating_sub(4);
        return self.det_battle_outcome(mana_run_out);
    }

    /// The wizard casts the drain spell
    pub fn cast_drain(&mut self) -> BattleStatus {
        let mana_run_out = self.spell_econ(73);
        self.bos_health = self.bos_health.saturating_sub(2);
        self.wiz_health += 2;
        return self.det_battle_outcome(mana_run_out);
    }

    /// The wizard casts the shield spell
    pub fn cast_shield(&mut self) -> BattleStatus {
        let mana_run_out = self.spell_econ(113);
        self.shield_turns = 6;
        return self.det_battle_outcome(mana_run_out);
    }

    /// The wizard casts the poison spell
    pub fn cast_poison(&mut self) -> BattleStatus {
        let mana_run_out = self.spell_econ(173);
        self.poison_turns = 6;
        return self.det_battle_outcome(mana_run_out);
    }

    /// The wizard casts the recharge spell
    pub fn cast_recharge(&mut self) -> BattleStatus {
        let mana_run_out = self.spell_econ(229);
        self.recharge_turns = 5;
        return self.det_battle_outcome(mana_run_out);
    }

    /// Take care of the housekeeping tasks at the start of a turn
    pub fn impl_active_effects(&mut self) -> BattleStatus {
        if self.shield_turns > 0 {
            self.shield_turns -= 1;
        };

        if self.poison_turns > 0 {
            self.poison_turns -= 1;
            self.bos_health = self.bos_health.saturating_sub(3);
        };

        if self.recharge_turns > 0 {
            self.recharge_turns -= 1;
            self.wiz_mana += 101;
        };
        return self.det_battle_outcome(false);
    }
}

/// Work out the maximum mana that would be spent to cast all the spells
fn spells_cost(all_spells: &Vec<&Spell>) -> u32 {
    return all_spells
        .iter()
        .map(|x| match x {
            Spell::Missile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        })
        .sum::<u32>();
}

/// Simulate multiple wizard battles and find the way to win with the least mana.
fn find_lowest_mana_to_win() -> u32 {
    let mut lowest_mana = u32::MAX;

    /* Assume that the solution is found by casting 100 or less spells. */
    for num_spells in 1..=100 {
        let mut no_sols_for_this_num = true;

        /* Pick a number of spells to cast. */
        for spell_comb in [
            Spell::Missile,
            Spell::Drain,
            Spell::Shield,
            Spell::Poison,
            Spell::Recharge,
        ]
        .iter()
        .combinations_with_replacement(num_spells)
        {
            /* Check if this combination could actually yield a solution. */
            if spells_cost(&spell_comb) > lowest_mana {
                continue;
            };

            /* Iterate over the permutations of this combination of spells. */
            for spell_perm in spell_comb.iter().permutations(num_spells) {
                let mut simul = WizardBattle::new(50, 500, 51, 9);
                let mut battle_result = BattleStatus::Undecided;

                /* Track this battle and see how it ends. */
                for spell in spell_perm.iter() {
                    battle_result = simul.impl_active_effects();
                    if battle_result != BattleStatus::Undecided {
                        break;
                    };

                    battle_result = match spell {
                        Spell::Missile => simul.cast_magic_missile(),
                        Spell::Drain => simul.cast_drain(),
                        Spell::Shield => simul.cast_shield(),
                        Spell::Poison => simul.cast_poison(),
                        Spell::Recharge => simul.cast_recharge(),
                    };
                    if battle_result != BattleStatus::Undecided {
                        break;
                    };

                    battle_result = simul.impl_active_effects();
                    if battle_result != BattleStatus::Undecided {
                        break;
                    };

                    battle_result = simul.boss_attacks();
                    if battle_result != BattleStatus::Undecided {
                        break;
                    };
                }

                /* Detect a new record for lowest mana spent to win. */
                if battle_result == BattleStatus::WizardWin && simul.spent_mana < lowest_mana {
                    lowest_mana = simul.spent_mana;
                    no_sols_for_this_num = false;
                }
            }
        }

        if lowest_mana != u32::MAX && no_sols_for_this_num {
            break;
        }
    }

    return lowest_mana;
}

fn main() {
    println!("Part 1 = {}", find_lowest_mana_to_win());
}
