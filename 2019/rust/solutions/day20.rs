use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;

use aoc_lib::cartesian::{Direction, Grid, Index};
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tile {
    None,
    Path,
    Wall,
    Portal(String, Direction, bool),
}

impl Tile {
    fn is_entrance(&self) -> bool {
        if let Tile::Portal(name, _, _) = self {
            if name == "AA" {
                return true;
            }
        }

        false
    }

    fn is_exit(&self) -> bool {
        if let Tile::Portal(name, _, _) = self {
            if name == "ZZ" {
                return true;
            }
        }

        false
    }

    fn portal_name(&self) -> Option<&str> {
        match self {
            Tile::Portal(name, _, _) => Some(name),
            _ => None,
        }
    }
}

pub struct Solution20;
impl Solution20 {
    fn parse(input: ProblemInput) -> Grid<Tile> {
        let char_grid = input.grid();
        let mut grid = char_grid.clone().map_elements(|ch| match ch {
            '#' => Tile::Wall,
            '.' => Tile::Path,
            _ => Tile::None,
        });
        let mut seen = HashSet::new();
        let size = char_grid.size;

        // Parse portals
        for j in 0..size.height {
            for i in 0..size.width {
                let idx = Index { i, j };
                if seen.contains(&idx) {
                    continue;
                }

                let letter = char_grid.get(&idx);
                if !letter.is_ascii_alphabetic() {
                    continue;
                }

                seen.insert(idx);

                let south = idx.advance(Direction::South);
                let letter_south = char_grid.get(&south);
                if letter_south.is_ascii_alphabetic() {
                    seen.insert(south);
                    let portal = format!("{letter}{letter_south}");
                    let outer = idx.i < 2 || idx.j < 2 || idx.i >= size.width - 2 || idx.j >= size.height - 2;
                    if let Some('.') = char_grid.get_checked(&idx.advance(Direction::North)) {
                        grid.set(&idx, Tile::Portal(portal, Direction::North, outer));
                    } else if let Some('.') = char_grid.get_checked(&south.advance(Direction::South)) {
                        grid.set(&south, Tile::Portal(portal, Direction::South, outer));
                    }
                }

                let east = idx.advance(Direction::East);
                let letter_east = char_grid.get(&east);
                if letter_east.is_ascii_alphabetic() {
                    seen.insert(east);
                    let portal = format!("{letter}{letter_east}");
                    let outer = idx.i < 2 || idx.j < 2 || idx.i >= size.width - 2 || idx.j >= size.height - 2;

                    if let Some('.') = char_grid.get_checked(&idx.advance(Direction::West)) {
                        grid.set(&idx, Tile::Portal(portal, Direction::West, outer));
                    } else if let Some('.') = char_grid.get_checked(&east.advance(Direction::East)) {
                        grid.set(&east, Tile::Portal(portal, Direction::East, outer))
                    }
                }
            }
        }

        grid
    }

    fn find_exit(grid: &Grid<Tile>, use_levels: bool) -> usize {
        let portals = Self::pair_portals(grid);

        // Find entrance
        let (start_portal, tile) = grid.enumerate().find(|(_, tile)| tile.is_entrance()).unwrap();
        let Tile::Portal(_, dir, _) = tile else { unreachable!() };
        let start = start_portal.advance(*dir);

        // BFS (Could collapse grid to graph, but not needed here)
        let mut seen = HashSet::from([(start_portal, 0)]);
        let mut queue = BinaryHeap::from([(Reverse(0), Reverse(0), start)]);
        while let Some((Reverse(steps), Reverse(level), idx)) = queue.pop() {
            let tile = grid.get(&idx);

            // Exit found
            if tile.is_exit() {
                if level == 0 {
                    return steps - 1;
                }
                continue;
            }

            // Entrance is always a wall (can be visited when using levels)
            if tile.is_entrance() {
                continue;
            }

            // Don't come back to visited
            if seen.contains(&(idx, level)) {
                continue;
            }
            seen.insert((idx, level));

            // Progress
            match tile {
                Tile::Path => {
                    for nb in idx.von_neumann_neighbors(grid.size) {
                        match grid.get(&nb) {
                            Tile::Path | Tile::Portal(_, _, _) => queue.push((Reverse(steps + 1), Reverse(level), nb)),
                            _ => (),
                        }
                    }
                }
                Tile::Portal(_, _, is_outer) => {
                    if !is_outer || level > 0 {
                        let target = portals[&idx];
                        let Tile::Portal(_, dir, _) = grid.get(&target) else {
                            unreachable!()
                        };
                        let next_level = if *is_outer { level - 1 } else { level + 1 };
                        let queued_level = if use_levels { next_level } else { level };
                        queue.push((Reverse(steps), Reverse(queued_level), target.advance(*dir)))
                    }
                }
                _ => unreachable!(),
            }
        }

        unreachable!()
    }

    fn pair_portals(grid: &Grid<Tile>) -> HashMap<Index, Index> {
        let portal_tiles = grid
            .enumerate()
            .filter_map(|(idx, tile)| tile.portal_name().map(|n| (idx, n)))
            .collect_vec();

        let groups = portal_tiles.into_iter().into_group_map_by(|(_, tile)| *tile);

        groups
            .into_iter()
            .flat_map(|(grp, tiles)| {
                if grp != "AA" && grp != "ZZ" {
                    assert_eq!(tiles.len(), 2);
                    let gate1 = tiles[0].0;
                    let gate2 = tiles[1].0;
                    return vec![(gate1, gate2), (gate2, gate1)];
                }

                vec![]
            })
            .collect()
    }
}

impl Solution for Solution20 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let grid = Self::parse(input);
        Self::find_exit(&grid, false).into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let grid = Self::parse(input);
        Self::find_exit(&grid, true).into_some()
    }
}
