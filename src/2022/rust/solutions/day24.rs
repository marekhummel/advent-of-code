use aoc_lib::cartesian::{Direction, Grid, Index};
use aoc_lib::graph::{DynamicGraph, PathFinding};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

#[derive(Debug, Clone, PartialEq, Eq)]
enum ValleyTile {
    Ground,
    Wall,
    Blizzards(Vec<Direction>),
}

pub struct Solution24;
impl Solution24 {
    fn parse(input: ProblemInput) -> (Grid<ValleyTile>, Index, Index) {
        let valley = input.grid().map_elements(|&c| match c {
            '.' => ValleyTile::Ground,
            '#' => ValleyTile::Wall,
            _ => ValleyTile::Blizzards(vec![c.try_into().unwrap()]),
        });

        let last_row = valley.size.height - 1;
        let start_i = valley.rows[0].iter().position(|c| *c == ValleyTile::Ground).unwrap();
        let goal_i = valley.rows[last_row]
            .iter()
            .position(|c| *c == ValleyTile::Ground)
            .unwrap();

        (valley, Index::new(start_i, 0), Index::new(goal_i, last_row))
    }

    fn create_graph(valley: Grid<ValleyTile>) -> DynamicGraph<(Index, usize)> {
        let mut valley_history = vec![valley];

        DynamicGraph {
            adjacent: Box::new(move |(index, time)| {
                // Update blizzards
                while valley_history.len() <= *time + 1 {
                    valley_history.push(Self::move_blizzards(valley_history.last().unwrap()));
                }
                let valley = &valley_history[time + 1];

                // Find possible next moves
                let mut children = Vec::new();
                let mut possible = index.von_neumann_neighbors(valley.size);
                possible.push(*index);
                for nb in possible {
                    if matches!(valley.get(&nb), ValleyTile::Ground) {
                        children.push(((nb, *time + 1), 1));
                    }
                }
                children
            }),
        }
    }

    fn move_blizzards(current: &Grid<ValleyTile>) -> Grid<ValleyTile> {
        let mut next_map = Grid::empty(current.size, ValleyTile::Ground);

        for (idx, tile) in current.enumerate() {
            match tile {
                ValleyTile::Ground => (),
                ValleyTile::Wall => next_map.set(&idx, tile.clone()),
                ValleyTile::Blizzards(dirs) => {
                    for &dir in dirs {
                        let mut next = idx.advance(dir);
                        if matches!(current.get(&next), ValleyTile::Wall) {
                            next = next.advance_wrap(dir, current.size).advance(dir);
                        }
                        let new_tile = match next_map.get(&next) {
                            ValleyTile::Ground => ValleyTile::Blizzards(vec![dir]),
                            ValleyTile::Blizzards(dirs) => ValleyTile::Blizzards([dirs.clone(), vec![dir]].concat()),
                            ValleyTile::Wall => unreachable!(),
                        };
                        next_map.set(&next, new_tile);
                    }
                }
            }
        }

        next_map
    }
}

impl Solution for Solution24 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I64(18),
            ProblemResult::I64(255),
            ProblemResult::USize(54),
            ProblemResult::USize(809),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (valley, start, goal) = Self::parse(input);

        let mut graph = Self::create_graph(valley);
        let fastest = graph.astar_many(&(start, 0), |(i, _)| *i == goal, |(index, _)| index.dist(&goal) as i64);
        fastest.unwrap().0.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (valley, start, goal) = Self::parse(input);

        let mut graph = Self::create_graph(valley);
        let mut total_time = 0;

        let trip1 = graph.astar_many(&(start, 0), |(i, _)| *i == goal, |(i, _)| goal.dist(i) as i64);
        total_time += trip1.unwrap().0 as usize;

        let trip2 = graph.astar_many(&(goal, total_time), |(i, _)| *i == start, |(i, _)| start.dist(i) as i64);
        total_time += trip2.unwrap().0 as usize;

        let trip3 = graph.astar_many(&(start, total_time), |(i, _)| *i == goal, |(i, _)| goal.dist(i) as i64);
        total_time += trip3.unwrap().0 as usize;

        total_time.to_result()
    }
}
