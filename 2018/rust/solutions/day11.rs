use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution11;
impl Solution11 {
    fn power_level(x: u32, y: u32, serial_number: u32) -> i32 {
        let rack = x + 10;
        let pl = (rack * y + serial_number) * rack;
        ((pl % 1000) / 100) as i32 - 5
    }

    fn summed_area_table(serial_number: u32) -> Vec<Vec<i32>> {
        // Summed area table: sat[y][x] = sum grid[y'][x'] with y' = 0..=y & x' = 0..=x
        let mut sat = vec![vec![0; 300]; 300];
        sat[0][0] = Self::power_level(1, 1, serial_number);
        for y in 1..300 {
            for x in 1..300 {
                let fuel_cell = Self::power_level(x as u32 + 1, y as u32 + 1, serial_number);
                sat[y][x] = sat[y - 1][x] + sat[y][x - 1] - sat[y - 1][x - 1] + fuel_cell;
            }
        }
        sat
    }

    fn best_fuel_array(sat: &[Vec<i32>], window_size: usize) -> (i32, usize, usize) {
        let mut best = (0, 0, 0);
        for y in 0..301 - window_size {
            for x in 0..301 - window_size {
                // Compute power by referring to the sat
                let mut power = sat[y + window_size - 1][x + window_size - 1];
                power -= if x > 0 { sat[y + window_size - 1][x - 1] } else { 0 };
                power -= if y > 0 { sat[y - 1][x + window_size - 1] } else { 0 };
                power += if x > 0 && y > 0 { sat[y - 1][x - 1] } else { 0 };

                if power > best.0 {
                    best = (power, x + 1, y + 1);
                }
            }
        }

        best
    }
}

impl Solution for Solution11 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let serial_number: u32 = input.string().parse().unwrap();
        let sat = Self::summed_area_table(serial_number);

        let best_fuel_cell = Self::best_fuel_array(&sat, 3);
        format!("{},{}", best_fuel_cell.1, best_fuel_cell.2).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let serial_number: u32 = input.string().parse().unwrap();
        let sat = Self::summed_area_table(serial_number);

        let (ws, (_, x, y)) = (2..=300)
            .map(|ws| (ws, Self::best_fuel_array(&sat, ws)))
            .max_by_key(|(_, (power, _, _))| *power)
            .unwrap();
        format!("{},{},{}", x, y, ws).to_result()
    }
}
