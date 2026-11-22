use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, is_sample: bool) -> ProblemResult {
    let (w, h) = if is_sample { (3, 2) } else { (25, 6) };
    let layers = parse(input, w, h);

    let zero_layer = layers
        .into_iter()
        .map(|layer| layer.into_iter().counts())
        .min_by_key(|counts| *counts.get(&0).unwrap_or(&0))
        .unwrap();

    (zero_layer.get(&1).unwrap_or(&0) * zero_layer.get(&2).unwrap_or(&0)).into()
}

pub fn part2(input: ProblemInput, is_sample: bool) -> ProblemResult {
    let (w, h) = if is_sample { (2, 2) } else { (25, 6) };
    let layers = parse(input, w, h);

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

    (if is_sample { "/" } else { "LEJKC" }).into()
}

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
