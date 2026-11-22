use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let licence = parse(input);
    let root = Node::from_licence(&mut licence.into_iter());
    root.total_metadata().into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let licence = parse(input);
    let root = Node::from_licence(&mut licence.into_iter());
    root.value().into()
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn from_licence<I: Iterator<Item = u32>>(iter: &mut I) -> Self {
        let num_children = iter.next().unwrap();
        let num_metadata = iter.next().unwrap();

        let children = (0..num_children).map(|_| Self::from_licence(iter)).collect();
        let metadata = iter.take(num_metadata as usize).collect();
        Node { children, metadata }
    }

    fn total_metadata(&self) -> u32 {
        self.metadata.iter().sum::<u32>() + self.children.iter().map(|c| c.total_metadata()).sum::<u32>()
    }

    fn value(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().sum::<u32>()
        } else {
            self.metadata
                .iter()
                .filter(|i| 1 <= **i && **i <= self.children.len() as u32)
                .map(|i| self.children[*i as usize - 1].value())
                .sum()
        }
    }
}

fn parse(input: ProblemInput) -> Vec<u32> {
    input.string().split_whitespace().parsed().collect()
}
