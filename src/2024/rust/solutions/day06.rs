use std::collections::HashSet;

use aoc_lib::cartesian::{Direction, Grid, Index};
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let map = input.grid();
    let guard = map.enumerate().find(|&(_, &c)| c == '^').unwrap().0;

    compute_path(&map, guard).0.len().into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut map = input.grid();
    let guard = map.enumerate().find(|&(_, &c)| c == '^').unwrap().0;
    let original_path = compute_path(&map, guard).0;

    let mut loops: usize = 0;
    for pos in original_path {
        if *map.get(&pos) != '.' {
            continue;
        }

        map.set(&pos, '#');
        let pathloop = compute_path(&map, guard);
        if pathloop.1 {
            loops += 1;
        }
        map.set(&pos, '.');
    }

    loops.into()
}

fn compute_path(map: &Grid<char>, guard_start: Index) -> (Vec<Index>, bool) {
    let mut guard = guard_start;
    let mut dir = Direction::North;
    let mut visited = HashSet::<(Index, Direction)>::new();
    loop {
        if !visited.insert((guard, dir)) {
            return (visited.into_iter().map(|(idx, _)| idx).unique().collect_vec(), true);
        }

        let next = match guard.advance_check(dir, map.size) {
            Some(pos) => pos,
            None => return (visited.into_iter().map(|(idx, _)| idx).unique().collect_vec(), false),
        };

        match map.get(&next) {
            '#' => dir = dir.right(),
            '.' | '^' => guard = next,
            _ => unreachable!(),
        }
    }
}
