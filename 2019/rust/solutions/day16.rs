use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution16;
impl Solution16 {
    const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];
    fn pattern(position: usize) -> impl Iterator<Item = i32> {
        Self::BASE_PATTERN
            .iter()
            .flat_map(move |&x| std::iter::repeat(x).take(position))
            .cycle()
            .skip(1)
    }

    fn fft(signal: Vec<u8>) -> Vec<u8> {
        (1..=signal.len())
            .map(|pos| {
                (signal
                    .iter()
                    .zip(Self::pattern(pos))
                    .map(|(&s, p)| s as i32 * p)
                    .sum::<i32>()
                    % 10)
                    .unsigned_abs() as u8
            })
            .collect()
    }
}

impl Solution for Solution16 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let signal = input.string().bytes().map(|b| b - b'0').collect_vec();
        let decoded = (0..100).fold(signal, |phase_signal, _| Self::fft(phase_signal));
        decoded.into_iter().take(8).map(|s| s.to_string()).join("").to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let sig_str = input.string();
        let length = sig_str.len() * 10000;
        let mut signal = sig_str.bytes().map(|b| b - b'0').cycle().take(length).collect_vec();

        let offset = signal.iter().take(7).fold(0usize, |acc, d| acc * 10 + *d as usize);

        // If this is true, then the summation will only include 00..111 pattern
        assert!(offset > length / 2);

        // Cut signal
        signal = signal[offset..].to_vec();

        // Apply fft with simpler algorithm due to simplification of pattern
        let decoded = (0..100).fold(signal, |phase_signal, _| {
            // New signal is just cumulative sum of old pattern
            let mut new_signal = phase_signal
                .into_iter()
                .rev()
                .scan(0u32, |acc, d| {
                    *acc += d as u32;
                    Some((*acc % 10) as u8)
                })
                .collect_vec();
            new_signal.reverse();
            new_signal
        });

        decoded.into_iter().take(8).map(|s| s.to_string()).join("").to_result()
    }
}
