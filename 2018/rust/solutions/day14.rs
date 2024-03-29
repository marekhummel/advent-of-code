use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution14;
impl Solution14 {
    fn cook(recipes: &mut Vec<u8>, i: &mut usize, j: &mut usize) {
        let mut total = recipes[*i] + recipes[*j];
        if total >= 10 {
            recipes.push(total / 10);
            total %= 10;
        }
        recipes.push(total);

        *i = (*i + 1 + recipes[*i] as usize) % recipes.len();
        *j = (*j + 1 + recipes[*j] as usize) % recipes.len();
    }
}

impl Solution for Solution14 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U64(5941429882),
            ProblemResult::U64(5371393113),
            ProblemResult::USize(2018),
            ProblemResult::USize(20286858),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let counter: usize = input.string().parse().unwrap();

        let mut recipes = vec![3, 7];
        let mut i = 0;
        let mut j = 1;

        for _ in 0.. {
            Self::cook(&mut recipes, &mut i, &mut j);
            if recipes.len() >= counter + 10 {
                break;
            }
        }

        let digits = &recipes[counter..counter + 10];
        let value = digits.iter().fold(0u64, |val, d| val * 10 + *d as u64);
        value.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let subset: Vec<u8> = input.string().chars().parsed().collect();

        let mut recipes = vec![3, 7];
        let mut i = 0;
        let mut j = 1;

        let mut solution = 0;

        for _ in 0.. {
            Self::cook(&mut recipes, &mut i, &mut j);

            for si in solution..recipes.len().saturating_sub(subset.len()) {
                let mut found = true;
                for k in 0..subset.len() {
                    if recipes[si + k] != subset[k] {
                        found = false;
                    }
                }

                if found {
                    return si.to_result();
                }
            }

            solution = recipes.len() - subset.len();
        }

        unreachable!()
    }
}
