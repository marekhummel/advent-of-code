use aoc_lib::cartesian::{Grid, Index, Size};
use aoc_lib::graph::{AStar, Graph};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution15;
impl Solution15 {
    fn scale_cave(grid: &Grid<u8>) -> Grid<u8> {
        // Scale grid to five time its size according to the rules
        let mut scaled = Grid::empty(Size::new(grid.size.width * 5, grid.size.height * 5), 0);
        for (idx, risk) in grid.enumerate() {
            for tile_idx in Size::new(5, 5).indices() {
                let offset = tile_idx.i as u8 + tile_idx.j as u8;
                let scaled_risk = (risk - 1 + offset) % 9 + 1;
                let scaled_idx = Index::new(
                    tile_idx.i * grid.size.width + idx.i,
                    tile_idx.j * grid.size.height + idx.j,
                );
                scaled.set(&scaled_idx, scaled_risk);
            }
        }

        scaled
    }

    fn find_lowest_risk(grid: &Grid<u8>) -> i64 {
        // Convert grid to graph (weight is target risk)
        let mut cave = Graph::empty();
        for idx in grid.size.indices() {
            for nb in idx.von_neumann_neighbors(grid.size) {
                let weight = *grid.get(&nb) as i64;
                cave.add_weighted_edge(&idx, &nb, weight, true);
            }
        }

        // Path finding
        let start = Index::new(0, 0);
        let goal = Index::new(grid.size.width - 1, grid.size.height - 1);
        cave.astar(&start, &goal, |from| from.dist(&goal) as i64).unwrap().0
    }
}

impl Solution for Solution15 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I64(40),
            ProblemResult::I64(696),
            ProblemResult::I64(315),
            ProblemResult::I64(2952),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let cave_grid = input.grid().map_elements(|c| *c as u8 - b'0');
        Self::find_lowest_risk(&cave_grid).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let cave_grid = input.grid().map_elements(|c| *c as u8 - b'0');
        let scaled_grid = Self::scale_cave(&cave_grid);
        Self::find_lowest_risk(&scaled_grid).to_result()
    }
}
