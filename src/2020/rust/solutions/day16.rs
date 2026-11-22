use std::collections::HashMap;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let (rules, _, nearby_tickets) = parse(input);

    let invalid_values = find_invalid_values(&rules, &nearby_tickets);
    let scanning_error_rate = invalid_values.iter().sum::<u64>();

    scanning_error_rate.into()
}

pub fn part2(input: ProblemInput, is_sample: bool) -> ProblemResult {
    if is_sample {
        return ProblemResult::NoSample;
    }

    let (rules, my_ticket, mut nearby_tickets) = parse(input);

    // Remove invalid tickets
    let invalid_values = find_invalid_values(&rules, &nearby_tickets);
    nearby_tickets.retain(|tckt| tckt.iter().all(|val| !invalid_values.contains(val)));

    // Find out which field is what
    let mut assigned_fields = vec![None; rules.len()];
    for i in (0..rules.len()).cycle() {
        if assigned_fields[i].is_some() {
            continue;
        }

        let field_values = nearby_tickets.iter().map(|tckt| tckt[i]).collect_vec();
        let possible_fields = rules
            .iter()
            .filter(|(_, ranges)| {
                field_values
                    .iter()
                    .all(|val| ranges.iter().any(|(low, high)| (low..=high).contains(&val)))
            })
            .map(|(field, _)| field)
            .filter(|field| !assigned_fields.contains(&Some(*field)))
            .collect_vec();

        assert!(!possible_fields.is_empty());
        if let [field] = &possible_fields[..] {
            assigned_fields[i] = Some(field);
            if assigned_fields.iter().all(|f| f.is_some()) {
                break;
            }
        }
    }

    // Find indices of departure fields
    let depart_fields = assigned_fields
        .iter()
        .enumerate()
        .filter_map(|(i, field)| field.filter(|f| f.starts_with("departure")).map(|_| my_ticket[i]))
        .product::<u64>();

    depart_fields.into()
}

type Ticket = Vec<u64>;
type Range = (u64, u64);
type Rule = Vec<Range>;

fn parse(input: ProblemInput) -> (HashMap<String, Rule>, Ticket, Vec<Ticket>) {
    let lines = input.lines();
    let (rule_strs, my_ticket_str, nearby_strs) = lines.split(|l| l.is_empty()).collect_tuple().unwrap();

    let rules = rule_strs
        .iter()
        .map(|l| {
            let (field, ranges_str) = l.split_once(": ").unwrap();
            let ranges = ranges_str
                .split(" or ")
                .map(|rs| rs.split('-').parsed().collect_tuple().unwrap())
                .collect_vec();
            (field.to_string(), ranges)
        })
        .collect();

    let my_ticket = my_ticket_str[1].split(',').parsed().collect();

    let nearby_tickets = nearby_strs
        .iter()
        .skip(1)
        .map(|t| t.split(',').parsed().collect())
        .collect();

    (rules, my_ticket, nearby_tickets)
}

fn find_invalid_values(rules: &HashMap<String, Rule>, tickets: &[Ticket]) -> Vec<u64> {
    tickets
        .iter()
        .flatten()
        .filter(|value| {
            rules
                .values()
                .all(|rng| !rng.iter().any(|(low, high)| (low..=high).contains(value)))
        })
        .copied()
        .collect()
}
