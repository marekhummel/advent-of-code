use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use regex::Regex;

type Ingredient = (i32, i32, i32, i32, i32);
type Cookie = (i32, i32, i32, i32, i32);

pub struct Solution15;
impl Solution15 {
    fn parse(input: ProblemInput) -> Vec<Ingredient> {
        let line_rgx = Regex::new(r"(?P<ingedient>\w*?): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)").unwrap();

        input
            .lines()
            .into_iter()
            .map(|l| {
                let captures = line_rgx.captures(&l).unwrap();
                let capacity = captures.name("capacity").unwrap().as_str().parse::<i32>().unwrap();
                let durability = captures.name("durability").unwrap().as_str().parse::<i32>().unwrap();
                let flavor = captures.name("flavor").unwrap().as_str().parse::<i32>().unwrap();
                let texture = captures.name("texture").unwrap().as_str().parse::<i32>().unwrap();
                let calories = captures.name("calories").unwrap().as_str().parse::<i32>().unwrap();

                (capacity, durability, flavor, texture, calories)
            })
            .collect()
    }

    fn score(ingredients: &[Ingredient], cookie: Cookie, teaspoons: i32, mind_calories: bool) -> i32 {
        if ingredients.is_empty() {
            let (c, d, f, t, cals) = cookie;
            if teaspoons == 0 && (!mind_calories || cals == 500) && c >= 0 && d >= 0 && f >= 0 && t >= 0 {
                return c * d * f * t;
            } else {
                return 0;
            }
        }

        let mut best = 0;
        let range = if ingredients.len() == 1 {
            vec![teaspoons]
        } else {
            (0..teaspoons).collect_vec()
        };
        for n in range {
            let (cac, dc, fc, tc, clc) = cookie;
            let (cai, di, fi, ti, cli) = ingredients[0];
            let new_cookie = (cac + cai * n, dc + di * n, fc + fi * n, tc + ti * n, clc + cli * n);
            best = best.max(Self::score(&ingredients[1..], new_cookie, teaspoons - n, mind_calories));
        }

        best
    }
}

impl Solution for Solution15 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let ingredients = Self::parse(input);
        Self::score(&ingredients, (0, 0, 0, 0, 0), 100, false).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let ingredients = Self::parse(input);
        Self::score(&ingredients, (0, 0, 0, 0, 0), 100, true).to_result()
    }
}
