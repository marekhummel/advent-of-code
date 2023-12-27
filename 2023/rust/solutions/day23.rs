use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use aoc_lib::solution::Solution;
use aoc_lib::types::{Grid, IntoSome, ProblemInput, ProblemResult};
use aoc_lib::util::{Direction, Index, Size};
use itertools::Itertools;

pub struct Solution23;
impl Solution23 {
    fn parse(input: ProblemInput) -> (Grid<char>, Index, Index) {
        let hiking_map = input.grid();
        let start = Index {
            i: hiking_map[0].iter().position(|c| *c == '.').unwrap(),
            j: 0,
        };
        let end = Index {
            i: hiking_map[hiking_map.len() - 1].iter().position(|c| *c == '.').unwrap(),
            j: hiking_map.len() - 1,
        };

        (hiking_map, start, end)
    }

    fn create_trail_graph(grid: &Grid<char>, slippery_slopes: bool) -> HashMap<Index, Vec<(Index, usize)>> {
        let map_size = Size::from_grid(grid);

        // Use refcells here for mutability problems in collapse part.
        let mut junctions: HashMap<Index, RefCell<Vec<(Index, usize)>>> = HashMap::new();
        for j in 0..map_size.height {
            for i in 0..map_size.width {
                let idx = Index { i, j };
                if *idx.grid_get(grid) != '#' {
                    junctions.insert(
                        idx,
                        RefCell::new(
                            Self::get_neighbors(grid, idx, slippery_slopes)
                                .into_iter()
                                .map(|nb| (nb, nb.dist(&idx)))
                                .collect_vec(),
                        ),
                    );
                }
            }
        }

        // Collapse graph for second part to speed up dfs
        // Won't work for part 1 due to the slopes being one-way roads.
        if !slippery_slopes {
            for (&idx, junction) in &junctions {
                if let [(left, ld), (right, rd)] = junction.borrow()[..] {
                    let mut left_nb = junctions[&left].borrow_mut();
                    left_nb.retain(|&(n, _)| n != idx);
                    left_nb.push((right, ld + rd));

                    let mut right_nb = junctions[&right].borrow_mut();
                    right_nb.retain(|&(n, _)| n != idx);
                    right_nb.push((left, ld + rd));
                }
            }
            junctions.retain(|_, j| j.borrow().len() != 2);
        }

        // Discard ref cells
        junctions
            .into_iter()
            .map(|(idx, nbs)| (idx, nbs.borrow().clone()))
            .collect()
    }

    fn get_neighbors(grid: &Grid<char>, idx: Index, slippery_slopes: bool) -> Vec<Index> {
        let map_size = Size::from_grid(grid);
        let directions = match slippery_slopes {
            true => match idx.grid_get(grid) {
                '>' => vec![Direction::East],
                '<' => vec![Direction::West],
                '^' => vec![Direction::North],
                'v' => vec![Direction::South],
                '.' => Direction::compass().to_vec(),
                _ => unreachable!(),
            },
            false => Direction::compass().to_vec(),
        };

        directions
            .into_iter()
            .filter_map(|d| idx.advance_check(d, map_size))
            .filter(|np| *np.grid_get(grid) != '#')
            .collect()
    }

    fn longest_path(
        junctions: &HashMap<Index, Vec<(Index, usize)>>,
        visited: &mut HashSet<Index>,
        pos: Index,
        end: Index,
    ) -> Option<usize> {
        if pos == end {
            return Some(0);
        }

        let mut best: Option<usize> = None;
        for (new_pos, dist) in junctions[&pos].iter() {
            if !visited.contains(new_pos) {
                visited.insert(*new_pos);

                if let Some(t) = Self::longest_path(junctions, visited, *new_pos, end) {
                    best = match best {
                        Some(b) => Some(b.max(t + dist)),
                        None => Some(t + dist),
                    }
                };

                visited.remove(new_pos);
            }
        }

        best
    }
}

impl Solution for Solution23 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let (map, start, end) = Self::parse(input);
        let trail_graph = Self::create_trail_graph(&map, true);
        let mut visited = HashSet::from([start]);
        Self::longest_path(&trail_graph, &mut visited, start, end)
            .unwrap()
            .into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let (map, start, end) = Self::parse(input);
        let trail_graph = Self::create_trail_graph(&map, false);
        let mut visited = HashSet::from([start]);
        Self::longest_path(&trail_graph, &mut visited, start, end)
            .unwrap()
            .into_some()
    }
}
