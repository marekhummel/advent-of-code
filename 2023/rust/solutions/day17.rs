use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;

use aoc_lib::cartesian::{Direction, Grid, Index, Size};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
pub struct Solution17;

impl Solution17 {
    fn get_graph(input: ProblemInput) -> LatticeGraph {
        let grid = input
            .grid()
            .map_elements(|elem| elem.to_string().parse::<u32>().unwrap());

        LatticeGraph {
            size: grid.size,
            weights: grid,
        }
    }
}

impl Solution for Solution17 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let graph = Self::get_graph(input);
        let start = Index::new(0, 0);
        let goal = Index::new(graph.size.width - 1, graph.size.height - 1);

        graph.shortest_path(start, goal, 0, 3).unwrap().heat.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let graph = Self::get_graph(input);
        let start = Index::new(0, 0);
        let goal = Index::new(graph.size.width - 1, graph.size.height - 1);

        graph.shortest_path(start, goal, 4, 10).unwrap().heat.to_result()
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
    size: Size,
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
            .filter_map(|d| vertex.pos.advance_check(*d, self.size))
            .map(|v| (v, *self.weights.get(&v)))
            .collect_vec()
    }
}
