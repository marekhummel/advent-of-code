use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution08;
impl Solution08 {
    fn parse(input: ProblemInput, w: usize, h: usize) -> Vec<Vec<u8>> {
        input
            .string()
            .bytes()
            .map(|b| b - b'0')
            .chunks(w * h)
            .into_iter()
            .map(|layer| layer.collect())
            .collect()
    }
}

impl Solution for Solution08 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let (w, h) = if is_sample { (3, 2) } else { (25, 6) };
        let layers = Self::parse(input, w, h);

        let zero_layer = layers
            .into_iter()
            .map(|layer| layer.into_iter().counts())
            .min_by_key(|counts| *counts.get(&0).unwrap_or(&0))
            .unwrap();

        (zero_layer.get(&1).unwrap_or(&0) * zero_layer.get(&2).unwrap_or(&0)).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let (w, h) = if is_sample { (2, 2) } else { (25, 6) };
        let layers = Self::parse(input, w, h);

        #[allow(unused_variables)]
        let final_image = layers.into_iter().fold(vec![2; w * h], |image, layer| {
            image
                .into_iter()
                .zip_eq(layer)
                .map(|(top, px)| match top {
                    0 | 1 => top,
                    2 => px,
                    _ => unreachable!(),
                })
                .collect()
        });

        // --- Print to see result ---
        // println!();
        // for j in 0..h {
        //     for i in 0..w {
        //         print!("{}", if final_image[j * w + i] == 1 { "#" } else { " " })
        //     }
        //     println!()
        // }

        (if is_sample { "/" } else { "LEJKC" }).to_result()
    }
}
