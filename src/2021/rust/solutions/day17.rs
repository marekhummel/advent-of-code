use std::collections::HashSet;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution17;
impl Solution17 {
    fn parse(input: ProblemInput) -> ((i32, i32), (i32, i32)) {
        let line = input.string();
        let target_area = line.trim_start_matches("target area: ");
        let (x_range, y_range) = target_area.split_once(", ").unwrap();
        let (x_min, x_max) = x_range.trim_start_matches("x=").split_once("..").unwrap();
        let (y_min, y_max) = y_range.trim_start_matches("y=").split_once("..").unwrap();

        (
            (x_min.parse().unwrap(), x_max.parse().unwrap()),
            (y_min.parse().unwrap(), y_max.parse().unwrap()),
        )
    }

    fn find_valid_initial_velocities(area_x: (i32, i32), area_y: (i32, i32)) -> HashSet<(i32, i32)> {
        // X and Y are independent. Thus, first compute valid Y velocites, and then based on that compute valid X velocities.
        //
        // We can skip the simulation for positive initial Y velocities vy0 with following observations:
        //  - Y velocity has a constant decrease of 1 per step, making the position a quadratic.
        //  - Each Y position the probe is reaching on its way up, will be encountered on its way down (especially y == 0).
        //
        // => If vy0 > 0, then the probe has velocity vy1 = -vy0 - 1 when at position y == 0, and it takes vy0 * 2 + 1
        //    steps to reach v == 0 again.
        // So we can forward the computation to the point where y == 0 and vy is negative.

        assert!(area_x.0 > 0); // Assume positive X area
        assert!(area_y.1 < 0); // Assume negative Y area so that the point y == 0 && vy < 0 will be reached.

        let mut valid_velocities = HashSet::new();
        // This range makes sure, that we won't overshoot the area within one step.
        for vy0 in area_y.0..=-area_y.0 {
            // Advance Y position and time to where y == 0 and vy <= 0
            let mut t = if vy0 > 0 { vy0 * 2 + 1 } else { 0 };
            let mut vy = if vy0 > 0 { -vy0 - 1 } else { vy0 };
            let mut y = 0;

            // Simulate while we are above the lower Y bound of the area
            while y >= area_y.0 {
                // If we are in the area, test initial X velocities
                if y <= area_y.1 {
                    for vx0 in 1.. {
                        // Compute x pos based on vx0 and time passed
                        let x = (1..=vx0).rev().take(t as usize).sum::<i32>();
                        if x >= area_x.0 {
                            // Overshot area
                            if x > area_x.1 {
                                break;
                            }
                            valid_velocities.insert((vx0, vy0));
                        }
                    }
                }

                // Advance probe by one time step
                y += vy;
                vy -= 1;
                t += 1;
            }
        }

        valid_velocities
    }
}

impl Solution for Solution17 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I32(45),
            ProblemResult::I32(2775),
            ProblemResult::USize(112),
            ProblemResult::USize(1566),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (area_x, area_y) = Self::parse(input);

        let valid_velocities = Self::find_valid_initial_velocities(area_x, area_y);
        let highest_vy0 = valid_velocities.into_iter().map(|(_, vy)| vy).max().unwrap();

        // We reached the highest point at vy0 * (vy0 + 1) / 2, just the sum of the integers from 1 to vy0.
        let highest_y = highest_vy0 * (highest_vy0 + 1) / 2;
        highest_y.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (area_x, area_y) = Self::parse(input);

        let valid_velocities = Self::find_valid_initial_velocities(area_x, area_y);
        valid_velocities.len().to_result()
    }
}
