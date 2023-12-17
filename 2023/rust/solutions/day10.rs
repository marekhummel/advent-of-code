use std::collections::{HashMap, HashSet};

use aoc_lib::solution::Solution;
use aoc_lib::types::{Grid, IntoSome, ProblemInput, ProblemResult};
use aoc_lib::util::{Direction, Position};
use itertools::Itertools;

type CharGrid = Grid<char>;

const PRINT: bool = false;

pub struct Solution10;
impl Solution10 {
    fn parse(&self, input: ProblemInput) -> CharGrid {
        input.grid()
    }

    fn find_loop(&self, grid: &CharGrid) -> Vec<Position> {
        let start = grid
            .iter()
            .find_position(|row| row.iter().any(|&c| c == 'S'))
            .map(|(y, row)| Position {
                y,
                x: row.iter().position(|&c| c == 'S').unwrap(),
            })
            .unwrap();

        let mut path = vec![start];
        let (mut curr_pos, mut curr_off) = self.find_path_beginning(start, grid);
        while *curr_pos.grid_get(grid) != 'S' {
            path.push(curr_pos);
            (curr_pos, curr_off) = self.find_next_element(curr_pos, curr_off, grid);
        }

        path
    }

    fn find_path_beginning(&self, start: Position, grid: &CharGrid) -> (Position, Direction) {
        let (height, width) = (grid.len(), grid[0].len());
        if let Some(up) = start.advance_check(Direction::North, width, height) {
            let symbol = up.grid_get(grid);
            match symbol {
                '|' => return (up, Direction::North),
                '7' => return (up, Direction::West),
                'F' => return (up, Direction::East),
                _ => (),
            }
        }

        if let Some(down) = start.advance_check(Direction::South, width, height) {
            let symbol = down.grid_get(grid);
            match symbol {
                '|' => return (down, Direction::South),
                'J' => return (down, Direction::West),
                'L' => return (down, Direction::East),
                _ => (),
            }
        }

        if let Some(left) = start.advance_check(Direction::West, width, height) {
            let symbol = left.grid_get(grid);
            match symbol {
                '-' => return (left, Direction::West),
                'F' => return (left, Direction::South),
                'L' => return (left, Direction::North),
                _ => (),
            }
        }

        if let Some(right) = start.advance_check(Direction::East, width, height) {
            let symbol = right.grid_get(grid);
            match symbol {
                '-' => return (right, Direction::East),
                '7' => return (right, Direction::South),
                'J' => return (right, Direction::North),
                _ => (),
            }
        }

        unreachable!()
    }

    fn find_next_element(&self, pos: Position, dir: Direction, grid: &CharGrid) -> (Position, Direction) {
        let next_pos = pos.advance(dir);

        let next_dir = match (dir, next_pos.grid_get(grid)) {
            (_, 'S') => Direction::None,
            (Direction::West, '-') => Direction::West,
            (Direction::East, '-') => Direction::East,
            (Direction::North, '|') => Direction::North,
            (Direction::South, '|') => Direction::South,
            (Direction::East, '7') => Direction::South,
            (Direction::North, '7') => Direction::West,
            (Direction::West, 'F') => Direction::South,
            (Direction::North, 'F') => Direction::East,
            (Direction::West, 'L') => Direction::North,
            (Direction::South, 'L') => Direction::East,
            (Direction::East, 'J') => Direction::North,
            (Direction::South, 'J') => Direction::West,
            _ => panic!("unexpected symbol in path"),
        };

        (next_pos, next_dir)
    }

    fn compute_area(&self, grid: &CharGrid, loop_path: Vec<Position>) -> u32 {
        let s_tile = self.find_tile_below_s(&loop_path);
        let loop_lookup: HashSet<Position> = loop_path.into_iter().collect();

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
                let tile_in_loop = loop_lookup.contains(&Position { y, x });
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

        let dir_prev = prev.get_direction_to(s);
        let dir_next = s.get_direction_to(next);
        match (dir_prev, dir_next) {
            (Direction::North, Direction::North) | (Direction::South, Direction::South) => '|',
            (Direction::East, Direction::East) | (Direction::West, Direction::West) => '-',
            (Direction::North, Direction::East) | (Direction::West, Direction::South) => 'F',
            (Direction::South, Direction::East) | (Direction::West, Direction::North) => 'L',
            (Direction::South, Direction::West) | (Direction::East, Direction::North) => 'J',
            (Direction::North, Direction::West) | (Direction::East, Direction::South) => '7',
            _ => unreachable!("Where is S going"),
        }
    }
}

impl Solution for Solution10 {
    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult> {
        // Add one to account for odd-length paths
        ((self.find_loop(&self.parse(input)).len() + 1) / 2).into_some()
    }

    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult> {
        let grid = self.parse(input);
        let loop_path = self.find_loop(&grid);
        self.compute_area(&grid, loop_path).into_some()
    }
}
