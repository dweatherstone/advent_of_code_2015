pub fn result_day21_stage1(lines: &[String], player_hit_points: Option<i32>) -> i32 {
    let mut fight = parse_day21(lines, player_hit_points.unwrap_or(100));
    let mut min_total_cost = i32::MAX;
    for player_weapon in &get_weapon_options() {
        for player_armour in &get_armour_options() {
            for player_ring_1 in &get_ring_options() {
                for player_ring_2 in &get_ring_options() {
                    // Skip if the same ring is chosen twice (unless both are none)
                    if player_ring_1.is_some() && player_ring_1 == player_ring_2 {
                        continue;
                    }
                    // Calculate stats of this combination
                    let mut total_damage = player_weapon.damage;
                    let mut total_armour = player_weapon.armour;
                    let mut total_cost = player_weapon.cost;
                    for item in [player_armour, player_ring_1, player_ring_2]
                        .into_iter()
                        .flatten()
                    {
                        total_damage += item.damage;
                        total_armour += item.armour;
                        total_cost += item.cost;
                    }
                    fight.player.damage = total_damage;
                    fight.player.armour = total_armour;
                    if fight.does_player_win() && total_cost < min_total_cost {
                        min_total_cost = total_cost;
                    }
                }
            }
        }
    }
    min_total_cost
}

pub fn result_day21_stage2(lines: &[String], player_hit_points: Option<i32>) -> i32 {
    let mut fight = parse_day21(lines, player_hit_points.unwrap_or(100));
    let mut max_total_cost = i32::MIN;
    for player_weapon in &get_weapon_options() {
        for player_armour in &get_armour_options() {
            for player_ring_1 in &get_ring_options() {
                for player_ring_2 in &get_ring_options() {
                    if player_ring_1.is_some() && player_ring_1 == player_ring_2 {
                        continue;
                    }
                    let mut total_damage = player_weapon.damage;
                    let mut total_armour = player_weapon.armour;
                    let mut total_cost = player_weapon.cost;
                    for item in [player_armour, player_ring_1, player_ring_2]
                        .into_iter()
                        .flatten()
                    {
                        total_damage += item.damage;
                        total_armour += item.armour;
                        total_cost += item.cost;
                    }
                    fight.player.damage = total_damage;
                    fight.player.armour = total_armour;
                    if !fight.does_player_win() && total_cost > max_total_cost {
                        max_total_cost = total_cost;
                    }
                }
            }
        }
    }
    max_total_cost
}

fn parse_day21(lines: &[String], player_hit_points: i32) -> Fight {
    let player = Combatant {
        hit_points: player_hit_points,
        damage: 0,
        armour: 0,
    };
    let mut boss_hit_points = 0;
    let mut boss_damage = 0;
    let mut boss_armour = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some((attribute, value)) = line.split_once(": ") {
            match attribute {
                "Hit Points" => boss_hit_points = value.parse().unwrap(),
                "Damage" => boss_damage = value.parse().unwrap(),
                "Armor" => boss_armour = value.parse().unwrap(),
                _ => panic!("Unknown attribute"),
            }
        }
    }
    let boss = Combatant {
        hit_points: boss_hit_points,
        damage: boss_damage,
        armour: boss_armour,
    };
    Fight { player, boss }
}

struct Fight {
    player: Combatant,
    boss: Combatant,
}

impl Fight {
    fn does_player_win(&self) -> bool {
        let mut player_hit_points = self.player.hit_points;
        let mut boss_hit_points = self.boss.hit_points;
        loop {
            // Player attacks
            boss_hit_points -= (self.player.damage - self.boss.armour).max(1);
            if boss_hit_points <= 0 {
                return true;
            }
            // Boss attacks
            player_hit_points -= (self.boss.damage - self.player.armour).max(1);
            if player_hit_points <= 0 {
                return false;
            }
        }
    }
}

struct Combatant {
    hit_points: i32,
    damage: i32,
    armour: i32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Item {
    cost: i32,
    damage: i32,
    armour: i32,
}

fn get_weapon_options() -> Vec<Item> {
    vec![
        Item {
            cost: 8,
            damage: 4,
            armour: 0,
        },
        Item {
            cost: 10,
            damage: 5,
            armour: 0,
        },
        Item {
            cost: 25,
            damage: 6,
            armour: 0,
        },
        Item {
            cost: 40,
            damage: 7,
            armour: 0,
        },
        Item {
            cost: 74,
            damage: 8,
            armour: 0,
        },
    ]
}

fn get_armour_options() -> Vec<Option<Item>> {
    vec![
        None,
        Some(Item {
            cost: 13,
            damage: 0,
            armour: 1,
        }),
        Some(Item {
            cost: 31,
            damage: 0,
            armour: 2,
        }),
        Some(Item {
            cost: 53,
            damage: 0,
            armour: 3,
        }),
        Some(Item {
            cost: 75,
            damage: 0,
            armour: 4,
        }),
        Some(Item {
            cost: 102,
            damage: 0,
            armour: 5,
        }),
    ]
}

fn get_ring_options() -> Vec<Option<Item>> {
    vec![
        None,
        Some(Item {
            cost: 25,
            damage: 1,
            armour: 0,
        }),
        Some(Item {
            cost: 50,
            damage: 2,
            armour: 0,
        }),
        Some(Item {
            cost: 100,
            damage: 3,
            armour: 0,
        }),
        Some(Item {
            cost: 20,
            damage: 0,
            armour: 1,
        }),
        Some(Item {
            cost: 40,
            damage: 0,
            armour: 2,
        }),
        Some(Item {
            cost: 80,
            damage: 0,
            armour: 3,
        }),
    ]
}

#[cfg(test)]
mod day21 {
    use super::*;

    #[test]
    fn fight_example() {
        let fight = Fight {
            player: Combatant {
                hit_points: 8,
                damage: 5,
                armour: 5,
            },
            boss: Combatant {
                hit_points: 12,
                damage: 7,
                armour: 2,
            },
        };
        assert!(fight.does_player_win());
    }
}
