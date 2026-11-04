use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use regex::Regex;

pub struct Solution04;
impl Solution04 {
    fn parse(input: ProblemInput) -> Vec<(String, u32, String)> {
        let room_rgx = Regex::new(r"^(?P<room>[a-z-]+)-(?P<sector>\d+)\[(?P<checksum>[a-z]{5})\]$").unwrap();
        input
            .lines()
            .into_iter()
            .map(|l| {
                let captures = room_rgx.captures(&l).unwrap();
                let room = captures.name("room").unwrap().as_str().to_string();
                let sector = captures.name("sector").unwrap().as_str().parse().unwrap();
                let checksum = captures.name("checksum").unwrap().as_str().to_string();
                (room, sector, checksum)
            })
            .collect_vec()
    }

    fn compute_checksum(room: &str) -> String {
        room.chars()
            .filter(|c| *c != '-')
            .counts()
            .into_iter()
            .sorted_by_key(|&(c, n)| (-(n as isize), c))
            .take(5)
            .map(|(c, _)| c)
            .join("")
    }

    fn decrypt_room(room: &str, sector: u32) -> String {
        room.chars()
            .map(|c| match c {
                '-' => ' ',
                _ => ((((c as u8 - b'a') as u32 + sector) % 26) as u8 + b'a') as char,
            })
            .join("")
    }
}

impl Solution for Solution04 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(1514),
            ProblemResult::U32(245102),
            ProblemResult::NoSample,
            ProblemResult::U32(324),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let rooms = Self::parse(input);

        rooms
            .into_iter()
            .filter(|(room, _, checksum)| Self::compute_checksum(room).as_str() == checksum)
            .map(|(_, sector, _)| sector)
            .sum::<u32>()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let rooms = Self::parse(input);
        rooms
            .into_iter()
            .find(|(room, sector, _)| Self::decrypt_room(room, *sector) == "northpole object storage")
            .unwrap()
            .1
            .to_result()
    }
}
