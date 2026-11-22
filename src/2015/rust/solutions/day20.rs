use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let target = input.string().parse::<u64>().unwrap();
    (1..)
        .find(|house| divsum(*house, false) * 10 >= target)
        .unwrap()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let target = input.string().parse::<u64>().unwrap();
    (1..)
        .find(|house| divsum(*house, true) * 11 >= target)
        .unwrap()
        .into()
}

fn divsum(n: u64, lazy: bool) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }

    let root_n = (n as f32).sqrt() as u64;
    let sum: u64 = (1..=root_n)
        .filter(|d| n.is_multiple_of(*d))
        .flat_map(|d| if d * d == n { vec![d] } else { vec![d, n / d] })
        .filter(|div| !lazy || div * 50 >= n)
        .sum();

    sum
}
