use std::collections::{HashMap, HashSet};

use aoc_lib::cartesian::Position;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use regex::Regex;

pub struct Solution15;
impl Solution15 {
    fn parse(input: ProblemInput) -> Vec<(Position, Position)> {
        let rgx = Regex::new(r"(?:x|y)=(-?\d+)").unwrap();
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (sx, sy, bx, by) = rgx
                    .captures_iter(&l)
                    .map(|c| c.get(1).unwrap().as_str().parse().unwrap())
                    .collect_tuple()
                    .unwrap();

                let sensor = Position::new(sx, sy);
                let beacon = Position::new(bx, by);
                (sensor, beacon)
            })
            .collect()
    }
}

impl Solution for Solution15 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I128(26),
            ProblemResult::I128(5040643),
            ProblemResult::I128(56000011),
            ProblemResult::I128(11016575214126),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let sensors = Self::parse(input);
        let row: i128 = if is_sample { 10 } else { 2_000_000 };

        let mut blocked = Vec::new();
        let mut beacons = HashSet::new();
        // Mark overlaps with significant row
        for (sensor, beacon) in sensors {
            let range = sensor.dist(&beacon);
            let y_dist = row.abs_diff(sensor.y);

            // Sensor won't affect row
            if y_dist > range {
                continue;
            }

            if beacon.y == row {
                beacons.insert(beacon.x);
            }

            let half_width = (range - y_dist) as i128;
            blocked.push((sensor.x - half_width, sensor.x + half_width));
        }

        // Reduce intervals (combine overlapping) to compute size
        loop {
            let mut new_blocked = Vec::new();
            let before = blocked.len();

            // Merge intervals
            let mut merged = HashSet::new();
            for (i, j) in (0..blocked.len()).tuple_combinations() {
                if merged.contains(&i) || merged.contains(&j) {
                    continue;
                }

                let (a0, b0) = blocked[i];
                let (a1, b1) = blocked[j];
                if a0 <= b1 && a1 <= b0 {
                    new_blocked.push((a0.min(a1), b0.max(b1)));
                    merged.extend([i, j]);
                }
            }

            // Remove merged ones and extend with new ones
            for i in merged.into_iter().sorted().rev() {
                blocked.remove(i);
            }
            blocked.extend(new_blocked);

            // No more merges
            if blocked.len() == before {
                break;
            }
        }

        let blocked_count = blocked.into_iter().map(|(a, b)| b - a + 1).sum::<i128>();
        (blocked_count - beacons.len() as i128).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let sensors = Self::parse(input);
        let bound: i128 = if is_sample { 20 } else { 4_000_000 };

        // Transform areas convered by sensors into squares where diagonals are main axes
        let ranges = sensors
            .iter()
            .map(|(s, b)| {
                let range = s.dist(b) as i128;
                let (left, right) = (s.x - range, s.x + range + 1); // exclusive end
                let min = Position::new(left + s.y, left - s.y);
                let max = Position::new(right + s.y, right - s.y);
                (min, max)
            })
            .collect_vec();

        // Create map of all mentioned xs and ys
        let positions = ranges.iter().flat_map(|(s, e)| [s, e]).sorted().collect_vec();
        let xx = positions.iter().map(|p| p.x).unique().sorted().collect_vec();
        let yy = positions.iter().map(|p| p.y).unique().sorted().collect_vec();

        // Use grid of which is reduced to mentioned xs and ys (initially all free)
        // And have dictionary which maps actual x and y to indices in grid
        let mut grid = vec![vec![false; xx.len() - 1]; yy.len() - 1];
        let xx_map: HashMap<_, _> = xx.iter().enumerate().map(|(ex, x)| (x, ex)).collect();
        let yy_map: HashMap<_, _> = yy.iter().enumerate().map(|(ey, y)| (y, ey)).collect();

        // Loop over blocked areas and mark in grid
        for (start, end) in ranges {
            for ay in yy_map[&start.y]..yy_map[&end.y] {
                for ax in xx_map[&start.x]..xx_map[&end.x] {
                    grid[ay][ax] = true;
                }
            }
        }

        // Find area which is not blocked
        let valid_area = 0..=bound;
        for ay in 0..grid.len() {
            for ax in 0..grid[ay].len() {
                if !grid[ay][ax] {
                    // Convert area indices to diagonal indices
                    let dx = xx[ax];
                    let dy = yy[ay];

                    // Convert diagonal indices to normal indices
                    let x = (dx + dy) / 2;
                    let y = (dx - dy) / 2;
                    if valid_area.contains(&x) && valid_area.contains(&y) {
                        return (x * 4_000_000 + y).to_result();
                    }
                }
            }
        }

        unreachable!()
    }
}
