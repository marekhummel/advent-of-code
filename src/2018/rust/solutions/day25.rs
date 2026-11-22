use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::{Itertools, iproduct};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let points = parse(input);

    // Union Find struct
    let mut constellations = points.iter().map(|pt| vec![pt.clone()]).collect_vec();

    loop {
        let mut new_constellations: Vec<Vec<Pos4>> = Vec::new();

        let mut update = false;
        for cons in &constellations {
            let mut added = false;

            for new_cons in new_constellations.iter_mut() {
                if iproduct!(cons.iter(), new_cons.iter()).any(|(p1, p2)| p1.dist(p2) <= 3) {
                    new_cons.extend(cons.iter().cloned());
                    update = true;
                    added = true;
                    break;
                }
            }

            if !added {
                new_constellations.push(cons.clone());
            }
        }

        if !update {
            break;
        }

        constellations = new_constellations;
    }

    constellations.len().into()
}

pub fn part2(_input: ProblemInput, _is_sample: bool) -> ProblemResult {
    ProblemResult::NoPartTwo
}

#[derive(Debug, Clone)]
struct Pos4(i32, i32, i32, i32);

impl Pos4 {
    fn dist(&self, other: &Pos4) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1) + self.2.abs_diff(other.2) + self.3.abs_diff(other.3)
    }
}

fn parse(input: ProblemInput) -> Vec<Pos4> {
    input
        .lines()
        .into_iter()
        .map(|l| {
            let (x, y, z, w) = l.split(',').collect_tuple().unwrap();
            Pos4(
                x.trim().parse().unwrap(),
                y.trim().parse().unwrap(),
                z.trim().parse().unwrap(),
                w.trim().parse().unwrap(),
            )
        })
        .collect()
}
