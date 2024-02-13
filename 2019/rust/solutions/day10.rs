use std::f64::consts::PI;

use aoc_lib::cartesian::Index;
use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use num::Integer;

pub struct Solution10;
impl Solution10 {
    fn dir(station: &Index, asteroid: &Index) -> (i32, i32) {
        let dx = asteroid.i as i32 - station.i as i32;
        let dy = asteroid.j as i32 - station.j as i32;
        let gcd = dx.gcd(&dy);

        (dx.checked_div(gcd).unwrap_or(dx), dy.checked_div(gcd).unwrap_or(dy))
    }

    fn angle(dir: (i32, i32)) -> f64 {
        let (dx, dy) = dir;

        let two_pi = 2.0 * PI;
        let mut angle = (-dy as f64).atan2(dx as f64); // invert y because y axis is flipped in unit circle
        angle += PI; // move range from [-pi .. pi] to [0 .. 2pi]
        angle = two_pi - angle; // flip rotation of laser
        angle = (angle - PI / 2.0).rem_euclid(two_pi); // start upwards

        angle
    }

    fn best_station(asteroids: &[Index]) -> (Index, usize) {
        asteroids
            .iter()
            .map(|station| {
                (
                    *station,
                    asteroids
                        .iter()
                        .filter(|asteroid| station != *asteroid)
                        .map(|asteroid| Self::dir(station, asteroid))
                        .unique()
                        .count(),
                )
            })
            .max_by_key(|(_, count)| *count)
            .unwrap()
    }
}

impl Solution for Solution10 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let map = input.grid().map_elements(|c| *c == '#');
        let asteroids = map.enumerate().filter(|(_, a)| **a).map(|(idx, _)| idx).collect_vec();

        let (_, in_sight) = Self::best_station(&asteroids);
        in_sight.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let map = input.grid().map_elements(|c| *c == '#');
        let asteroids = map.enumerate().filter(|(_, a)| **a).map(|(idx, _)| idx).collect_vec();

        let (station, _) = Self::best_station(&asteroids);

        // Group asteroids based on dir vector, and then sort by angle.
        // For each dir vector, have a list of asteroids on that line, starting with the furthest.
        let mut targets = asteroids
            .iter()
            .filter(|a| **a != station)
            .into_group_map_by(|asteroid| Self::dir(&station, asteroid))
            .into_iter()
            .sorted_by(|(a, _), (b, _)| Self::angle(*a).total_cmp(&Self::angle(*b)))
            .map(|(_, asteroid_line)| {
                asteroid_line
                    .into_iter()
                    .sorted_by_key(|a| a.dist(&station))
                    .rev()
                    .collect_vec()
            })
            .collect_vec();

        // Need 200 asteroids at least, otherwise infinite loop
        assert!(asteroids.len() >= 200, "not enough asteroids");

        // Loop through all asteroids (sorted by angle) and for each line disintegrate (=pop) the closest one
        let mut idx = 0;
        for _ in 0..200 - 1 {
            while targets[idx].is_empty() {
                idx = (idx + 1) % targets.len();
            }

            targets.get_mut(idx).unwrap().pop();
            idx = (idx + 1) % targets.len();
        }

        // Find next asteroid in list
        let bet = targets
            .into_iter()
            .cycle()
            .skip(idx)
            .find(|asteroids| !asteroids.is_empty())
            .unwrap()[0];

        (bet.i * 100 + bet.j).to_result()
    }
}
