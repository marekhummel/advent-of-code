use std::collections::HashMap;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

#[derive(Debug)]
#[allow(dead_code)] // technically filename is unused, but its kept for clarity
enum Entry {
    File(String, u64),
    Directory(String),
}

pub struct Solution07;
impl Solution07 {
    fn parse(input: ProblemInput) -> HashMap<Vec<String>, Vec<Entry>> {
        let mut tree: HashMap<Vec<String>, Vec<Entry>> = HashMap::new();
        let mut cwd = vec![];

        for line in input.lines() {
            if let Some(command) = line.strip_prefix("$ ") {
                if let Some(dir) = command.strip_prefix("cd ") {
                    // CD
                    match dir {
                        "/" => cwd = vec![],
                        ".." => _ = cwd.pop(),
                        _ => cwd.push(dir.to_string()),
                    }
                }
            } else {
                // Output line of LS command
                let entry = if let Some(dirname) = line.strip_prefix("dir ") {
                    Entry::Directory(dirname.to_string())
                } else {
                    let (size, file) = line.split_once(' ').unwrap();
                    Entry::File(file.to_string(), size.parse().unwrap())
                };
                tree.entry(cwd.clone()).or_default().push(entry);
            }
        }

        tree
    }

    fn dir_size(directory: Vec<String>, files: &HashMap<Vec<String>, Vec<Entry>>) -> u64 {
        files[&directory]
            .iter()
            .map(|content| match content {
                Entry::File(_, size) => *size,
                Entry::Directory(subdir) => {
                    let mut full_path = directory.clone();
                    full_path.push(subdir.clone());
                    Self::dir_size(full_path, files)
                }
            })
            .sum()
    }
}

impl Solution for Solution07 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U64(95437),
            ProblemResult::U64(1844187),
            ProblemResult::U64(24933642),
            ProblemResult::U64(4978279),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let tree = Self::parse(input);
        let dir_sizes = tree.keys().map(|dir| Self::dir_size(dir.clone(), &tree));
        dir_sizes.filter(|&s| s <= 100000).sum::<u64>().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let tree = Self::parse(input);

        let occupied = Self::dir_size(vec![], &tree);
        let unused_space = 70_000_000 - occupied;
        let to_be_deleted_space = 30_000_000 - unused_space;

        let dir_sizes = tree.keys().map(|dir| Self::dir_size(dir.clone(), &tree));
        let smallest = dir_sizes.filter(|&ds| ds > to_be_deleted_space).min().unwrap();
        smallest.to_result()
    }
}
