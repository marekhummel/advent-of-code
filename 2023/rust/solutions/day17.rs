use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::solution::Solution;
use aoc_lib::types::{Grid, IntoSome, ProblemInput, ProblemResult};
use aoc_lib::util::{Direction, Index};
use itertools::Itertools;
pub struct Solution17;

impl Solution17 {
    fn get_graph(input: ProblemInput, width: usize, height: usize) -> LatticeGraph {
        let grid = input
            .grid()
            .iter()
            .map(|row| row.iter().parsed().collect_vec())
            .collect_vec();

        LatticeGraph {
            weights: grid,
            width,
            height,
        }
    }
}

impl Solution for Solution17 {
    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult> {
        let (height, width) = input.grid_size();
        let graph = Self::get_graph(input, width, height);
        let start = Index { i: 0, j: 0 };
        let goal = Index {
            i: width - 1,
            j: height - 1,
        };

        graph.shortest_path(start, goal, 0, 3).unwrap().heat.into_some()
    }

    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult> {
        let (height, width) = input.grid_size();
        let graph = Self::get_graph(input, width, height);
        let start = Index { i: 0, j: 0 };
        let goal = Index {
            i: width - 1,
            j: height - 1,
        };

        graph.shortest_path(start, goal, 4, 10).unwrap().heat.into_some()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Vertex {
    pos: Index,
    straight: Option<(Direction, usize)>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct QueueElement {
    heat: u32,
    vertex: Vertex,
}

// Explicit implementation for Ord and PartialOrd to get Min-Heap
impl Ord for QueueElement {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .heat
            .cmp(&self.heat)
            .then_with(|| other.vertex.pos.cmp(&self.vertex.pos))
    }
}

impl PartialOrd for QueueElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct LatticeGraph {
    weights: Grid<u32>,
    width: usize,
    height: usize,
}

impl LatticeGraph {
    fn shortest_path(
        &self,
        start: Index,
        goal: Index,
        min_straight: usize,
        max_straight: usize,
    ) -> Option<QueueElement> {
        let mut visited: HashSet<Vertex> = HashSet::new();
        let mut queue: BinaryHeap<QueueElement> = BinaryHeap::from([QueueElement {
            heat: 0,
            vertex: Vertex {
                pos: start,
                straight: None,
            },
        }]);

        while let Some(state) = queue.pop() {
            let u = state.vertex.pos;
            if u == goal {
                return Some(state);
            }
            if visited.contains(&state.vertex) {
                continue;
            }
            visited.insert(state.vertex.clone());

            for (v, weight) in self.get_neighbors(&state.vertex, min_straight, max_straight) {
                let new_heat = state.heat + weight;
                let dir = u.get_direction_to(v);

                let last_str = state.vertex.straight.unwrap_or((Direction::None, 0));
                let straight = if last_str.0 == dir { last_str.1 + 1 } else { 0 };
                queue.push(QueueElement {
                    heat: new_heat,
                    vertex: Vertex {
                        pos: v,
                        straight: Some((dir, straight)),
                    },
                })
            }
        }

        None
    }

    fn get_neighbors(&self, vertex: &Vertex, min_straight: usize, max_straight: usize) -> Vec<(Index, u32)> {
        let valid_directions = match vertex.straight {
            Some((dir, count)) => {
                let mut directions = Vec::new();
                if count < max_straight - 1 {
                    directions.push(dir)
                }
                if min_straight == 0 || min_straight - 1 <= count {
                    directions.extend(dir.turn())
                }
                directions
            }
            None => Direction::compass().to_vec(),
        };

        valid_directions
            .iter()
            .filter_map(|d| vertex.pos.advance_check(*d, self.width, self.height))
            .map(|v| (v, *v.grid_get(&self.weights)))
            .collect_vec()
    }
}
