use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::solution::Solution;
use aoc_lib::types::{Grid, IntoSome, ProblemInput, ProblemResult};
use aoc_lib::util::{Direction, Position};
use itertools::{iproduct, Itertools};
pub struct Solution17;

impl Solution17 {
    fn get_graph(input: ProblemInput, width: usize, height: usize) -> LatticeGraph {
        let grid = input
            .grid()
            .iter()
            .map(|row| row.iter().parsed().collect_vec())
            .collect_vec();

        let vertices = iproduct!(0..width, 0..height)
            .map(|(x, y)| Position { x, y })
            .collect_vec();

        LatticeGraph {
            weights: grid.to_vec(),
            width,
            height,
        }
    }
}

impl Solution for Solution17 {
    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult> {
        let (height, width) = input.grid_size();
        let graph = Self::get_graph(input, width, height);
        let start = Position { x: 0, y: 0 };
        let goal = Position {
            x: width - 1,
            y: height - 1,
        };

        graph.shortest_path(start, goal, 0, 3).unwrap().heat.into_some()
    }

    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult> {
        let (height, width) = input.grid_size();
        let graph = Self::get_graph(input, width, height);
        let start = Position { x: 0, y: 0 };
        let goal = Position {
            x: width - 1,
            y: height - 1,
        };

        graph.shortest_path(start, goal, 4, 10).unwrap().heat.into_some()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    heat: u32,
    vertex: Position,
    straight: Option<(Direction, usize)>,
}

// Explicit implementation for Ord and PartialOrd to get Min-Heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .heat
            .cmp(&self.heat)
            .then_with(|| other.vertex.y.cmp(&self.vertex.y))
            .then_with(|| other.vertex.x.cmp(&self.vertex.x))
    }
}

impl PartialOrd for State {
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
        start: Position,
        goal: Position,
        min_straight: usize,
        max_straight: usize,
    ) -> Option<State> {
        let mut visited: HashSet<(Position, (Direction, usize))> = HashSet::new();
        let mut queue: BinaryHeap<State> = BinaryHeap::from([State {
            heat: 0,
            vertex: start,
            straight: None,
        }]);

        while let Some(state) = queue.pop() {
            let u = state.vertex;
            if u == goal {
                return Some(state);
            }
            if let Some(straight) = state.straight {
                if visited.contains(&(state.vertex, straight)) {
                    continue;
                }
            }
            visited.insert((state.vertex, state.straight.unwrap_or((Direction::None, 0))));

            for (v, weight) in self.get_neighbors(&state, min_straight, max_straight) {
                let new_heat = state.heat + weight;
                let dir = u.get_direction_to(v);

                let last_str = state.straight.unwrap_or((Direction::None, 0));
                let straight = if last_str.0 == dir { last_str.1 + 1 } else { 0 };
                queue.push(State {
                    heat: new_heat,
                    vertex: v,
                    straight: Some((dir, straight)),
                })
            }
        }

        None
    }

    fn get_neighbors(&self, state: &State, min_straight: usize, max_straight: usize) -> Vec<(Position, u32)> {
        let valid_directions = match state.straight {
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
            .filter_map(|d| state.vertex.advance_check(*d, self.width, self.height))
            .map(|v| (v, *v.grid_get(&self.weights)))
            .collect_vec()
    }
}
