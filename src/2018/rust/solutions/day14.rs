use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let counter: usize = input.string().parse().unwrap();

    let mut recipes = vec![3, 7];
    let mut i = 0;
    let mut j = 1;

    for _ in 0.. {
        cook(&mut recipes, &mut i, &mut j);
        if recipes.len() >= counter + 10 {
            break;
        }
    }

    let digits = &recipes[counter..counter + 10];
    let value = digits.iter().fold(0u64, |val, d| val * 10 + *d as u64);
    value.into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let subset: Vec<u8> = input.string().chars().parsed().collect();

    let mut recipes = vec![3, 7];
    let mut i = 0;
    let mut j = 1;

    let mut solution = 0;

    for _ in 0.. {
        cook(&mut recipes, &mut i, &mut j);

        for si in solution..recipes.len().saturating_sub(subset.len()) {
            let mut found = true;
            for k in 0..subset.len() {
                if recipes[si + k] != subset[k] {
                    found = false;
                }
            }

            if found {
                return si.into();
            }
        }

        solution = recipes.len() - subset.len();
    }

    unreachable!()
}

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
