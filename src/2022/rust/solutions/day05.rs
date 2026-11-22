use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let (mut stacks, moves) = parse(input);

    for (count, from, to) in moves {
        for _ in 0..count {
            let moving_crate = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(moving_crate);
        }
    }

    let tops = stacks.into_iter().map(|s| *s.last().unwrap());
    tops.collect::<String>().into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let (mut stacks, moves) = parse(input);

    for (count, from, to) in moves {
        let mut moving = Vec::new();
        for _ in 0..count {
            moving.push(stacks[from - 1].pop().unwrap());
        }
        moving.reverse();
        stacks[to - 1].extend(moving);
    }

    let tops = stacks.into_iter().map(|s| *s.last().unwrap());
    tops.collect::<String>().into()
}

type Stack = Vec<char>;
type Move = (usize, usize, usize);

fn parse(input: ProblemInput) -> (Vec<Stack>, Vec<Move>) {
    let lines = input.lines();
    let (initial_strs, moves_strs) = lines.split(|l| l.is_empty()).collect_tuple().unwrap();

    let (stack_ids, stack_lines) = initial_strs.split_last().unwrap();

    let num_stacks = stack_ids.chars().filter(|c| !c.is_ascii_whitespace()).count();
    let mut stacks = vec![vec![]; num_stacks];
    for sl in stack_lines.iter().rev() {
        let crates = sl.chars().enumerate().filter(|(_, c)| c.is_ascii_uppercase());
        for (i, c) in crates {
            stacks[(i - 1) / 4].push(c);
        }
    }

    let moves = moves_strs
        .iter()
        .map(|m| {
            let words = m.split_ascii_whitespace().collect_vec();
            (
                words[1].parse().unwrap(),
                words[3].parse().unwrap(),
                words[5].parse().unwrap(),
            )
        })
        .collect();

    (stacks, moves)
}
