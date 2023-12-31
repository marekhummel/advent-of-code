use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use regex::Regex;

pub struct Solution08;
impl Solution08 {}

impl Solution for Solution08 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let hexa_rgx = Regex::new(r"\\x[0-9a-fA-F]{2}").unwrap();

        let mut delta = 0;
        for code_string in input.lines() {
            let code_len = code_string.len();

            let no_escaped = code_string.replace("\\\\", "_").replace("\\\"", "_");
            let no_hexa = hexa_rgx.replace_all(&no_escaped, "_");
            let no_quotes = no_hexa.replace('\"', "");

            let char_len = no_quotes.len();
            delta += code_len - char_len;
        }

        delta.into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut delta = 0;
        for code_string in input.lines() {
            let code_len = code_string.len();

            let escape_slash = code_string.replace('\\', "\\\\");
            let escape_quotes = escape_slash.replace('\"', "\\\"");
            let add_quotes = format!("\"{escape_quotes}\"");

            let escaped_len = add_quotes.len();
            delta += escaped_len - code_len;
        }

        delta.into_some()
    }
}

// 300 lines
// 6502 chars
// 5758 remaining
