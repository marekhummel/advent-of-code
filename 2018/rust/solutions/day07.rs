use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

use aoc_lib::graph::{self, Graph};
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

pub struct Solution07;
impl Solution07 {
    fn parse(input: ProblemInput) -> Graph<char> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let words = l.split_whitespace().collect_vec();
                (words[1].chars().next().unwrap(), words[7].chars().next().unwrap())
            })
            .into_group_map_by(|(src, _)| *src)
            .into_iter()
            .map(|(src, edges)| (src, edges.into_iter().map(|(_, trg)| trg).collect()))
            .collect()
    }
}

impl Solution for Solution07 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let graph = Self::parse(input);
        let sorting = graph::topo_sorting(&graph).unwrap();

        sorting.into_iter().join("").into_some()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        let graph = Self::parse(input);
        let num_workers = if is_sample { 2 } else { 5 };
        let workload = |task: &char| -> u8 { (*task as u8 - b'A') + 1 + if is_sample { 0 } else { 60 } };

        // Invert graph to map trg to source nodes
        let mut sources = graph::invert(&graph);
        let mut ready: BinaryHeap<_> = sources
            .iter()
            .filter(|(_, srcs)| srcs.is_empty())
            .map(|(trg, _)| Reverse(*trg)) // Use reverse to ensure smaller nodes are chosen first
            .collect();

        // Start with n idle workers
        let mut workers: Vec<Option<(Reverse<char>, u8)>> = vec![None; num_workers];
        let mut checked_tasks: HashSet<_> = ready.iter().map(|r| r.0).collect();

        // Simulate
        let mut time = 0;
        while !sources.is_empty() {
            // Update busy workers
            for worker in workers.iter_mut() {
                if let Some((Reverse(job), remaining)) = worker {
                    if *remaining > 1 {
                        *remaining -= 1;
                        continue;
                    }

                    // Remove completed task from graph (both as vertex and edges) and add vertices with no incoming edges
                    sources.remove(job);
                    for (trg, srcs) in sources.iter_mut() {
                        srcs.remove(job);
                        if srcs.is_empty() && !checked_tasks.contains(trg) {
                            ready.push(Reverse(*trg));
                            checked_tasks.insert(*trg);
                        }
                    }
                    *worker = None;
                }
            }

            // Start new workers
            for worker in workers.iter_mut() {
                if worker.is_none() {
                    if let Some(Reverse(job)) = ready.pop() {
                        *worker = Some((Reverse(job), workload(&job)));
                    }
                }
            }

            time += 1;
        }

        (time - 1).into_some()
    }
}
