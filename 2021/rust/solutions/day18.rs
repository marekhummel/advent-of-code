use std::collections::VecDeque;
use std::fmt::Debug;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

#[derive(Clone, PartialEq, Eq)]
enum Num {
    Regular(u8),
    Pair(Box<Num>, Box<Num>),
}

impl Debug for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular(arg0) => write!(f, "{}", arg0),
            Self::Pair(arg0, arg1) => write!(f, "[{:?},{:?}]", arg0, arg1),
        }
    }
}

impl Num {
    fn parse(num: &mut VecDeque<char>) -> Num {
        let next = num.pop_front().unwrap();
        if next.is_ascii_digit() {
            return Num::Regular(next as u8 - b'0');
        }

        assert_eq!(next, '[');
        let first = Self::parse(num);
        assert_eq!(num.pop_front().unwrap(), ',');
        let second = Self::parse(num);
        assert_eq!(num.pop_front().unwrap(), ']');
        Num::Pair(Box::new(first), Box::new(second))
    }

    fn add(&self, other: &Num) -> Num {
        let mut result = Num::Pair(Box::new(self.clone()), Box::new(other.clone()));

        // Reduce
        loop {
            if result.explode(0).is_some() {
                continue;
            }
            if result.split() {
                continue;
            }
            break;
        }

        // Return
        result
    }

    fn magnitute(&self) -> u32 {
        match self {
            Num::Regular(n) => *n as u32,
            Num::Pair(left, right) => 3 * left.magnitute() + 2 * right.magnitute(),
        }
    }

    fn explode(&mut self, depth: u8) -> Option<(Option<u8>, Option<u8>)> {
        match self {
            Num::Regular(_) => return None,
            Num::Pair(left_box, right_box) => {
                if depth == 4 {
                    // Find exploding pair and return to parent
                    let Num::Regular(left) = left_box.as_ref().clone() else { unreachable!() };
                    let Num::Regular(right) = right_box.as_ref().clone() else { unreachable!() };
                    *self = Num::Regular(0);
                    return Some((Some(left), Some(right)));
                }

                // If exploding pair found in some child on the left, propagate right num to right child
                if let Some((sv1, sv2)) = left_box.explode(depth + 1) {
                    if let Some(v2) = sv2 {
                        right_box.as_mut().add_explosion(v2, false);
                    }
                    return Some((sv1, None));
                }

                // If exploding pair found in some child on the right, propagate left num to left child
                if let Some((sv1, sv2)) = right_box.explode(depth + 1) {
                    if let Some(v1) = sv1 {
                        left_box.as_mut().add_explosion(v1, true);
                    }
                    return Some((None, sv2));
                }
            }
        }

        None
    }

    fn add_explosion(&mut self, val: u8, right: bool) {
        // Add number to the right / left most child in this tree
        match self {
            Num::Regular(v) => *v += val,
            Num::Pair(_, r) if right => r.add_explosion(val, right),
            Num::Pair(l, _) => l.add_explosion(val, right),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Num::Regular(v) if *v >= 10 => {
                *self = Num::Pair(Box::new(Num::Regular(*v / 2)), Box::new(Num::Regular((*v + 1) / 2)));
                true
            }
            Num::Regular(_) => false,
            Num::Pair(l, r) => {
                if l.split() {
                    return true;
                }
                r.split()
            }
        }
    }
}

pub struct Solution18;
impl Solution18 {}

impl Solution for Solution18 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(4140),
            ProblemResult::U32(3981),
            ProblemResult::U32(3993),
            ProblemResult::U32(4687),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let nums = input
            .lines()
            .into_iter()
            .map(|l| Num::parse(&mut l.chars().collect()))
            .collect_vec();

        let final_num = nums.iter().skip(1).fold(nums[0].clone(), |sum, n| sum.add(n));
        final_num.magnitute().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let largest_magnitude = input
            .lines()
            .into_iter()
            .map(|l| Num::parse(&mut l.chars().collect()))
            .permutations(2)
            .map(|perm| perm[0].add(&perm[1]).magnitute())
            .max()
            .unwrap();

        largest_magnitude.to_result()
    }
}
