use itertools::Itertools;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
struct MapEntry {
    dst: u64,
    src: u64,
    len: u64,
}

impl MapEntry {
    fn in_range(&self, value: u64) -> bool {
        (self.src..(self.src + self.len)).contains(&value)
    }

    fn map(&self, value: u64) -> u64 {
        self.dst + (value - self.src)
    }
}

#[derive(Clone, Copy)]
struct SeedRange {
    first: u64,
    len: u64,
}

impl SeedRange {
    fn last(&self) -> u64 {
        self.first + self.len - 1
    }
}

type Seed = u64;
type Map = Vec<MapEntry>;

pub struct Solution05;

impl Solution05 {
    fn parse(&self, input: ProblemInput) -> (Vec<Seed>, Vec<Map>) {
        let seeds = input
            .lines()
            .get(0)
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect_vec();

        let mut maps = vec![];
        let mut current_map = vec![];

        for line in input.lines().iter().skip(2) {
            if line.chars().all(char::is_whitespace) {
                maps.push(current_map);
                current_map = vec![];
                continue;
            }

            if line.ends_with("map:") {
                continue;
            }

            let values = line
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .take(3)
                .collect_tuple()
                .unwrap();
            let (d, s, l) = values;
            current_map.push(MapEntry { dst: d, src: s, len: l });
        }

        maps.push(current_map);
        (seeds, maps)
    }

    fn parse2(&self, lines: ProblemInput) -> (Vec<SeedRange>, Vec<Map>) {
        let (seeds, maps) = self.parse(lines);
        let seed_ranges = seeds
            .into_iter()
            .tuples()
            .map(|(s, l)| SeedRange { first: s, len: l })
            .collect();
        (seed_ranges, maps)
    }

    fn apply_map(&self, value: Seed, map: &Map) -> Seed {
        map.iter()
            .find(|entry| entry.in_range(value))
            .map_or(value, |entry| entry.map(value))
    }

    fn apply_map2(&self, values: Vec<SeedRange>, map: &Map) -> Vec<SeedRange> {
        let mut mapped_values = vec![];
        let mut remaining_values = values;

        loop {
            let mut new_values = vec![];

            for value in remaining_values.iter() {
                let mut is_mapped = false;
                for entry in map {
                    let init_fits = entry.in_range(value.first);
                    let tail_fits = entry.in_range(value.last());

                    if init_fits && tail_fits {
                        mapped_values.push(SeedRange {
                            first: entry.map(value.first),
                            len: value.len,
                        });
                        is_mapped = true;
                        break;
                    } else if init_fits {
                        let mapped_len = (entry.src + entry.len) - value.first;
                        mapped_values.push(SeedRange {
                            first: entry.map(value.first),
                            len: mapped_len,
                        });
                        new_values.push(SeedRange {
                            first: value.first + mapped_len,
                            len: value.len - mapped_len,
                        });
                        is_mapped = true;
                        break;
                    } else if tail_fits {
                        let mapped_len = (value.first + value.len) - entry.src;
                        mapped_values.push(SeedRange {
                            first: entry.dst,
                            len: mapped_len,
                        });
                        new_values.push(SeedRange {
                            first: value.first,
                            len: value.len - mapped_len,
                        });
                        is_mapped = true;
                        break;
                    }

                    // Forgot to check if middle fits
                    // (so that the middle is mapped and init and tail have to be rechecked)
                    // Apparently thats not a problem
                }

                if !is_mapped {
                    mapped_values.push(*value);
                }
            }

            if new_values.is_empty() {
                break;
            }

            remaining_values = new_values;
        }

        mapped_values
    }
}

impl Solution for Solution05 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (seeds, maps) = self.parse(input);
        seeds
            .iter()
            .map(|s| maps.iter().fold(*s, |acc, m| self.apply_map(acc, m)))
            .min()
            .unwrap()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (seeds, maps) = self.parse2(input);
        seeds
            .into_iter()
            .flat_map(|s| maps.iter().fold(vec![s], |acc, m| self.apply_map2(acc, m)))
            .map(|sr| sr.first)
            .min()
            .unwrap()
            .to_result()
    }
}
