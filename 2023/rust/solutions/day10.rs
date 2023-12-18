use aoc_lib::solution::Solution;
use aoc_lib::types::{Grid, IntoSome, ProblemInput, ProblemResult};
use aoc_lib::util::{Direction, Index};
use itertools::Itertools;

type CharGrid = Grid<char>;

pub struct Solution10;
impl Solution10 {
    fn parse(&self, input: ProblemInput) -> CharGrid {
        input.grid()
    }

    fn find_loop(&self, grid: &CharGrid) -> Vec<Index> {
        let start = grid
            .iter()
            .find_position(|row| row.iter().any(|&c| c == 'S'))
            .map(|(y, row)| Index {
                j: y,
                i: row.iter().position(|&c| c == 'S').unwrap(),
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

    fn find_path_beginning(&self, start: Index, grid: &CharGrid) -> (Index, Direction) {
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

    fn find_next_element(&self, pos: Index, dir: Direction, grid: &CharGrid) -> (Index, Direction) {
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

    fn compute_area(&self, grid: &CharGrid, loop_path: Vec<Index>) -> u64 {
        let mut vertices = loop_path
            .iter()
            .filter(|idx| "SFL7J".contains(*idx.grid_get(grid)))
            .collect_vec();
        vertices.push(vertices[0]);

        // Shoelace formula
        let mut area_inside = 0i64;
        for (v, nv) in vertices.iter().zip(vertices.iter().skip(1)) {
            area_inside += (v.j as i64 + nv.j as i64) * (v.i as i64 - nv.i as i64);
        }
        area_inside /= 2;

        // Pick's theorem
        let boundary = loop_path.len() as u64;
        area_inside.unsigned_abs() - boundary / 2 + 1
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
