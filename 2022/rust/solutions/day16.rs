use std::collections::{HashMap, VecDeque};

use aoc_lib::graph::Graph;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use regex::Regex;

type Valve = u8;
type ValveSet = u64;
type FlowMap = HashMap<Valve, u32>;
type Tunnels = Graph<Valve>;

pub struct Solution16;
impl Solution16 {
    fn parse(input: ProblemInput) -> (FlowMap, Tunnels, Valve) {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let rgx = Regex::new(r"^Valve (?P<valve>[A-Z]{2}) has flow rate=(?P<flow>\d+); tunnels? leads? to valves? (?P<tunnels>(?:[A-Z]{2}(?:, )?)+)$")
            .unwrap();

        let parsed = input
            .lines()
            .into_iter()
            .map(|l| {
                let captures = rgx.captures(&l).unwrap();
                let valve = captures.name("valve").unwrap().as_str().to_string();
                let flow_rate = captures.name("flow").unwrap().as_str().parse().unwrap();
                let tunnel_list = captures.name("tunnels").unwrap().as_str();
                let tunnels = tunnel_list.split(", ").map(|t| t.to_string()).collect_vec();
                (valve, flow_rate, tunnels)
            })
            .collect_vec();

        let valve_map: HashMap<_, _> = parsed.iter().enumerate().map(|(i, v)| (&v.0, i as Valve)).collect();
        let flow_map: FlowMap = parsed.iter().map(|(v, f, _)| (valve_map[v], *f)).collect();
        let tunnels: Tunnels = parsed
            .iter()
            .map(|(v, _, t)| (valve_map[v], t.iter().map(|t| valve_map[t]).collect()))
            .collect();

        (flow_map, tunnels, valve_map[&"AA".to_string()])
    }

    /// Reduce tunnel graph to interconnect all valves and skip jammed valves.
    fn update_tunnel_network(tunnels: Tunnels, flow_map: &FlowMap, start_valve: Valve) -> Tunnels {
        let shortest_paths = tunnels.floyd_warshall();
        let valves = tunnels.vertices();
        let mut better_tunnels = Graph::empty();
        for (a, b) in valves.into_iter().tuple_combinations() {
            // Only add edge if both ends are not jammed or either valve is the start.
            if (flow_map[&a] != 0 && (flow_map[&b] != 0 || b == start_valve))
                || (flow_map[&b] != 0 && (flow_map[&a] != 0 || a == start_valve))
            {
                better_tunnels.add_weighted_edge(&a, &b, shortest_paths[&(a, b)] as i64, false);
            }
        }
        better_tunnels
    }

    fn compute_best_flows(tunnels: &Tunnels, flow_map: &FlowMap, start: Valve, max_time: u8) -> HashMap<ValveSet, u32> {
        // Total flow, time, current flow rate, location, open valves
        let mut queue = VecDeque::from([(0u32, 0, 0u32, start, 0)]);

        let mut max_flows = HashMap::new();
        while let Some((total_flow, time, current_flow_rate, location, open_valves)) = queue.pop_front() {
            // Compute flow at time = 30 (26) if idling from now on.
            let min_possible_flow = total_flow + (max_time - time) as u32 * current_flow_rate;
            max_flows
                .entry(open_valves)
                .and_modify(|e: &mut u32| *e = (*e).max(min_possible_flow))
                .or_insert(min_possible_flow);

            // Abort
            if time >= max_time {
                continue;
            }

            // Combine moving and opening the valve to drastically reduce search tree
            for tunnel in tunnels.adjacent_vertices(&location) {
                // Only move if target valve is not open
                if (open_valves >> tunnel) & 1 == 1 {
                    continue;
                }

                // Compute time needed to move and open the valve
                let dist = tunnels.get_weight(&location, &tunnel) as u8 + 1;
                let new_time = time + dist;
                if new_time > max_time {
                    continue;
                }

                // Append next state
                queue.push_back((
                    total_flow + dist as u32 * current_flow_rate,
                    new_time,
                    current_flow_rate + flow_map[&tunnel],
                    tunnel,
                    open_valves | 1 << tunnel,
                ));
            }
        }

        max_flows
    }
}

impl Solution for Solution16 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(1651),
            ProblemResult::U32(1617),
            ProblemResult::U32(1707),
            ProblemResult::U32(2171),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (flow_map, tunnels, valve_aa) = Self::parse(input);
        let better_tunnels = Self::update_tunnel_network(tunnels, &flow_map, valve_aa);

        let max_flows = Self::compute_best_flows(&better_tunnels, &flow_map, valve_aa, 30);
        max_flows.values().max().unwrap().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (flow_map, tunnels, valve_aa) = Self::parse(input);
        let better_tunnels = Self::update_tunnel_network(tunnels, &flow_map, valve_aa);
        let max_flows = Self::compute_best_flows(&better_tunnels, &flow_map, valve_aa, 26);

        let pair_paths = max_flows.iter().tuple_combinations();
        let non_overlapping = pair_paths.filter(|&((valves1, _), (valves2, _))| valves1 & valves2 == 0);
        let optimal = non_overlapping.map(|((_, flow1), (_, flow2))| flow1 + flow2).max();
        optimal.unwrap().to_result()
    }
}
