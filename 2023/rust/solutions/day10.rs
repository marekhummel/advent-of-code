use std::collections::{HashMap, HashSet};

use crate::solution::{ProblemInput, ProblemResult, Solution};
use itertools::Itertools;

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);
type Offset = (i32, i32);

const LEFT: Offset = (0, -1);
const RIGHT: Offset = (0, 1);
const UP: Offset = (-1, 0);
const DOWN: Offset = (1, 0);

const PRINT: bool = true;

pub struct Solution10;
impl Solution10 {
    fn parse(&self, input: ProblemInput) -> Grid {
        input.into_iter().map(|line| line.chars().collect_vec()).collect_vec()
    }

    fn find_loop(&self, grid: &Grid) -> Vec<Position> {
        let start = grid
            .iter()
            .find_position(|row| row.iter().any(|&c| c == 'S'))
            .map(|(i, row)| (i, row.iter().position(|&c| c == 'S').unwrap()))
            .unwrap();

        let mut path = vec![start];
        let (mut curr_pos, mut curr_off) = self.find_path_beginning(start, &grid);
        while grid[curr_pos.0][curr_pos.1] != 'S' {
            path.push(curr_pos);
            (curr_pos, curr_off) = self.find_next_element(curr_pos, curr_off, &grid);
        }

        // println!("{path:?}, {0}", path.len());
        // println!("{0:?}", path.iter().map(|&(y, x)| grid[y][x]).collect_vec());
        path
    }

    fn find_path_beginning(&self, start: Position, grid: &Grid) -> (Position, Offset) {
        if start.0 > 0 {
            let symbol = grid[start.0 - 1][start.1];

            match symbol {
                '|' => return ((start.0 - 1, start.1), UP),
                '7' => return ((start.0 - 1, start.1), LEFT),
                'F' => return ((start.0 - 1, start.1), RIGHT),
                _ => (),
            }
        }

        if start.0 < grid.len() - 1 {
            let symbol = grid[start.0 + 1][start.1];
            match symbol {
                '|' => return ((start.0 + 1, start.1), DOWN),
                'J' => return ((start.0 + 1, start.1), LEFT),
                'L' => return ((start.0 + 1, start.1), RIGHT),
                _ => (),
            }
        }

        if start.1 > 0 {
            let symbol = grid[start.0][start.1 - 1];
            match symbol {
                '-' => return ((start.0, start.1 - 1), LEFT),
                'F' => return ((start.0, start.1 - 1), DOWN),
                'L' => return ((start.0, start.1 - 1), UP),
                _ => (),
            }
        }

        if start.1 < grid[start.0].len() - 1 {
            let symbol = grid[start.0][start.1 + 1];
            match symbol {
                '-' => return ((start.0, start.1 + 1), RIGHT),
                '7' => return ((start.0, start.1 + 1), DOWN),
                'J' => return ((start.0, start.1 + 1), UP),
                _ => (),
            }
        }

        unreachable!()
    }

    fn find_next_element(&self, pos: Position, offset: Offset, grid: &Grid) -> (Position, Offset) {
        let next_pos = ((pos.0 as i32 + offset.0) as usize, (pos.1 as i32 + offset.1) as usize);

        let next_off = match (offset, grid[next_pos.0][next_pos.1]) {
            (_, 'S') => (0, 0),
            (LEFT, '-') => LEFT,
            (RIGHT, '-') => RIGHT,
            (UP, '|') => UP,
            (DOWN, '|') => DOWN,
            (RIGHT, '7') => DOWN,
            (UP, '7') => LEFT,
            (LEFT, 'F') => DOWN,
            (UP, 'F') => RIGHT,
            (LEFT, 'L') => UP,
            (DOWN, 'L') => RIGHT,
            (RIGHT, 'J') => UP,
            (DOWN, 'J') => LEFT,
            _ => panic!("unexpected symbol in path"),
        };

        (next_pos, next_off)
    }

    fn compute_area(&self, grid: &Grid, loop_path: Vec<Position>) -> u32 {
        let s_tile = self.find_tile_below_s(&loop_path);
        let loop_lookup: HashSet<(usize, usize)> = loop_path.into_iter().collect();

        // Define what tiles toggle the parity. Only choose either side of corners (JL or F7)
        let mut toggle_tiles: HashSet<char> = "|JL".chars().collect();
        if toggle_tiles.contains(&s_tile) {
            toggle_tiles.insert('S');
        }

        let tile_replacement: HashMap<char, char> = "S|-F7LJ".chars().zip_eq("S║═╔╗╚╝".chars()).collect();

        let mut area = 0u32;
        for (y, row) in grid.iter().enumerate() {
            let mut inside_loop = false;
            for (x, tile) in row.iter().enumerate() {
                let tile_in_loop = loop_lookup.contains(&(y, x));
                inside_loop ^= tile_in_loop && toggle_tiles.contains(tile);
                area += (inside_loop && !tile_in_loop) as u32;

                if PRINT {
                    if tile_in_loop {
                        print!("{0}", tile_replacement.get(tile).copied().unwrap());
                    } else {
                        print!("{0}", if inside_loop && !tile_in_loop { "∙" } else { " " });
                    }
                }
            }
            if PRINT {
                println!()
            }
        }

        area
    }

    fn find_tile_below_s(&self, loop_path: &[Position]) -> char {
        let s = loop_path[0];
        let next = loop_path[1];
        let prev = loop_path.last().copied().unwrap();

        let dir_prev = (s.0 as i32 - prev.0 as i32, s.1 as i32 - prev.1 as i32);
        let dir_next = (next.0 as i32 - s.0 as i32, next.1 as i32 - s.1 as i32);

        match (dir_prev, dir_next) {
            (UP, UP) | (DOWN, DOWN) => '|',
            (RIGHT, RIGHT) | (LEFT, LEFT) => '-',
            (UP, RIGHT) | (LEFT, DOWN) => 'F',
            (DOWN, RIGHT) | (LEFT, UP) => 'L',
            (DOWN, LEFT) | (RIGHT, UP) => 'J',
            (UP, LEFT) | (RIGHT, DOWN) => '7',
            _ => unreachable!("Where is S going"),
        }
    }
}

impl Solution for Solution10 {
    fn get_day(&self) -> u8 {
        10
    }

    fn solve_version01(&self, input: ProblemInput) -> ProblemResult {
        // Add one to account for odd-length paths
        ((self.find_loop(&self.parse(input)).len() + 1) / 2).try_into().unwrap()
    }

    fn solve_version02(&self, input: ProblemInput) -> ProblemResult {
        let grid = self.parse(input);
        let loop_path = self.find_loop(&grid);
        self.compute_area(&grid, loop_path).into()
    }
}
