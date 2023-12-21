use itertools::Itertools;

use std::{collections::HashMap, iter};

use aoc_lib::solution::Solution;
use aoc_lib::types::{Grid, IntoSome, ProblemInput, ProblemResult};

type CharGrid = Grid<char>;

pub struct Solution14;
impl Solution14 {
    fn parse(&self, input: ProblemInput) -> CharGrid {
        input.grid()
    }

    fn eval_load(grid: &CharGrid) -> usize {
        grid.iter()
            .enumerate()
            .map(|(i, row)| row.iter().filter(|c| **c == 'O').count() * (grid.len() - i))
            .sum()
    }

    fn cycle(grid: &CharGrid) -> CharGrid {
        let tilted_north = Self::tilt(grid, true, false);
        let tilted_west = Self::tilt(&tilted_north, false, false);
        let tilted_south = Self::tilt(&tilted_west, true, true);
        Self::tilt(&tilted_south, false, true)
    }

    fn tilt(grid: &CharGrid, transpose: bool, reverse: bool) -> CharGrid {
        let grid_t = Self::transpose(grid);
        let grid_iter = match transpose {
            false => grid,
            true => &grid_t,
        };

        let mapped_grid = match reverse {
            false => grid_iter.iter().map(|l| Self::tilt_line_left(l)).collect_vec(),
            true => grid_iter.iter().map(|l| Self::tilt_line_right(l)).collect_vec(),
        };

        match transpose {
            false => mapped_grid,
            true => Self::transpose(&mapped_grid),
        }
    }

    fn transpose(grid: &CharGrid) -> CharGrid {
        (0..grid[0].len())
            .map(|i| grid.iter().map(|inner| inner[i]).collect_vec())
            .collect()
    }

    fn tilt_line_left(line: &[char]) -> Vec<char> {
        let mut cube_pos = line
            .iter()
            .positions(|c| *c == '#')
            .map(|i| i.try_into().unwrap())
            .collect_vec();
        cube_pos.insert(0, -1i32);
        cube_pos.push(line.len() as i32);

        let section_lengths = cube_pos
            .iter()
            .skip(1)
            .zip(cube_pos.iter())
            .map(|(nxt, cur)| nxt - cur - 1)
            .collect_vec();

        let rounds_per_section = line
            .split(|c| *c == '#')
            .map(|section| section.iter().filter(|c| **c == 'O').count() as i32)
            .collect_vec();

        Itertools::intersperse(
            section_lengths
                .iter()
                .zip_eq(rounds_per_section.iter())
                .map(|(&sec_len, &rounds)| {
                    iter::repeat('O')
                        .take(rounds as usize)
                        .chain(iter::repeat('.').take((sec_len - rounds) as usize))
                        .collect_vec()
                }),
            vec!['#'],
        )
        .flatten()
        .collect_vec()
    }

    fn tilt_line_right(line: &[char]) -> Vec<char> {
        let rev_col = line.iter().rev().cloned().collect_vec();
        let rev_tilt = Self::tilt_line_left(&rev_col);
        rev_tilt.into_iter().rev().collect_vec()
    }

    #[allow(dead_code)]
    fn print_grid(grid: &CharGrid) {
        for row in grid {
            for c in row {
                print!("{c}");
            }
            println!();
        }
        println!();
    }
}

impl Solution for Solution14 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let grid = self.parse(input);
        let tilted = Self::tilt(&grid, true, false); // North
        Self::eval_load(&tilted).into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut dish = self.parse(input);
        let mut iterations = HashMap::from([(dish.clone(), 0)]);

        for i in 1.. {
            dish = Self::cycle(&dish);
            if let Some(prev_i) = iterations.get(&dish) {
                // Repetition found, calculate where the billionth iteration would fall into this cycle
                let modulus = i - prev_i;
                let offset = (1_000_000_000 - prev_i) % modulus;
                for _ in 0..offset {
                    dish = Self::cycle(&dish);
                }
                return Self::eval_load(&dish).into_some();
            }

            iterations.insert(dish.clone(), i);
        }

        unreachable!()
    }
}
