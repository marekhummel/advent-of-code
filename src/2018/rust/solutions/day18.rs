use aoc_lib::algo;
use aoc_lib::cartesian::Grid;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut state = parse(input);

    for _ in 0..10 {
        state = magic(state);
    }

    let counts = state.iter().counts();
    (counts[&Acre::Tree] * counts[&Acre::Lumberyard]).into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let state = parse(input);

    let final_state = algo::find_final_state(state, magic, 1_000_000_000);
    let counts = final_state.iter().counts();

    (counts.get(&Acre::Tree).unwrap_or(&0) * counts.get(&Acre::Lumberyard).unwrap_or(&0)).into()
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Acre {
    Open,
    Tree,
    Lumberyard,
}

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
