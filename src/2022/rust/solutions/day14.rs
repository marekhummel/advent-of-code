use aoc_lib::cartesian::{Direction, Grid, Index, Size};
use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut cave = parse(input);
    let sand = (0u32..).find(|_| produce_sand(&mut cave, true)).unwrap();
    sand.into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut cave = parse(input);
    let sand = (1u32..).find(|_| produce_sand(&mut cave, false)).unwrap();
    sand.into()
}

fn parse(input: ProblemInput) -> Grid<char> {
    let corners = input
        .lines()
        .into_iter()
        .map(|l| {
            l.split(" -> ")
                .map(|s| Index::from_tuple(s.split(',').parsed().collect_tuple().unwrap()))
                .collect_vec()
        })
        .collect_vec();

    let width = corners.iter().flatten().map(|idx| idx.i).max().unwrap() + 1;
    let height = corners.iter().flatten().map(|idx| idx.j).max().unwrap() + 2;

    // Max i will be the height plus start, if the end result is a perfect pyramid
    let mut cave = Grid::empty(Size::new(width.max(500 + height), height), '.');
    for structure in corners {
        for (start, end) in structure.into_iter().tuple_windows() {
            if start.i == end.i {
                for j in start.j.min(end.j)..=start.j.max(end.j) {
                    cave.set(&Index::new(start.i, j), '#');
                }
            } else if start.j == end.j {
                for i in start.i.min(end.i)..=start.i.max(end.i) {
                    cave.set(&Index::new(i, start.j), '#');
                }
            }
        }
    }

    cave
}

fn produce_sand(cave: &mut Grid<char>, abyss: bool) -> bool {
    let mut pos = Index::new(500, 0);

    loop {
        if let Some(down) = pos.advance_check(Direction::South, cave.size) {
            if *cave.get(&down) == '.' {
                pos = down;
                continue;
            }

            let down_left = down.advance(Direction::West);
            if *cave.get(&down_left) == '.' {
                pos = down_left;
                continue;
            }

            let down_right = down.advance(Direction::East);
            if *cave.get(&down_right) == '.' {
                pos = down_right;
                continue;
            }
        } else if abyss {
            // Bottom of grid reached, if part 1 this is the sentinel
            return true;
        }

        cave.set(&pos, 'o');
        return pos == Index::new(500, 0);
    }
}
