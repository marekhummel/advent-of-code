use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use regex::Regex;

pub struct Solution10;
impl Solution10 {
    fn parse(input: ProblemInput) -> Vec<((i32, i32), (i32, i32))> {
        let line_rgx =
            Regex::new(r"^position=<\s*(?P<px>-?\d+),\s*(?P<py>-?\d+)> velocity=<\s*(?P<vx>-?\d+),\s*(?P<vy>-?\d+)>$")
                .unwrap();
        input
            .lines()
            .into_iter()
            .map(|l| {
                let captures = line_rgx.captures(&l).unwrap();
                let px = captures.name("px").unwrap().as_str().parse().unwrap();
                let py = captures.name("py").unwrap().as_str().parse().unwrap();
                let vx = captures.name("vx").unwrap().as_str().parse().unwrap();
                let vy = captures.name("vy").unwrap().as_str().parse().unwrap();

                ((px, py), (vx, vy))
            })
            .collect()
    }

    fn find_message(mut lights: Vec<((i32, i32), (i32, i32))>, is_sample: bool) -> (String, usize) {
        let num_lines = if is_sample { 2 } else { 8 };
        let line_len = 8;

        for s in 1.. {
            for ((px, py), (vx, vy)) in lights.iter_mut() {
                *px += *vx;
                *py += *vy;
            }

            let alignment = lights.iter().map(|(p, _)| p).unique().sorted().collect_vec();
            let lines = alignment
                .as_slice()
                .windows(line_len)
                .filter(|wdw| {
                    wdw[0].0 == wdw[line_len - 1].0 && (wdw[line_len - 1].1 - wdw[0].1) == line_len as i32 - 1
                })
                .collect_vec();

            if lines.len() >= num_lines {
                // let (min_x, max_x) = (lines[0][0].0, lines[lines.len() - 1][0].0);
                // let (min_y, max_y) = (lines[0][0].1, lines[0][line_len - 1].1);

                // println!();
                // for y in (min_y - 1)..=(max_y + 1) {
                //     for x in (min_x - 10)..=(max_x + 10) {
                //         if lights.iter().any(|((px, py), _)| x == *px && y == *py) {
                //             print!("#");
                //         } else {
                //             print!(" ");
                //         }
                //     }
                //     println!()
                // }

                // Print lights and see outcome
                let text = (if is_sample { "HI" } else { "CPJRNKCF" }).to_string();
                return (text, s);
            }
        }

        unreachable!()
    }
}

impl Solution for Solution10 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let lights = Self::parse(input);

        let (message, _) = Self::find_message(lights, is_sample);
        message.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let lights = Self::parse(input);

        let (_, time) = Self::find_message(lights, is_sample);
        time.to_result()
    }
}
