use std::cell::RefCell;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::math;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: (String, String),
    test: u64,
    if_true: u8,
    if_false: u8,
    inspected_items: usize,
}

impl Monkey {
    fn from_strs(strs: &[String]) -> Self {
        let items = strs[1]
            .trim_start_matches("  Starting items: ")
            .split(',')
            .parsed()
            .collect();
        let operation = strs[2]
            .trim_start_matches("  Operation: new = old ")
            .split_once(' ')
            .unwrap();
        let test = strs[3].trim_start_matches("  Test: divisible by ").parse().unwrap();
        let if_true = strs[4]
            .trim_start_matches("    If true: throw to monkey ")
            .parse()
            .unwrap();
        let if_false = strs[5]
            .trim_start_matches("    If false: throw to monkey ")
            .parse()
            .unwrap();

        Self {
            items,
            operation: (operation.0.to_string(), operation.1.to_string()),
            test,
            if_true,
            if_false,
            inspected_items: 0,
        }
    }
}

pub struct Solution11;
impl Solution11 {
    fn parse(input: ProblemInput) -> Vec<RefCell<Monkey>> {
        let lines = input.lines();
        let monkey_strs = lines.split(|l| l.is_empty());
        monkey_strs.map(|strs| RefCell::new(Monkey::from_strs(strs))).collect()
    }

    fn round<F: Fn(u64) -> u64>(monkeys: &[RefCell<Monkey>], manage_worry: F) {
        for monkey_cell in monkeys {
            let mut monkey = monkey_cell.borrow_mut();
            for item in &monkey.items {
                let (op, value) = &monkey.operation;
                let mut new_worry = match (op.as_str(), value.as_str()) {
                    ("+", "old") => item + item,
                    ("*", "old") => item * item,
                    ("+", _) => item + value.parse::<u64>().unwrap(),
                    ("*", _) => item * value.parse::<u64>().unwrap(),
                    _ => unreachable!(),
                };
                new_worry = manage_worry(new_worry);

                let next_monkey = if new_worry % monkey.test == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                } as usize;
                monkeys[next_monkey].borrow_mut().items.push(new_worry);
            }
            monkey.inspected_items += monkey.items.len();
            monkey.items.clear();
        }
    }
}

impl Solution for Solution11 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(10605),
            ProblemResult::USize(110888),
            ProblemResult::USize(2713310158),
            ProblemResult::USize(25590400731),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let monkeys = Self::parse(input);

        (0..20).for_each(|_| Self::round(&monkeys, |w| w / 3));

        let inspections = monkeys.into_iter().map(|m| m.borrow().inspected_items);
        inspections.sorted().rev().take(2).product::<usize>().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let monkeys = Self::parse(input);

        // Limit worry to range 0..lcm(tests), this wont affect the test
        let common_multiple = math::lcm(&monkeys.iter().map(|m| m.borrow().test).collect_vec());
        (0..10000).for_each(|_| Self::round(&monkeys, |w| w % common_multiple));

        let inspections = monkeys.into_iter().map(|m| m.borrow().inspected_items);
        inspections.sorted().rev().take(2).product::<usize>().to_result()
    }
}
