use aoc_lib::iterator::ParsedExt;
use itertools::Itertools;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
#[derive(Debug)]
struct Race {
    time: f64,
    record: f64,
}

pub struct Solution06;

impl Solution06 {
    fn parse(&self, input: ProblemInput) -> Vec<Race> {
        let (times, distances) = input
            .lines()
            .iter()
            .map(|l| l.split(':').nth(1).unwrap().split_whitespace().parsed().collect_vec())
            .take(2)
            .collect_tuple()
            .unwrap();

        times
            .into_iter()
            .zip_eq(distances)
            .map(|(t, d)| Race { time: t, record: d })
            .collect_vec()
    }

    fn parse2(&self, input: ProblemInput) -> Race {
        let (time, dist) = input
            .lines()
            .iter()
            .map(|l| {
                l.split(':')
                    .nth(1)
                    .unwrap()
                    .split_whitespace()
                    .collect::<String>()
                    .parse::<f64>()
                    .unwrap()
            })
            .take(2)
            .collect_tuple()
            .unwrap();

        Race { time, record: dist }
    }

    fn solve_race(&self, race: &Race) -> u32 {
        // dist = (race_time - hold) * hold
        // => Inverse parabola, max `r` at race_time `t` / 2
        // => hold_record = 0.5 * (t +- sqrt(t^2 - 4r))
        let sqrt = (race.time * race.time - 4.0 * race.record).sqrt();
        let (hold_low, hold_high) = (0.5 * (race.time - sqrt), 0.5 * (race.time + sqrt));

        // At low and high its equal to the record.
        // Thus we intentionally include the integers below low and above high, and then subtract 2 after.
        // This avoids checking if low and high matches the record exactly.
        (hold_high.ceil() as u32) - (hold_low.floor() as u32) - 1
    }
}

impl Solution for Solution06 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(288),
            ProblemResult::U32(293046),
            ProblemResult::U32(71503),
            ProblemResult::U32(35150181),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let races = self.parse(input);
        races
            .into_iter()
            .map(|r| self.solve_race(&r))
            .product::<u32>()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let race = self.parse2(input);
        self.solve_race(&race).to_result()
    }
}
