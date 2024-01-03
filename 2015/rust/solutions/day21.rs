use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::{iproduct, Itertools};

type Item = (u32, u32, u32);
type Loadout = (Item, Vec<Item>, Vec<Item>);

pub struct Solution21;
impl Solution21 {
    fn parse(input: ProblemInput) -> (u32, u32, u32) {
        let lines = input.lines();
        let hp = lines[0].trim_start_matches("Hit Points: ").parse::<u32>().unwrap();
        let dmg = lines[1].trim_start_matches("Damage: ").parse::<u32>().unwrap();
        let armor = lines[2].trim_start_matches("Armor: ").parse::<u32>().unwrap();
        (hp, dmg, armor)
    }

    fn compute_winning_stats(bhp: u32, bdmg: u32, barmor: u32) -> Vec<(u32, u32)> {
        const MY_HP: u32 = 100;
        (0..bdmg)
            .map(|marmor| {
                let rounds = (MY_HP + 1) / (bdmg - marmor).max(1);
                let mdmg = (bhp + 1) / rounds + barmor;
                (mdmg, marmor)
            })
            .collect_vec()
    }

    fn loadouts() -> impl IntoIterator<Item = Loadout> {
        const WEAPONS: [Item; 5] = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
        const ARMOR: [Item; 5] = [(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];
        const RINGS: [Item; 6] = [(25, 1, 0), (50, 2, 0), (100, 3, 0), (20, 0, 1), (40, 0, 2), (80, 0, 3)];

        let weapon_options = WEAPONS.into_iter();
        let armor_options = ARMOR.into_iter().powerset().filter(|set| set.len() < 2);
        let ring_options = RINGS.into_iter().powerset().filter(|set| set.len() < 3);
        iproduct!(weapon_options, armor_options, ring_options)
    }

    fn eval_loadout(loadout: Loadout) -> Item {
        let (weapon, armors, rings) = loadout;
        let items = [vec![weapon], armors, rings].concat();
        items
            .into_iter()
            .fold((0, 0, 0), |(ac, ad, aa), (c, d, a)| (ac + c, ad + d, aa + a))
    }
}

impl Solution for Solution21 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        if _is_sample {
            return None;
        }

        let (bhp, bdmg, barmor) = Self::parse(input);
        let winning_stats = Self::compute_winning_stats(bhp, bdmg, barmor);

        Self::loadouts()
            .into_iter()
            .map(Self::eval_loadout)
            .filter(|(_, dmg, armor)| winning_stats.iter().any(|(wd, wa)| dmg >= wd && armor >= wa))
            .min()
            .unwrap()
            .0
            .into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        if _is_sample {
            return None;
        }

        let (bhp, bdmg, barmor) = Self::parse(input);
        let winning_stats = Self::compute_winning_stats(bhp, bdmg, barmor);

        Self::loadouts()
            .into_iter()
            .map(Self::eval_loadout)
            .filter(|(_, dmg, armor)| !winning_stats.iter().any(|(wd, wa)| dmg >= wd && armor >= wa))
            .max()
            .unwrap()
            .0
            .into_some()
    }
}
