use std::collections::HashMap;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

type Date = (u16, u16, u16);
type Time = (u8, u8);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Record {
    ShiftBegin(u32),
    Sleep,
    Wakeup,
}

pub struct Solution04;
impl Solution04 {
    fn parse(input: ProblemInput) -> Vec<(Date, Time, Record)> {
        input
            .lines()
            .into_iter()
            .map(|line| {
                let (stamp, action) = line.split_once("] ").unwrap();
                let (date_str, time_str) = stamp.trim_start_matches('[').split_once(' ').unwrap();

                let date = date_str.split('-').parsed().collect_tuple().unwrap();
                let time = time_str.split(':').parsed().collect_tuple().unwrap();

                let record = match &action.split(' ').collect_vec()[..] {
                    ["Guard", id, "begins", "shift"] => Record::ShiftBegin(id.trim_start_matches('#').parse().unwrap()),
                    ["falls", "asleep"] => Record::Sleep,
                    ["wakes", "up"] => Record::Wakeup,
                    _ => unreachable!(),
                };

                (date, time, record)
            })
            .sorted()
            .collect()
    }

    fn create_sleep_map(records: &[(Date, Time, Record)]) -> HashMap<u32, Vec<u32>> {
        let mut sleep_map = HashMap::new();
        let mut on_duty = 0;
        let mut asleep_since = None;
        for (_, time, record) in records {
            match record {
                Record::ShiftBegin(id) => on_duty = *id,
                Record::Sleep => asleep_since = Some(time.1),
                Record::Wakeup => {
                    let Some(start) = asleep_since else { unreachable!()};
                    let chart = sleep_map.entry(on_duty).or_insert(vec![0; 60]);
                    for minute in start..time.1 {
                        chart[minute as usize] += 1;
                    }
                }
            }
        }

        sleep_map
    }
}

impl Solution for Solution04 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let records = Self::parse(input);
        let sleep_map = Self::create_sleep_map(&records);

        let (id, sleep_chart) = sleep_map
            .into_iter()
            .max_by_key(|(_, map)| map.iter().sum::<u32>())
            .unwrap();

        let best_minute = sleep_chart.into_iter().position_max_by_key(|count| *count).unwrap();

        (id * best_minute as u32).into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let records = Self::parse(input);
        let sleep_map = Self::create_sleep_map(&records);

        let (id, sleepiest_minute) = sleep_map
            .into_iter()
            .map(|(id, map)| (id, map.into_iter().enumerate().max_by_key(|(_, c)| *c).unwrap()))
            .max_by_key(|(_, (_, c))| *c)
            .unwrap();

        (id * sleepiest_minute.0 as u32).into_some()
    }
}
