use strum::{EnumIter, IntoEnumIterator};

pub fn result_day22_stage1(lines: &[String]) -> u64 {
    let fight = parse_day22(lines, false);
    let mut min_mana_spend = u64::MAX;
    run_step(fight, &mut min_mana_spend);

    min_mana_spend
}

pub fn result_day22_stage2(lines: &[String]) -> u64 {
    let fight = parse_day22(lines, true);
    let mut min_mana_spend = u64::MAX;
    run_step(fight, &mut min_mana_spend);
    min_mana_spend
}

fn parse_day22(lines: &[String], hard_mode: bool) -> Fight {
    let mut boss_hp = 0;
    let mut boss_damage = 0;
    for line in lines {
        if let Some((attribute, value)) = line.split_once(": ") {
            match attribute {
                "Hit Points" => boss_hp = value.parse().unwrap(),
                "Damage" => boss_damage = value.parse().unwrap(),
                _ => panic!("unknown attribute"),
            }
        }
    }
    Fight {
        player_hp: 50,
        player_mana: 500,
        boss_hp,
        boss_damage,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        mana_spent: 0,
        hard_mode,
    }
}

fn run_step(state: Fight, min_mana: &mut u64) {
    // If we've already spent more than the record, give up on this branch
    if state.mana_spent >= *min_mana {
        return;
    }

    // Try every spell in the book
    for spell in Spell::iter() {
        let mut next_turn = state.clone();

        match next_turn.play_round(&spell) {
            GameResult::PlayerWon => {
                *min_mana = (*min_mana).min(next_turn.mana_spent);
            }
            GameResult::CannotContinue => {}
            GameResult::InProgress => run_step(next_turn, min_mana),
        }
    }
}

#[derive(Clone)]
struct Fight {
    player_hp: i32,
    player_mana: i32,
    boss_hp: i32,
    boss_damage: i32,
    // Timers: if > 0, the effect is active
    shield_timer: i32,
    poison_timer: i32,
    recharge_timer: i32,
    mana_spent: u64,
    hard_mode: bool,
}

impl Fight {
    fn can_cast(&self, spell: &Spell) -> bool {
        match spell {
            Spell::MagicMissile | Spell::Drain => self.player_mana >= spell.cost(),
            Spell::Poison => self.poison_timer == 0 && self.player_mana >= spell.cost(),
            Spell::Shield => self.shield_timer == 0 && self.player_mana >= spell.cost(),
            Spell::Recharge => self.recharge_timer == 0 && self.player_mana >= spell.cost(),
        }
    }

    fn play_round(&mut self, spell: &Spell) -> GameResult {
        // If we are in hard mode, then subtract 1 from player's hp and check if player is still alive
        if self.hard_mode {
            self.player_hp -= 1;
            if self.player_hp <= 0 {
                return GameResult::CannotContinue;
            }
        }

        self.apply_effects();
        if self.boss_hp <= 0 {
            return GameResult::PlayerWon;
        }

        if !self.can_cast(spell) {
            return GameResult::CannotContinue;
        }

        self.cast_spell(spell);
        if self.boss_hp <= 0 {
            return GameResult::PlayerWon;
        }

        // Boss turn
        self.apply_effects();
        if self.boss_hp <= 0 {
            return GameResult::PlayerWon;
        }
        self.take_boss_damage();
        if self.player_hp > 0 {
            GameResult::InProgress
        } else {
            GameResult::CannotContinue
        }
    }

    fn apply_effects(&mut self) {
        if self.poison_timer > 0 {
            self.boss_hp -= 3;
            self.poison_timer -= 1;
        }
        if self.recharge_timer > 0 {
            self.player_mana += 101;
            self.recharge_timer -= 1;
        }
        if self.shield_timer > 0 {
            self.shield_timer -= 1;
        }
    }

    fn cast_spell(&mut self, spell: &Spell) {
        match spell {
            Spell::MagicMissile => self.boss_hp -= 4,
            Spell::Drain => {
                self.boss_hp -= 2;
                self.player_hp += 2;
            }
            Spell::Shield => self.shield_timer = 6,
            Spell::Poison => self.poison_timer = 6,
            Spell::Recharge => self.recharge_timer = 5,
        }
        self.player_mana -= spell.cost();
        self.mana_spent += spell.cost() as u64;
    }

    fn take_boss_damage(&mut self) {
        let current_armour = if self.shield_timer > 0 { 7 } else { 0 };
        let damage_taken = (self.boss_damage - current_armour).max(1);
        self.player_hp -= damage_taken;
    }
}

#[derive(PartialEq, EnumIter)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    /// How much mana does casting the spell cost?
    fn cost(&self) -> i32 {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }
}

enum GameResult {
    InProgress,
    PlayerWon,
    CannotContinue,
}

#[cfg(test)]
mod day22 {
    use super::*;

    #[test]
    fn stage1_example1() {
        let mut state = Fight {
            player_hp: 10,
            player_mana: 250,
            boss_hp: 13,
            boss_damage: 8,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
            mana_spent: 0,
            hard_mode: false,
        };
        // Player turn - casts Poison
        state.apply_effects(); // Nothing happens
        state.cast_spell(&Spell::Poison);
        assert_eq!(state.player_mana, 77);
        assert_eq!(state.poison_timer, 6);
        assert_eq!(state.boss_hp, 13);

        // Boss turn
        state.apply_effects(); // Poison hits! Boss HP 13 -> 10
        assert_eq!(state.boss_hp, 10);
        assert_eq!(state.poison_timer, 5);
        // Boss attacks
        state.take_boss_damage();
        assert_eq!(state.player_hp, 2);

        // Player turn
        state.apply_effects(); // Poison hits
        assert_eq!(state.boss_hp, 7);
        assert_eq!(state.poison_timer, 4);
        state.cast_spell(&Spell::MagicMissile); // Does 4 damage
        assert_eq!(state.player_mana, 24);
        assert_eq!(state.boss_hp, 3);

        // Boss turn
        state.apply_effects(); // Poison hits! Boss dies and player wins
        assert_eq!(state.boss_hp, 0);
        assert_eq!(state.mana_spent, 226);
    }

    #[test]
    fn stage1_example2() {
        let mut state = Fight {
            player_hp: 10,
            player_mana: 250,
            boss_hp: 14,
            boss_damage: 8,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
            mana_spent: 0,
            hard_mode: false,
        };
        // Player turn - casts recharge
        state.apply_effects();
        state.cast_spell(&Spell::Recharge);
        assert_eq!(state.player_mana, 21);
        assert_eq!(state.recharge_timer, 5);
        assert_eq!(state.boss_hp, 14);

        // Boss turn
        state.apply_effects(); // Recharge: Mana increased by 101
        assert_eq!(state.player_mana, 122);
        assert_eq!(state.recharge_timer, 4);
        // Boss attacks - 8 damage
        state.take_boss_damage();
        assert_eq!(state.player_hp, 2);

        // Player turn - gets shield
        state.apply_effects();
        assert_eq!(state.player_mana, 223);
        assert_eq!(state.recharge_timer, 3);
        state.cast_spell(&Spell::Shield);
        assert_eq!(state.player_mana, 110);
        assert_eq!(state.shield_timer, 6);

        // Boss turn
        state.apply_effects();
        assert_eq!(state.player_mana, 211);
        assert_eq!(state.recharge_timer, 2);
        assert_eq!(state.shield_timer, 5);
        // Boss attacks - 1 damage
        state.take_boss_damage();
        assert_eq!(state.player_hp, 1);

        // Player turn - drain
        state.apply_effects();
        assert_eq!(state.player_mana, 312);
        assert_eq!(state.recharge_timer, 1);
        assert_eq!(state.shield_timer, 4);
        state.cast_spell(&Spell::Drain);
        assert_eq!(state.boss_hp, 12);
        assert_eq!(state.player_hp, 3);
        assert_eq!(state.player_mana, 239);

        // Boss turn
        state.apply_effects();
        assert_eq!(state.player_mana, 340);
        assert_eq!(state.recharge_timer, 0);
        assert_eq!(state.shield_timer, 3);
        // Boss attacks - 1 damage
        state.take_boss_damage();
        assert_eq!(state.player_hp, 2);

        // Player turn - poison
        state.apply_effects();
        assert_eq!(state.player_mana, 340);
        assert_eq!(state.recharge_timer, 0);
        assert_eq!(state.shield_timer, 2);
        state.cast_spell(&Spell::Poison);
        assert_eq!(state.player_mana, 167);
        assert_eq!(state.poison_timer, 6);
        assert_eq!(state.boss_hp, 12);

        // Boss turn
        state.apply_effects();
        assert_eq!(state.shield_timer, 1);
        assert_eq!(state.poison_timer, 5);
        assert_eq!(state.boss_hp, 9);
        // Boss attacks - 1 damage
        state.take_boss_damage();
        assert_eq!(state.player_hp, 1);

        // Player turn - Magic missile
        state.apply_effects();
        assert_eq!(state.shield_timer, 0);
        assert_eq!(state.poison_timer, 4);
        assert_eq!(state.boss_hp, 6);
        state.cast_spell(&Spell::MagicMissile);
        assert_eq!(state.player_mana, 114);
        assert_eq!(state.boss_hp, 2);

        // Boss turn
        state.apply_effects();
        assert_eq!(state.poison_timer, 3);
        assert_eq!(state.boss_hp, -1);

        assert_eq!(state.mana_spent, 641);
    }
}
