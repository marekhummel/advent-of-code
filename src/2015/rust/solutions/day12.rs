use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use serde_json::Value;

pub struct Solution12;
impl Solution12 {
    fn sum_values(object: &Value, ignore_red: bool) -> i32 {
        match object {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::String(_) => 0,
            Value::Number(n) => n.as_i64().unwrap() as i32,
            Value::Array(a) => a.iter().map(|v| Self::sum_values(v, ignore_red)).sum(),
            Value::Object(o) => {
                if ignore_red && o.values().any(|v| matches!(v, Value::String(s) if s == "red")) {
                    return 0;
                }
                o.values().map(|v| Self::sum_values(v, ignore_red)).sum()
            }
        }
    }
}

impl Solution for Solution12 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I32(6),
            ProblemResult::I32(111754),
            ProblemResult::I32(4),
            ProblemResult::I32(65402),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let dump = input.string();
        let json = serde_json::from_str(&dump).unwrap();
        Self::sum_values(&json, false).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let dump = input.string();
        let json = serde_json::from_str(&dump).unwrap();
        Self::sum_values(&json, true).to_result()
    }
}
