use std::collections::{HashMap, HashSet, VecDeque};

use aoc_lib::cartesian::{Grid, Index};
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let map = input.grid();

    let graph = create_graph(&map);
    let shortest_route = perm_intermediate_stops(&map)
        .map(|perm| graph[&'0'][perm[0]] + perm.iter().tuple_windows().map(|(u, v)| graph[u][v]).sum::<usize>())
        .min()
        .unwrap();

    shortest_route.into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let map = input.grid();

    let graph = create_graph(&map);
    let shortest_route = perm_intermediate_stops(&map)
        .map(|perm| {
            graph[&'0'][perm[0]]
                + perm.iter().tuple_windows().map(|(u, v)| graph[u][v]).sum::<usize>()
                + graph[perm.last().unwrap()][&'0']
        })
        .min()
        .unwrap();

    shortest_route.into()
}

// BFS to find shortest path to other stops given a start.
fn shortest_paths(map: &Grid<char>, start: Index) -> HashMap<char, usize> {
    let mut seen = HashSet::from([(start)]);
    let mut queue = VecDeque::from([(start, 0)]);
    let mut found = HashMap::new();

    while let Some((pos, steps)) = queue.pop_front() {
        for next_pos in pos.von_neumann_neighbors(map.size) {
            let tile = *map.get(&next_pos);
            if tile != '#' && !seen.contains(&next_pos) {
                seen.insert(next_pos);
                queue.push_back((next_pos, steps + 1));

                if tile != '.' {
                    found.insert(tile, steps + 1);
                }
            }
        }
    }

    found
}

// Filter out stops in maze
fn determine_targets(map: &Grid<char>) -> Vec<(Index, &char)> {
    map.enumerate().filter(|(_, c)| **c != '#' && **c != '.').collect()
}

// Compose a graph from the maze and its targets
fn create_graph(map: &Grid<char>) -> HashMap<char, HashMap<char, usize>> {
    determine_targets(map)
        .iter()
        .map(|(idx, c)| (**c, shortest_paths(map, *idx)))
        .collect()
}

// Since the start (and finish) are fixed, permute all stops in between
fn perm_intermediate_stops(map: &Grid<char>) -> impl Iterator<Item = Vec<&char>> {
    let targets = determine_targets(map);
    let n = targets.len();
    targets
        .into_iter()
        .map(|(_, c)| c)
        .filter(|c| **c != '0')
        .permutations(n - 1)
}
