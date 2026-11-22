use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use serde_json::Value;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let dump = input.string();
    let json = serde_json::from_str(&dump).unwrap();
    sum_values(&json, false).into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let dump = input.string();
    let json = serde_json::from_str(&dump).unwrap();
    sum_values(&json, true).into()
}

fn sum_values(object: &Value, ignore_red: bool) -> i32 {
    match object {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap() as i32,
        Value::Array(a) => a.iter().map(|v| sum_values(v, ignore_red)).sum(),
        Value::Object(o) => {
            if ignore_red && o.values().any(|v| matches!(v, Value::String(s) if s == "red")) {
                return 0;
            }
            o.values().map(|v| sum_values(v, ignore_red)).sum()
        }
    }
}
