use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

use aoc_lib::cartesian::{Direction, Grid};
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
    Entrance,
    SubEntrance(usize),
    Passage,
    Wall,
    Key(char),
    Door(char),
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Entrance => write!(f, "@"),
            Self::SubEntrance(i) => write!(f, "{}", i),
            Self::Passage => write!(f, "."),
            Self::Wall => write!(f, "#"),
            Self::Key(k) => write!(f, "{}", k),
            Self::Door(k) => write!(f, "{}", k.to_ascii_uppercase()),
        }
    }
}

impl Tile {
    fn from_char(c: &char) -> Self {
        match c {
            '@' => Tile::Entrance,
            '#' => Tile::Wall,
            '.' => Tile::Passage,
            _ if c.is_ascii_lowercase() => Tile::Key(*c),
            _ => Tile::Door(c.to_ascii_lowercase()),
        }
    }

    fn is_keypoint(&self) -> bool {
        match self {
            Tile::Entrance => true,
            Tile::SubEntrance(_) => true,
            Tile::Key(_) => true,
            Tile::Passage => false,
            Tile::Wall => false,
            Tile::Door(_) => false,
        }
    }
}

#[derive(Debug)]
struct Path {
    length: u32,
    keys: Vec<char>,
    doors: Vec<char>,
}

impl Path {
    fn new(length: u32, keys: &[char], doors: &[char]) -> Path {
        Path {
            length,
            keys: keys.to_vec(),
            doors: doors.to_vec(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CollectedKeys {
    keys: u32,
}

impl CollectedKeys {
    fn num_collected(&self) -> usize {
        self.keys.count_ones() as usize
    }

    fn in_possession(&self, key: char) -> bool {
        let bit = 1 << (key as u8 - b'a');
        self.keys & bit != 0
    }

    fn collect(&mut self, key: char) {
        self.keys |= 1 << (key as u8 - b'a')
    }
}

pub struct Solution18;
impl Solution18 {
    // Compose a graph from the maze and its targets
    fn create_graph(map: &Grid<Tile>) -> HashMap<Tile, HashMap<Tile, Path>> {
        map.enumerate()
            .filter(|(_, c)| c.is_keypoint())
            .map(|(start, c)| {
                // BFS to find shortest path to other stops given a start.
                let size = map.size();
                let mut seen = HashSet::from([(start)]);
                let mut queue = VecDeque::from([(start, 0, vec![], vec![])]);
                let mut found = HashMap::new();

                while let Some((pos, steps, keys, doors)) = queue.pop_front() {
                    for next_pos in pos.von_neumann_neighbors(size) {
                        let tile = *map.get(&next_pos);
                        if !matches!(tile, Tile::Wall) && !seen.contains(&next_pos) {
                            seen.insert(next_pos);
                            let mut new_keys = keys.clone();
                            let mut new_doors = doors.clone();
                            if let Tile::Door(k) = tile {
                                new_doors.push(k);
                            }
                            if let Tile::Key(k) = tile {
                                new_keys.push(k);
                                found.insert(tile, Path::new(steps + 1, &new_keys, &new_doors));
                            }

                            queue.push_back((next_pos, steps + 1, new_keys, new_doors));
                        }
                    }
                }

                (*c, found)
            })
            .collect()
    }

    // Shortest path to collect all keys
    fn collect_keys(graph: &HashMap<Tile, HashMap<Tile, Path>>, all_keys: &[char], start: Tile) -> u32 {
        let mut queue = BinaryHeap::from([(Reverse(0), Reverse(CollectedKeys { keys: 0 }), start)]);
        let mut visited = HashSet::new();
        while let Some((Reverse(steps), Reverse(keys), tile)) = queue.pop() {
            // Collected all keys
            if keys.num_collected() == all_keys.len() {
                return steps;
            }

            // Don't revisit same key with same path before
            if visited.contains(&(tile, keys)) {
                continue;
            }
            visited.insert((tile, keys));

            // Try possible next keys
            for next_key in all_keys {
                // No need to collect this key again
                if keys.in_possession(*next_key) {
                    continue;
                }

                let key_tile = Tile::Key(*next_key);
                let path = &graph[&tile][&key_tile];

                // Only accept this path if we have all keys for it (and the key is collectible for this robot)
                if path
                    .doors
                    .iter()
                    .all(|door| !all_keys.contains(door) || keys.in_possession(*door))
                {
                    let mut new_keys = keys;
                    path.keys.iter().for_each(|ck| new_keys.collect(*ck));
                    queue.push((Reverse(steps + path.length), Reverse(new_keys), key_tile));
                }
            }
        }

        unreachable!()
    }
}

impl Solution for Solution18 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let map = input.grid().map_elements(Tile::from_char);
        let graph = Self::create_graph(&map);

        let all_keys = map
            .iter()
            .filter_map(|t| if let Tile::Key(k) = t { Some(*k) } else { None })
            .collect_vec();

        Self::collect_keys(&graph, &all_keys, Tile::Entrance).into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut map = input.grid().map_elements(Tile::from_char);

        // Transform entrance
        let main = map
            .enumerate()
            .find(|(_, tile)| matches!(tile, Tile::Entrance))
            .unwrap()
            .0;
        map.set(&main, Tile::Wall);
        for idx in main.von_neumann_neighbors(map.size()) {
            map.set(&idx, Tile::Wall);
        }
        let north = main.advance(Direction::North);
        let south = main.advance(Direction::South);
        map.set(&north.advance(Direction::West), Tile::SubEntrance(0));
        map.set(&north.advance(Direction::East), Tile::SubEntrance(1));
        map.set(&south.advance(Direction::West), Tile::SubEntrance(2));
        map.set(&south.advance(Direction::East), Tile::SubEntrance(3));

        // Create graph
        let graph = Self::create_graph(&map);

        // Shortest paths in each section, ignoring doors where keys are elsewhere
        // Technically not fool proof (if sections are too dependent), but works here
        let shortest_paths = (0..4)
            .map(|si| {
                let sub_entrance = Tile::SubEntrance(si);
                let reachable_keys = graph[&sub_entrance]
                    .keys()
                    .filter_map(|t| match t {
                        Tile::Key(k) => Some(*k),
                        _ => None,
                    })
                    .collect_vec();

                Self::collect_keys(&graph, &reachable_keys, sub_entrance)
            })
            .collect_vec();

        // Sum each section
        shortest_paths.into_iter().sum::<u32>().into_some()
    }
}
