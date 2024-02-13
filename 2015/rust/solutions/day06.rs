use std::collections::HashMap;

use aoc_lib::cartesian::{Direction, Index};
use aoc_lib::iterator::ParsedExt;
use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::{iproduct, Itertools};

type Instruction = (String, Index, Index);

pub struct Solution06;
impl Solution06 {
    fn parse(input: ProblemInput) -> Vec<Instruction> {
        input
            .lines()
            .iter()
            .map(|l| l.trim_start_matches("turn").split_whitespace().collect_vec())
            .map(|tokens| {
                (
                    tokens[0].to_string(),
                    Index::from_tuple(tokens[1].split(',').parsed::<usize>().collect_tuple().unwrap()),
                    Index::from_tuple(tokens[3].split(',').parsed::<usize>().collect_tuple().unwrap()),
                )
            })
            .map(|(a, start, end)| (a, start, end.advance(Direction::South).advance(Direction::East))) // end exclusive
            .collect_vec()
    }

    fn get_relevant_indices(instructions: &[Instruction]) -> (Vec<usize>, Vec<usize>) {
        let (is, js): (Vec<usize>, Vec<usize>) = instructions
            .iter()
            .flat_map(|(_, start, end)| [(start.i, start.j), (end.i, end.j)])
            .unzip();

        (
            is.into_iter().sorted().collect_vec(),
            js.into_iter().sorted().collect_vec(),
        )
    }

    fn get_block_sizes(indices: &[usize]) -> Vec<u128> {
        indices
            .iter()
            .tuple_windows()
            .map(|(c, n)| (n - c) as u128)
            .collect_vec()
    }

    fn apply_instructions<T, F: FnMut(&str, &mut T)>(
        grid: &mut [Vec<T>],
        instructions: &[Instruction],
        ii: &[usize],
        jj: &[usize],
        mut f: F,
    ) {
        // Maps actual grid index to block index
        let ii_map: HashMap<_, _> = ii.iter().enumerate().map(|(ei, i)| (i, ei)).collect();
        let jj_map: HashMap<_, _> = jj.iter().enumerate().map(|(ej, j)| (j, ej)).collect();

        for (action, start, end) in instructions {
            #[allow(clippy::needless_range_loop)]
            for j in jj_map[&start.j]..jj_map[&end.j] {
                for i in ii_map[&start.i]..ii_map[&end.i] {
                    f(action, &mut grid[j][i]);
                }
            }
        }
    }

    fn compute_illumination(flat_grid: &[u128], block_sizes: &[u128]) -> u128 {
        flat_grid
            .iter()
            .zip_eq(block_sizes)
            .map(|(light, scale)| light * scale)
            .sum::<u128>()
    }
}

impl Solution for Solution06 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let instructions = Self::parse(input);
        let (ii, jj) = Self::get_relevant_indices(&instructions);

        let mut grid = vec![vec![false; ii.len() - 1]; jj.len() - 1];
        Self::apply_instructions(&mut grid, &instructions, &ii, &jj, |action, block| match action {
            "on" => *block = true,
            "off" => *block = false,
            "toggle" => *block = !*block,
            _ => unreachable!(),
        });

        let block_sizes = iproduct!(Self::get_block_sizes(&jj), Self::get_block_sizes(&ii))
            .map(|(bj, bi)| bj * bi)
            .collect_vec();
        let flat_grid = grid.into_iter().flatten().map(|l| l as u128).collect_vec();
        Self::compute_illumination(&flat_grid, &block_sizes).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let instructions = Self::parse(input);
        let (ii, jj) = Self::get_relevant_indices(&instructions);

        let mut grid = vec![vec![0u128; ii.len() - 1]; jj.len() - 1];
        Self::apply_instructions(&mut grid, &instructions, &ii, &jj, |action, block| match action {
            "on" => *block += 1,
            "off" => *block = (*block).max(1) - 1,
            "toggle" => *block += 2,
            _ => unreachable!(),
        });

        let block_sizes = iproduct!(Self::get_block_sizes(&jj), Self::get_block_sizes(&ii))
            .map(|(bj, bi)| bj * bi)
            .collect_vec();

        let flat_grid = grid.into_iter().flatten().collect_vec();
        Self::compute_illumination(&flat_grid, &block_sizes).to_result()
    }
}
