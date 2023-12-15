use itertools::Itertools;

use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult};
#[derive(Debug)]
struct Race {
    time: f64,
    record: f64,
}

pub struct Solution06;

impl Solution06 {
    fn parse(&self, input: ProblemInput) -> Vec<Race> {
        let (times, distances) = input
            .iter()
            .map(|l| {
                l.split(':')
                    .nth(1)
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse::<f64>().unwrap())
                    .collect_vec()
            })
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
    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult> {
        let races = self.parse(input);
        Some(races.into_iter().map(|r| self.solve_race(&r)).product::<u32>().into())
    }

    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult> {
        let race = self.parse2(input);
        Some(self.solve_race(&race).into())
    }
}
