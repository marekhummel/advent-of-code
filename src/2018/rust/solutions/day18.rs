use aoc_lib::algo;
use aoc_lib::cartesian::Grid;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Acre {
    Open,
    Tree,
    Lumberyard,
}

pub struct Solution18;
impl Solution18 {
    fn parse(input: ProblemInput) -> Grid<Acre> {
        input.grid().map_elements(|c| match c {
            '.' => Acre::Open,
            '|' => Acre::Tree,
            '#' => Acre::Lumberyard,
            _ => unreachable!(),
        })
    }

    fn magic(state: Grid<Acre>) -> Grid<Acre> {
        let mut new_state = Grid::empty(state.size, Acre::Open);

        for (idx, acre) in state.enumerate() {
            let nbs = idx.moore_neighbors(state.size);
            let nb_acres = nbs.into_iter().map(|idx| state.get(&idx)).counts();
            let adj_trees = *nb_acres.get(&Acre::Tree).unwrap_or(&0);
            let adj_lumber = *nb_acres.get(&Acre::Lumberyard).unwrap_or(&0);

            let new_acre = match acre {
                Acre::Open if adj_trees >= 3 => Acre::Tree,
                Acre::Open => Acre::Open,
                Acre::Tree if adj_lumber >= 3 => Acre::Lumberyard,
                Acre::Tree => Acre::Tree,
                Acre::Lumberyard if adj_lumber >= 1 && adj_trees >= 1 => Acre::Lumberyard,
                Acre::Lumberyard => Acre::Open,
            };

            new_state.set(&idx, new_acre);
        }

        new_state
    }
}

impl Solution for Solution18 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(1147),
            ProblemResult::USize(606416),
            ProblemResult::USize(0),
            ProblemResult::USize(210796),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut state = Self::parse(input);

        for _ in 0..10 {
            state = Self::magic(state);
        }

        let counts = state.iter().counts();
        (counts[&Acre::Tree] * counts[&Acre::Lumberyard]).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let state = Self::parse(input);

        let final_state = algo::find_final_state(state, Self::magic, 1_000_000_000);
        let counts = final_state.iter().counts();

        (counts.get(&Acre::Tree).unwrap_or(&0) * counts.get(&Acre::Lumberyard).unwrap_or(&0)).to_result()
    }
}
