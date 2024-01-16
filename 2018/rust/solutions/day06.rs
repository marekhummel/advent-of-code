use aoc_lib::cartesian::{Index, Size};
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};

pub struct Solution06;
impl Solution06 {
    fn parse(input: ProblemInput) -> (Vec<Index>, Size) {
        let indices = input
            .lines()
            .into_iter()
            .map(|l| {
                let (x, y) = l.split_once(',').unwrap();
                Index {
                    i: x.trim().parse().unwrap(),
                    j: y.trim().parse().unwrap(),
                }
            })
            .collect_vec();

        let Some((min_i, max_i)) = indices.iter().minmax_by_key(|idx| idx.i).into_option() else {
            panic!()
        };
        let Some((min_j, max_j)) = indices.iter().minmax_by_key(|idx| idx.j).into_option() else {
            panic!()
        };

        let size = Size {
            // Add padding of 1 around
            width: max_i.i - min_i.i + 3,
            height: max_j.j - min_j.j + 3,
        };

        let normalized_indices = indices
            .iter()
            .map(|idx| Index {
                i: idx.i - min_i.i + 1,
                j: idx.j - min_j.j + 1,
            })
            .collect();

        (normalized_indices, size)
    }
}

impl Solution for Solution06 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let (locations, size) = Self::parse(input);

        let mut infinite = HashSet::new();
        let mut areas = HashMap::new();

        for (i, j) in iproduct!(0..size.width, 0..size.height) {
            let idx = Index { i, j };
            let closest = locations.iter().enumerate().min_set_by_key(|(_, loc)| loc.dist(&idx));

            if closest.len() == 1 {
                let (id, _) = closest[0];
                *areas.entry(id).or_insert(0) += 1;

                if idx.i == 0 || idx.j == 0 || idx.i == size.width - 1 || idx.j == size.height + 1 {
                    infinite.insert(id);
                }
            }
        }

        areas
            .into_iter()
            .filter(|(id, _)| !infinite.contains(id))
            .map(|(_, area)| area)
            .max()
            .unwrap()
            .into_some()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        let (locations, size) = Self::parse(input);
        let range = if is_sample { 32 } else { 10000 };

        iproduct!(0..size.width, 0..size.height)
            .map(|(i, j)| Index { i, j })
            .map(|idx| locations.iter().map(|loc| loc.dist(&idx)).sum::<usize>())
            .filter(|dist| *dist < range)
            .count()
            .into_some()
    }
}
