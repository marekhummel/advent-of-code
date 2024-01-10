use std::collections::HashMap;

use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;
use regex::Regex;

struct Reindeer {
    name: String,
    speed: u16,
    fly_duration: u16,
    rest_time: u16,
}

pub struct Solution14;
impl Solution14 {
    fn parse(input: ProblemInput) -> Vec<Reindeer> {
        let line_rgx = Regex::new(r"(?P<reindeer>\w*?) can fly (?P<speed>\d*) km/s for (?P<duration>\d*) seconds, but then must rest for (?P<rest>\d*) seconds.").unwrap();

        input
            .lines()
            .into_iter()
            .map(|l| {
                let captures = line_rgx.captures(&l).unwrap();
                let name = captures.name("reindeer").unwrap().as_str().to_string();
                let speed = captures.name("speed").unwrap().as_str().parse::<u16>().unwrap();
                let fly_duration = captures.name("duration").unwrap().as_str().parse::<u16>().unwrap();
                let rest_time = captures.name("rest").unwrap().as_str().parse::<u16>().unwrap();

                Reindeer {
                    name,
                    speed,
                    fly_duration,
                    rest_time,
                }
            })
            .collect()
    }

    fn distance(reindeer: &Reindeer, race_length: u16) -> u16 {
        let flyrest = reindeer.fly_duration + reindeer.rest_time;
        let full_iterations = race_length / flyrest;
        let remaining_fly_duration = (race_length % flyrest).min(reindeer.fly_duration);
        (full_iterations * reindeer.fly_duration + remaining_fly_duration) * reindeer.speed
    }
}

impl Solution for Solution14 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        let race_length = if is_sample { 1000 } else { 2503 };

        let reindeers = Self::parse(input);
        reindeers
            .into_iter()
            .map(|reindeer| Self::distance(&reindeer, race_length))
            .max()
            .unwrap()
            .into_some()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        let race_length = if is_sample { 1000 } else { 2503 };

        let reindeers = Self::parse(input);
        let mut points = HashMap::new();
        for time in 1..=race_length {
            let leaders = reindeers
                .iter()
                .map(|reindeer| (reindeer.name.clone(), Self::distance(reindeer, time)))
                .max_set_by_key(|(_, d)| *d);

            for (l, _) in leaders {
                *points.entry(l).or_insert(0) += 1
            }
        }

        points.values().max().unwrap().into_some()
    }
}
