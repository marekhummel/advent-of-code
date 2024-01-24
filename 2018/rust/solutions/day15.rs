use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

use aoc_lib::cartesian::{Grid, Index};
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Team {
    Elf,
    Goblin,
}

#[derive(Debug, Clone)]
struct Entity {
    team: Team,
    idx: Index,
    hitpoints: u8,
    dmg: u8,
}

pub struct Solution15;
impl Solution15 {
    fn parse(input: ProblemInput) -> (Grid<bool>, Vec<RefCell<Entity>>) {
        let initial = input.grid();

        let entities = initial
            .enumerate()
            .filter_map(|(idx, ch)| {
                let etype = match ch {
                    'E' => Team::Elf,
                    'G' => Team::Goblin,
                    _ => return None,
                };

                Some(RefCell::new(Entity {
                    team: etype,
                    idx,
                    hitpoints: 200,
                    dmg: 3,
                }))
            })
            .collect_vec();

        let caves = initial.map_elements(|ch| *ch != '#');

        (caves, entities)
    }

    fn combat(caves: &Grid<bool>, entities: &[RefCell<Entity>]) -> u32 {
        for n in 1.. {
            if !Self::round(caves, entities) {
                return entities.iter().map(|e| e.borrow().hitpoints as u32).sum::<u32>() * (n - 1);
            }
        }

        unreachable!()
    }

    fn round(caves: &Grid<bool>, entities: &[RefCell<Entity>]) -> bool {
        for entity in entities.iter().sorted_by_key(|ent| ent.borrow().idx) {
            if entity.borrow().hitpoints == 0 {
                continue;
            }

            if !Self::turn(entity, caves, entities) {
                return false;
            }
        }

        true
    }

    fn turn(unit: &RefCell<Entity>, caves: &Grid<bool>, entities: &[RefCell<Entity>]) -> bool {
        let targets = entities
            .iter()
            .filter(|e| e.borrow().team != unit.borrow().team && e.borrow().hitpoints > 0)
            .collect_vec();

        if targets.is_empty() {
            return false;
        }

        // Check if any targets are adjacent to unit
        let curr_idx = unit.borrow().idx;
        if targets.iter().all(|t| t.borrow().idx.dist(&curr_idx) > 1) {
            // Move
            let cave_size = caves.size();
            let entity_squares: HashSet<_> = entities
                .iter()
                .filter(|e| e.borrow().hitpoints > 0 && e.borrow().idx != curr_idx)
                .map(|e| e.borrow().idx)
                .collect();
            let in_range_squares = targets
                .iter()
                .flat_map(|t| t.borrow().idx.von_neumann_neighbors(cave_size))
                .filter(|sq| *caves.get(sq) && !entity_squares.contains(sq))
                .unique()
                .collect_vec();

            let chosen = Self::find_chosen_square(curr_idx, &in_range_squares, caves, &entity_squares);
            if let Some(chosen_square) = chosen {
                let step = Self::find_chosen_square(
                    chosen_square,
                    &curr_idx.von_neumann_neighbors(cave_size),
                    caves,
                    &entity_squares,
                );

                unit.borrow_mut().idx = step.unwrap();
            }
        }

        // Check again if any targets are now in attack range
        let targetable = targets
            .iter()
            .filter(|t| t.borrow().idx.dist(&unit.borrow().idx) == 1)
            .min_by_key(|t| (t.borrow().hitpoints, t.borrow().idx));
        if let Some(target) = targetable {
            let hp = target.borrow().hitpoints;
            target.borrow_mut().hitpoints = hp.saturating_sub(unit.borrow().dmg);
        }

        true
    }

    fn find_chosen_square(
        start: Index,
        squares: &[Index],
        caves: &Grid<bool>,
        entity_squares: &HashSet<Index>,
    ) -> Option<Index> {
        let cave_size = caves.size();

        let mut seen = HashSet::new();
        let mut queue = BinaryHeap::from([Reverse((0, start))]);

        while let Some(Reverse((dist, idx))) = queue.pop() {
            if seen.contains(&idx) || entity_squares.contains(&idx) || !caves.get(&idx) {
                continue;
            }

            if squares.contains(&idx) {
                return Some(idx);
            }

            seen.insert(idx);
            for nb in idx.von_neumann_neighbors(cave_size) {
                queue.push(Reverse((dist + 1, nb)))
            }
        }

        None
    }
}

impl Solution for Solution15 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let (caves, entities) = Self::parse(input);

        Self::combat(&caves, &entities).into_some()
    }

    // Needs 5-10 secs.
    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let (caves, entities) = Self::parse(input);

        loop {
            for e in &entities {
                if e.borrow().team == Team::Elf {
                    e.borrow_mut().dmg += 1;
                }
            }

            let army = entities.iter().cloned().collect_vec();
            let outcome = Self::combat(&caves, &army);
            if army
                .iter()
                .filter(|e| e.borrow().team == Team::Elf)
                .all(|e| e.borrow().hitpoints > 0)
            {
                return outcome.into_some();
            }
        }
    }
}
