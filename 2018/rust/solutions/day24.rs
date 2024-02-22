use std::collections::{HashMap, HashSet};

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use regex::Regex;

type DivId = (Army, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Army {
    ImmuneSystem,
    Infection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Division {
    army: Army,
    num: u8,
    soldiers: u32,
    hp: u32,
    dmg: u32,
    dmg_type: String,
    weaknesses: HashSet<String>,
    immunities: HashSet<String>,
    initiative: u32,
}

impl Division {
    fn from_line(line: &str, army_name: &str, num: u8) -> Self {
        let rgx = Regex::new(r"^(?P<soldiers>\d+) units each with (?P<hp>\d+) hit points (?:\((?P<specials>[\w,; ]+)\) )?with an attack that does (?P<dmg>\d+) (?P<dmg_type>\w+) damage at initiative (?P<initiative>\d+)$").unwrap();
        let captures = rgx.captures(line).unwrap();
        let soldiers = captures.name("soldiers").unwrap().as_str().parse().unwrap();
        let hp = captures.name("hp").unwrap().as_str().parse().unwrap();
        let specials = captures.name("specials").map(|m| m.as_str()).unwrap_or("");
        let dmg = captures.name("dmg").unwrap().as_str().parse().unwrap();
        let dmg_type = captures.name("dmg_type").unwrap().as_str().to_string();
        let initiative = captures.name("initiative").unwrap().as_str().parse().unwrap();

        let mut weaknesses = HashSet::new();
        let mut immunities = HashSet::new();
        for special in specials.split("; ") {
            if let Some(weaks) = special.strip_prefix("weak to ") {
                weaknesses.extend(weaks.split(',').map(|w| w.trim().to_string()));
            }
            if let Some(immuns) = special.strip_prefix("immune to ") {
                immunities.extend(immuns.split(',').map(|w| w.trim().to_string()));
            }
        }

        let army = match army_name {
            "Immune System" => Army::ImmuneSystem,
            "Infection" => Army::Infection,
            _ => unreachable!(),
        };

        Division {
            army,
            num,
            soldiers,
            hp,
            dmg,
            dmg_type,
            weaknesses,
            immunities,
            initiative,
        }
    }

    fn id(&self) -> DivId {
        (self.army, self.num)
    }

    fn effective_power(&self) -> u32 {
        self.soldiers * self.dmg
    }

    fn select_target(&self, divisions: &[Division], selected: &[DivId]) -> Option<DivId> {
        divisions
            .iter()
            .filter(|d| d.army != self.army)
            .filter(|enemy| enemy.soldiers > 0)
            .filter(|enemy| !selected.contains(&enemy.id()))
            .max_by_key(|enemy| (self.expected_dmg(enemy), enemy.effective_power(), enemy.initiative))
            .filter(|enemy| self.expected_dmg(enemy) > 0)
            .map(|t| t.id())
    }

    fn expected_dmg(&self, other: &Division) -> u32 {
        let ep = self.effective_power();
        if other.immunities.contains(&self.dmg_type) {
            0
        } else if other.weaknesses.contains(&self.dmg_type) {
            ep * 2
        } else {
            ep
        }
    }

    fn attack(&self, other: &mut Division) -> bool {
        let killed = self.expected_dmg(other) / other.hp;
        other.soldiers = other.soldiers.saturating_sub(killed);
        killed > 0
    }
}

pub struct Solution24;
impl Solution24 {
    fn parse(input: ProblemInput) -> Vec<Division> {
        let mut divisions = Vec::new();

        for team in input.lines().split(|l| l.is_empty()) {
            let army = team[0].trim_end_matches(':');
            divisions.extend(
                team[1..]
                    .iter()
                    .enumerate()
                    .map(|(i, l)| Division::from_line(l, army, i as u8 + 1)),
            );
        }

        divisions
    }

    fn fight(mut divisions: Vec<Division>) -> Option<(Army, u32)> {
        for _ in 1.. {
            // Target Selection
            let mut targets = HashMap::new();
            for div in divisions
                .iter()
                .filter(|d| d.soldiers > 0)
                .sorted_by_key(|d| (d.effective_power(), d.initiative))
                .rev()
            {
                let selected = targets.values().copied().collect_vec();
                if let Some(target) = div.select_target(&divisions, &selected) {
                    targets.insert(div.id(), target);
                }
            }

            // Attack
            let mut success = false;
            let attackers = divisions
                .iter()
                .filter(|d| d.soldiers > 0)
                .sorted_by_key(|d| d.initiative)
                .rev()
                .map(|d| d.id())
                .collect_vec();
            for att in attackers {
                let attacker = divisions.iter().find(|a| a.id() == att).unwrap().clone();
                if let Some(t_id) = targets.get(&att) {
                    let target = divisions.iter_mut().find(|t| t.id() == *t_id).unwrap();
                    success |= attacker.attack(target);
                }
            }

            // Fight finished without any attack => remis
            if !success {
                return None;
            }

            if !Self::has_soldiers(&divisions, Army::ImmuneSystem) {
                return Some((Army::Infection, divisions.into_iter().map(|d| d.soldiers).sum()));
            }

            if !Self::has_soldiers(&divisions, Army::Infection) {
                return Some((Army::ImmuneSystem, divisions.into_iter().map(|d| d.soldiers).sum()));
            }
        }

        unreachable!()
    }

    fn has_soldiers(divisions: &[Division], army: Army) -> bool {
        divisions.iter().filter(|d| d.army == army && d.soldiers > 0).count() > 0
    }

    fn fight_boosted(mut divisions: Vec<Division>, boost: u32) -> Option<(Army, u32)> {
        for div in divisions.iter_mut() {
            if div.army == Army::ImmuneSystem {
                div.dmg += boost;
            }
        }

        Self::fight(divisions)
    }
}

impl Solution for Solution24 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(5216),
            ProblemResult::U32(26937),
            ProblemResult::U32(51),
            ProblemResult::U32(4893),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let divisions = Self::parse(input);
        let (_, remaining) = Self::fight(divisions).unwrap();
        remaining.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let divisions = Self::parse(input);

        let mut low = 1;
        let mut high = u32::MAX / 2;

        while high - low > 1 {
            let attack_boost = (low + high) / 2;

            match Self::fight_boosted(divisions.clone(), attack_boost) {
                Some((Army::ImmuneSystem, _)) => high = attack_boost,
                Some((Army::Infection, _)) => low = attack_boost,
                None => low = attack_boost,
            }
        }

        Self::fight_boosted(divisions, high).unwrap().1.to_result()
    }
}
