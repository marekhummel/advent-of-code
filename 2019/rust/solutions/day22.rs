use aoc_lib::math;
use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
enum Shuffle {
    DealNew,
    DealInc(usize),
    Cut(usize),
    CutInv(usize),
}

impl Shuffle {
    fn apply(&self, deck: &[u32]) -> Vec<u32> {
        match self {
            Shuffle::DealNew => deck.iter().copied().rev().collect(),
            Shuffle::DealInc(inc) => {
                let mut new_deck = vec![0; deck.len()];
                let mut table_pos = 0;
                for card in deck {
                    new_deck[table_pos] = *card;
                    table_pos = (table_pos + inc) % deck.len();
                }
                new_deck
            }
            Shuffle::Cut(off) => {
                let mut new_deck = deck.to_vec();
                new_deck.rotate_left(*off);
                new_deck
            }
            Shuffle::CutInv(off) => {
                let mut new_deck = deck.to_vec();
                new_deck.rotate_right(*off);
                new_deck
            }
        }
    }
}

pub struct Solution22;
impl Solution22 {
    fn parse(input: ProblemInput) -> Vec<Shuffle> {
        input
            .lines()
            .into_iter()
            .map(|l| match &l.split_whitespace().collect_vec()[..] {
                ["deal", "into", "new", "stack"] => Shuffle::DealNew,
                ["deal", "with", "increment", inc] => Shuffle::DealInc(inc.parse().unwrap()),
                ["cut", offset] => match offset.strip_prefix('-') {
                    Some(suffix) => Shuffle::CutInv(suffix.parse().unwrap()),
                    None => Shuffle::Cut(offset.parse().unwrap()),
                },
                _ => unreachable!(),
            })
            .collect()
    }
}

impl Solution for Solution22 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let (deck_size, card) = if is_sample { (10, 0) } else { (10007, 2019) };
        let shuffles = Self::parse(input);
        let mut deck = (0..deck_size).collect_vec();
        for shuffle in shuffles {
            deck = shuffle.apply(&deck);
        }
        deck.into_iter().position(|c| c == card).unwrap().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let deck_size = if is_sample { 10 } else { 119315717514047 };
        let mut pos = if is_sample { 0 } else { 2020 };
        let mut reps: i128 = if is_sample { 1 } else { 101741582076661 };

        // Describe permutation done by entire input as linear function: p := delta * p + offset
        let shuffles = Self::parse(input);
        let (mut delta, mut offset) = (1i128, 0i128);
        for shuffle in &shuffles {
            (delta, offset) = match shuffle {
                Shuffle::DealNew => (-delta, (offset - delta) % deck_size),
                Shuffle::Cut(cut) => (delta, (offset + *cut as i128 * delta) % deck_size),
                Shuffle::CutInv(cut) => (delta, (offset - *cut as i128 * delta) % deck_size),
                Shuffle::DealInc(inc) => {
                    let inv = math::mod_inverse(*inc as i128, deck_size).unwrap();
                    ((delta * inv) % deck_size, offset)
                }
            };
        }

        // Instead of applying function n times, make use of double applications, and proceed similar to the Horner schema
        while reps > 0 {
            if reps & 1 == 1 {
                // Applying the function to our pos, gives the card value at that pos and thus the previous position.
                pos = (delta * pos + offset) % deck_size;
                reps -= 1;
            } else {
                // p := a * (a * p + b) + b = a^2 * p + (a*b + b)
                (delta, offset) = ((delta * delta) % deck_size, (delta * offset + offset) % deck_size);
                reps >>= 1;
            }
        }
        pos.rem_euclid(deck_size).to_result()
    }
}
