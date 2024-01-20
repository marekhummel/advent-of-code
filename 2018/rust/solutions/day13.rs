use std::cell::RefCell;
use std::collections::HashSet;

use aoc_lib::cartesian::{Direction, Grid, Index};
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

#[derive(Debug)]
struct Cart {
    idx: Index,
    dir: Direction,
    turn: u8,
}

pub struct Solution13;
impl Solution13 {
    fn parse(input: ProblemInput) -> (Grid<char>, Vec<RefCell<Cart>>) {
        let initial = input.grid();

        let carts = initial
            .enumerate()
            .filter_map(|(idx, ch)| {
                let dir = match ch {
                    '^' => Direction::North,
                    '>' => Direction::East,
                    'v' => Direction::South,
                    '<' => Direction::West,
                    _ => return None,
                };

                Some(RefCell::new(Cart { idx, dir, turn: 0 }))
            })
            .collect_vec();

        let tracks = initial.map_elements(|ch| match ch {
            '^' | 'v' => '|',
            '<' | '>' => '-',
            _ => *ch,
        });

        (tracks, carts)
    }

    fn cart_tick(cart: &mut Cart, tracks: &Grid<char>) {
        cart.dir = match tracks.get(&cart.idx) {
            '/' => match cart.dir {
                Direction::North | Direction::South => cart.dir.right(),
                Direction::East | Direction::West => cart.dir.left(),
                _ => unreachable!(),
            },
            '\\' => match cart.dir {
                Direction::North | Direction::South => cart.dir.left(),
                Direction::East | Direction::West => cart.dir.right(),
                _ => unreachable!(),
            },
            '+' => {
                let next = match cart.turn {
                    0 => cart.dir.left(),
                    1 => cart.dir,
                    2 => cart.dir.right(),
                    _ => unreachable!(),
                };

                cart.turn = (cart.turn + 1) % 3;
                next
            }
            '|' | '-' => cart.dir,
            _ => unreachable!(),
        };

        cart.idx = cart.idx.advance(cart.dir);
    }
}

impl Solution for Solution13 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let (tracks, mut carts) = Self::parse(input);

        loop {
            carts.sort_by_key(|cart| (cart.borrow().idx.j, cart.borrow().idx.i));

            for (i, mut cart) in carts.iter().map(|rc| rc.borrow_mut()).enumerate() {
                Self::cart_tick(&mut cart, &tracks);

                // Compare cart with other carts for crashes. Note that the order of updates is important, some
                // carts have already moved, others haven't, hence the RefCells.
                if carts
                    .iter()
                    .enumerate()
                    .any(|(j, other)| i != j && cart.idx == other.borrow().idx)
                {
                    return format!("{},{}", cart.idx.i, cart.idx.j).into_some();
                }
            }
        }
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let (tracks, mut carts) = Self::parse(input);

        loop {
            carts.sort_by_key(|cart| (cart.borrow().idx.j, cart.borrow().idx.i));

            let mut crashed = HashSet::new();
            for (i, mut cart) in carts.iter().map(|rc| rc.borrow_mut()).enumerate() {
                Self::cart_tick(&mut cart, &tracks);

                for (j, _) in carts
                    .iter()
                    .enumerate()
                    .filter(|(j, other)| i != *j && cart.idx == other.borrow().idx)
                {
                    crashed.insert(i);
                    crashed.insert(j);
                }
            }

            for crash_cart in crashed.into_iter().sorted().rev() {
                carts.remove(crash_cart);
            }

            if carts.len() == 1 {
                let idx = carts[0].borrow().idx;
                return format!("{},{}", idx.i, idx.j).into_some();
            }
        }
    }
}
