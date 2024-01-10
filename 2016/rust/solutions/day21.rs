use std::collections::HashMap;
use std::usize;

use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    SwapPos(usize, usize),
    SwapLetter(char, char),
    RotLeft(usize),
    RotRight(usize),
    RotLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Instruction {
    fn perform(&self, pwd: &str) -> String {
        match *self {
            Instruction::SwapPos(x, y) => [
                &pwd[0..x],
                &pwd[y..y + 1],
                &pwd[x + 1..y],
                &pwd[x..x + 1],
                &pwd[y + 1..],
            ]
            .concat(),
            Instruction::SwapLetter(a, b) => {
                let x = pwd.find(a).unwrap();
                let y = pwd.find(b).unwrap();
                Instruction::SwapPos(x.min(y), x.max(y)).perform(pwd)
            }
            Instruction::RotLeft(s) => [&pwd[s..], &pwd[0..s]].concat(),
            Instruction::RotRight(s) => [&pwd[pwd.len() - s..], &pwd[0..pwd.len() - s]].concat(),
            Instruction::RotLetter(a) => {
                let x = pwd.find(a).unwrap();
                let s = (1 + x + (x >= 4) as usize) % pwd.len();
                Instruction::RotRight(s).perform(pwd)
            }
            Instruction::Reverse(x, y) => {
                [&pwd[0..x], pwd[x..=y].chars().rev().join("").as_str(), &pwd[y + 1..]].concat()
            }
            Instruction::Move(x, y) => {
                let mut new_pwd = pwd.to_string();
                let c = new_pwd.remove(x);
                new_pwd.insert(y, c);
                new_pwd
            }
        }
    }

    fn invert(&self, pwd: &str, rot_letter_map: &HashMap<usize, Instruction>) -> Instruction {
        match *self {
            Instruction::SwapPos(_, _) => *self,
            Instruction::SwapLetter(_, _) => *self,
            Instruction::RotLeft(s) => Instruction::RotRight(s),
            Instruction::RotRight(s) => Instruction::RotLeft(s),
            Instruction::RotLetter(a) => rot_letter_map[&pwd.find(a).unwrap()],
            Instruction::Reverse(_, _) => *self,
            Instruction::Move(x, y) => Instruction::Move(y, x),
        }
    }
}

pub struct Solution21;
impl Solution21 {
    fn parse(input: ProblemInput) -> Vec<Instruction> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let words = l.split_whitespace().collect_vec();
                match &words[..] {
                    ["swap", "position", ..] => {
                        let x: usize = words[2].parse().unwrap();
                        let y: usize = words[5].parse().unwrap();
                        Instruction::SwapPos(x.min(y), x.max(y))
                    }
                    ["swap", "letter", ..] => {
                        Instruction::SwapLetter(words[2].chars().next().unwrap(), words[5].chars().next().unwrap())
                    }
                    ["rotate", "left", ..] => Instruction::RotLeft(words[2].parse().unwrap()),
                    ["rotate", "right", ..] => Instruction::RotRight(words[2].parse().unwrap()),
                    ["rotate", "based", ..] => Instruction::RotLetter(words[6].chars().next().unwrap()),
                    ["reverse", ..] => Instruction::Reverse(words[2].parse().unwrap(), words[4].parse().unwrap()),
                    ["move", ..] => Instruction::Move(words[2].parse().unwrap(), words[5].parse().unwrap()),
                    _ => panic!(),
                }
            })
            .collect_vec()
    }

    // Rotate by letter is not trivally invertable. Thus we look at each outcome and see that the mapping position of
    // the target char actually is a bijection, at least for the password length of the input.
    // This function creates a mapping from new position to instruction which rotates back to the old position.
    fn create_mapping_rev_letter(pwd_len: usize) -> HashMap<usize, Instruction> {
        let base = "a".repeat(pwd_len);
        let inst = Instruction::RotLetter('b');

        // Find old and new position mapping when rotating on b for all indices of b
        let pos_mapping: HashMap<_, _> = (0..pwd_len)
            .map(|i| {
                let mut pwd = base.clone();
                pwd.replace_range(i..i + 1, "b");
                let new_i = inst.perform(&pwd).find('b').unwrap();
                (new_i, i)
            })
            .collect();

        // Turn old pos into an instruction based on new pos
        pos_mapping
            .into_iter()
            .map(|(new, old)| {
                let inst = if new > old {
                    Instruction::RotLeft(new - old)
                } else {
                    Instruction::RotRight(old - new)
                };
                (new, inst)
            })
            .collect()
    }
}

impl Solution for Solution21 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        let instructions = Self::parse(input);
        let password = if is_sample { "abcde" } else { "abcdefgh" };

        let mut scramble = password.to_string();
        for inst in instructions {
            scramble = inst.perform(&scramble);
        }

        scramble.into_some()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        let instructions = Self::parse(input);
        let password = if is_sample { "decab" } else { "fbgdceah" };
        let rot_letter_map = Self::create_mapping_rev_letter(password.len());

        let mut unscramble = password.to_string();
        for inst in instructions.into_iter().rev() {
            let inv_inst = inst.invert(&unscramble, &rot_letter_map);
            unscramble = inv_inst.perform(&unscramble);
        }

        unscramble.into_some()
    }
}
