use aoc_lib::cartesian::Grid;
use itertools::Itertools;

use std::{collections::HashMap, iter};

use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let grid = parse(input);
    let tilted = tilt(&grid, true, false); // North
    eval_load(&tilted).into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut dish = parse(input);
    let mut iterations = HashMap::from([(dish.clone(), 0)]);

    for i in 1.. {
        dish = cycle(&dish);
        if let Some(prev_i) = iterations.get(&dish) {
            // Repetition found, calculate where the billionth iteration would fall into this cycle
            let modulus = i - prev_i;
            let offset = (1_000_000_000 - prev_i) % modulus;
            for _ in 0..offset {
                dish = cycle(&dish);
            }
            return eval_load(&dish).into();
        }

        iterations.insert(dish.clone(), i);
    }

    unreachable!()
}

type CharGrid = Grid<char>;

fn parse(input: ProblemInput) -> CharGrid {
    input.grid()
}

fn eval_load(grid: &CharGrid) -> usize {
    grid.enumerate()
        .filter(|(_, c)| **c == 'O')
        .map(|(idx, _)| grid.size.height - idx.j)
        .sum()
}

fn cycle(grid: &CharGrid) -> CharGrid {
    let tilted_north = tilt(grid, true, false);
    let tilted_west = tilt(&tilted_north, false, false);
    let tilted_south = tilt(&tilted_west, true, true);
    tilt(&tilted_south, false, true)
}

fn tilt(grid: &CharGrid, transpose: bool, reverse: bool) -> CharGrid {
    let grid_t = grid.transpose();
    let grid_iter = match transpose {
        false => grid,
        true => &grid_t,
    };

    let mapped_grid = Grid::new(match reverse {
        false => grid_iter.rows.iter().map(|l| tilt_line_left(l)).collect_vec(),
        true => grid_iter.rows.iter().map(|l| tilt_line_right(l)).collect_vec(),
    });

    match transpose {
        false => mapped_grid,
        true => mapped_grid.transpose(),
    }
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
                iter::repeat_n('O', rounds as usize)
                    .chain(iter::repeat_n('.', (sec_len - rounds) as usize))
                    .collect_vec()
            }),
        vec!['#'],
    )
    .flatten()
    .collect_vec()
}

fn tilt_line_right(line: &[char]) -> Vec<char> {
    let rev_col = line.iter().rev().cloned().collect_vec();
    let rev_tilt = tilt_line_left(&rev_col);
    rev_tilt.into_iter().rev().collect_vec()
}
