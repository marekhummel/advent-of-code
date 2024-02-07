use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::{iproduct, Itertools};

#[derive(Debug, Clone)]
struct Pos4(i32, i32, i32, i32);

impl Pos4 {
    fn dist(&self, other: &Pos4) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1) + self.2.abs_diff(other.2) + self.3.abs_diff(other.3)
    }
}

pub struct Solution25;
impl Solution25 {
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
}

impl Solution for Solution25 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let points = Self::parse(input);

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

        constellations.len().into_some()
    }

    fn solve_version02(&self, _input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        None
    }
}
