use regex::Regex;
use std::collections::HashMap;
use std::hash::Hash;

use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};

#[derive(Debug, Default)]
struct Spell {
    mana: u32,
    dmg: u32,
    heal: u32,
    effect: Option<Effect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum EffectType {
    Dot,
    ManaReg,
    TempArmor,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Effect {
    etype: EffectType,
    value: u32,
    duration: u8,
}

#[derive(Debug, Clone, Eq)]
struct GameState {
    player: (i32, u32, u32),
    boss: (i32, u32),
    active_effects: Vec<(Effect, u8)>,
    mana_spent: u32,
}

// Exclude mana spent from cache
impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.player == other.player && self.boss == other.boss && self.active_effects == other.active_effects
    }
}

// Exclude mana spent from cache
impl Hash for GameState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.player.hash(state);
        self.boss.hash(state);
        self.active_effects.hash(state);
    }
}

impl GameState {
    fn spells() -> [Spell; 5] {
        [
            Spell {
                // Magic Missile
                mana: 53,
                dmg: 4,
                ..Spell::default()
            },
            Spell {
                // Drain
                mana: 73,
                dmg: 2,
                heal: 2,
                ..Spell::default()
            },
            Spell {
                // Shield
                mana: 113,
                effect: Some(Effect {
                    etype: EffectType::TempArmor,
                    value: 7,
                    duration: 6,
                }),
                ..Spell::default()
            },
            Spell {
                // Poison
                mana: 173,
                effect: Some(Effect {
                    etype: EffectType::Dot,
                    value: 3,
                    duration: 6,
                }),
                ..Spell::default()
            },
            Spell {
                // Recharge
                mana: 229,
                effect: Some(Effect {
                    etype: EffectType::ManaReg,
                    value: 101,
                    duration: 5,
                }),
                ..Spell::default()
            },
        ]
    }

    fn is_ongoing(&self) -> bool {
        !self.is_player_dead() && !self.is_boss_dead()
    }

    fn is_player_dead(&self) -> bool {
        self.player.0 <= 0
    }

    fn is_boss_dead(&self) -> bool {
        self.boss.0 <= 0
    }

    fn apply_effects(&mut self) {
        for (effect, timer) in self.active_effects.iter_mut() {
            match effect.etype {
                EffectType::Dot if *timer > 0 => self.boss.0 -= effect.value as i32,
                EffectType::ManaReg if *timer > 0 => self.player.2 += effect.value,
                EffectType::TempArmor => {
                    if *timer == effect.duration {
                        self.player.1 += effect.value;
                    } else if *timer == 1 {
                        self.player.1 -= effect.value;
                    }
                }
                _ => (),
            }

            *timer -= 1;
        }
        self.active_effects.retain(|(_, timer)| *timer > 0);
    }

    fn can_cast_spell(&self, spell: &Spell) -> bool {
        if self.player.2 < spell.mana {
            return false;
        }

        if spell.effect.is_some()
            && self
                .active_effects
                .iter()
                .any(|(effect, _)| effect == spell.effect.as_ref().unwrap())
        {
            return false;
        }

        true
    }

    fn player_cast_spell(&mut self, spell: &Spell) {
        self.player.2 -= spell.mana;
        self.boss.0 -= spell.dmg as i32;
        self.player.0 += spell.heal as i32;
        if let Some(effect) = &spell.effect {
            self.active_effects.push((effect.clone(), effect.duration));
        }
        self.mana_spent += spell.mana;
    }

    fn boss_attack(&mut self) {
        self.player.0 -= (self.boss.1 as i32 - self.player.1 as i32).max(1);
    }
}

pub struct Solution22;
impl Solution22 {
    fn parse(input: ProblemInput) -> (i32, u32) {
        let rgx = Regex::new(r"Hit Points: (?P<hp>\d+)Damage: (?P<dmg>\d+)").unwrap();
        let line = input.string();
        let captures = rgx.captures(&line).unwrap();
        let hp = captures.name("hp").unwrap().as_str().parse().unwrap();
        let dmg = captures.name("dmg").unwrap().as_str().parse().unwrap();
        (hp, dmg)
    }

    fn emulate_game(state: &mut GameState, hard_mode: bool, cache: &mut HashMap<GameState, u32>) -> u32 {
        if let Some(mana) = cache.get(state) {
            return *mana;
        }

        if !state.is_ongoing() {
            return if state.is_boss_dead() {
                state.mana_spent
            } else {
                u32::MAX
            };
        }

        let mut least_mana_spent = u32::MAX;
        for spell in GameState::spells() {
            let mut new_state = state.clone();
            if Self::run_two_turns(&mut new_state, &spell, hard_mode) {
                let spent = Self::emulate_game(&mut new_state, hard_mode, cache);
                cache.insert(state.clone(), spent);
                least_mana_spent = least_mana_spent.min(spent);
            }
        }

        least_mana_spent
    }

    fn run_two_turns(state: &mut GameState, spell: &Spell, hard_mode: bool) -> bool {
        for players_turn in [true, false] {
            // Effects first
            state.apply_effects();
            if state.is_boss_dead() {
                return true;
            }

            // Spell cast / attack
            match players_turn {
                true => {
                    if !state.can_cast_spell(spell) {
                        return false;
                    }

                    if hard_mode {
                        state.player.0 -= 1;
                    }

                    state.player_cast_spell(spell);
                }
                false => {
                    state.boss_attack();
                }
            }

            if !state.is_ongoing() {
                return true;
            }
        }

        true
    }
}

impl Solution for Solution22 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let player = if is_sample { (10, 0, 250) } else { (50, 0, 500) };
        let boss = Self::parse(input);

        let mut game_state = GameState {
            player,
            boss,
            active_effects: vec![],
            mana_spent: 0,
        };

        let mut cache = HashMap::new();
        Self::emulate_game(&mut game_state, false, &mut cache).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let player = if is_sample { (15, 0, 250) } else { (50, 0, 500) };
        let boss = Self::parse(input);

        let mut game_state = GameState {
            player,
            boss,
            active_effects: vec![],
            mana_spent: 0,
        };

        let mut cache = HashMap::new();
        Self::emulate_game(&mut game_state, true, &mut cache).to_result()
    }
}
