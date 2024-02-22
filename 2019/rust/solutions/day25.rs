use aoc_lib::cartesian::Direction;
use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use aoc_lib::specific::intcode::Program;
use itertools::Itertools;
use regex::Regex;

enum Command {
    Move(Direction),
    Take(String),
    Drop(String),
}

struct Droid {
    program: Program,
    intcode: String,
    visited: Vec<String>,
    collected_items: Vec<String>,
    forbidden_items: Vec<String>,
    pressure_sens_floor: Option<Vec<Direction>>,
}

impl Droid {
    fn new(intcode: String) -> Self {
        Droid {
            program: Program::init(&intcode),
            intcode,
            visited: vec![],
            pressure_sens_floor: None,
            collected_items: vec![],
            forbidden_items: vec!["infinite loop".to_string()], // have to hardcode this item
        }
    }

    fn reset(&mut self) {
        self.visited.clear();
        self.collected_items.clear();
        self.program = Program::init(&self.intcode);
    }

    // Explore ship until either back at start (success) or we took an forbidden item (fail)
    fn explore(&mut self, movements: &mut Vec<Direction>) -> bool {
        let (lines, _) = self.execute_until_command();
        let Some((room, doors, items_room)) = Self::parse_output(&lines) else {
            let Some(last_item) = self.collected_items.last() else {
                panic!()
            };
            self.forbidden_items.push(last_item.clone());
            return false;
        };

        let prev_dir = *movements.last().unwrap_or(&Direction::None);

        if room == "Pressure-Sensitive Floor" {
            self.pressure_sens_floor = Some(movements.clone());
        }

        for item in items_room {
            if self.forbidden_items.contains(&item) {
                continue;
            }

            self.give_command(Command::Take(item.clone()), true);
            self.collected_items.push(item);
        }

        if !self.visited.contains(&room) {
            self.visited.push(room);
            for dir in doors {
                if dir == prev_dir.inverse() {
                    continue;
                }

                self.give_command(Command::Move(dir), false);
                movements.push(dir);
                if !self.explore(movements) {
                    return false;
                }
            }
        }

        if prev_dir != Direction::None {
            let _ = movements.pop();
            self.give_command(Command::Move(prev_dir.inverse()), true);
        }

        true
    }

    // Run robot until command is requested
    fn execute_until_command(&mut self) -> (Vec<String>, bool) {
        let mut lines = Vec::new();
        loop {
            let mut line = String::new();
            loop {
                if let Some(output) = self.program.execute_until_output() {
                    let ch = output as u8 as char;
                    if ch == '\n' {
                        lines.push(line.clone());
                        break;
                    }

                    line.push(ch);
                } else {
                    return (lines, false);
                }
            }

            if line == "Command?" {
                return (lines, true);
            }
        }
    }

    // Send command to droid and optionally execute it
    fn give_command(&mut self, command: Command, execute_immediately: bool) {
        let str_cmd = match command {
            Command::Move(dir) => match dir {
                Direction::North => "north".to_string(),
                Direction::East => "east".to_string(),
                Direction::South => "south".to_string(),
                Direction::West => "west".to_string(),
                Direction::None => panic!(),
            },
            Command::Take(item) => ["take", item.as_str()].join(" "),
            Command::Drop(item) => ["drop", item.as_str()].join(" "),
        };

        self.program.input.extend(str_cmd.bytes().parsed::<i128>());
        self.program.input.push_back(b'\n' as i128);

        if execute_immediately {
            let _ = self.execute_until_command();
        }
    }

    // Parses list of output lines to find room name, doors and items
    fn parse_output(output: &[String]) -> Option<(String, Vec<Direction>, Vec<String>)> {
        let room_opt = output
            .iter()
            .find_map(|l| l.strip_prefix("== ").and_then(|sl| sl.strip_suffix(" ==")));

        room_opt.map(|room| {
            let doors = output
                .iter()
                .skip_while(|l| *l != "Doors here lead:")
                .skip(1)
                .take_while(|l| !l.is_empty())
                .map(|l| l.trim_start_matches("- ").try_into().unwrap())
                .collect::<Vec<Direction>>();

            let items_room = output
                .iter()
                .skip_while(|l| *l != "Items here:")
                .skip(1)
                .take_while(|l| !l.is_empty())
                .map(|l| l.trim_start_matches("- ").to_string())
                .collect_vec();

            (room.to_string(), doors, items_room)
        })
    }
}

pub struct Solution25;
impl Solution25 {}

impl Solution for Solution25 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::NoSample,
            ProblemResult::String("134227456".to_string()),
            ProblemResult::NoPartTwo,
            ProblemResult::NoPartTwo,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        // Interactive
        // use std::io::{self, BufRead};
        // let stdin = io::stdin();
        // let mut droid = Droid::new(input.string());
        // loop {
        //     let (output, command_req) = droid.execute_until_command();
        //     println!("{}", output.join("\n"));

        //     if !command_req {
        //         break;
        //     }

        //     let cmd = stdin.lock().lines().next().unwrap().unwrap();
        //     droid.program.input.extend(cmd.bytes().parsed::<i128>());
        //     droid.program.input.push_back(b'\n' as i128);
        // }
        // return ProblemResult::Unsolved;

        // Explore ship and try to pick up all items
        let mut droid = Droid::new(input.string());
        while !droid.explore(&mut vec![]) {
            droid.reset();
        }

        // Walk back to pressure sensitive floor
        assert!(droid.pressure_sens_floor.is_some());
        let mut path = droid.pressure_sens_floor.clone().unwrap();
        let door = path.pop().unwrap();
        for dir in path {
            droid.give_command(Command::Move(dir), true);
        }

        // Try all item combinations
        let items = droid.collected_items.clone();
        for drop_items in items.iter().powerset() {
            // Drop
            for item in &drop_items {
                droid.give_command(Command::Drop(item.to_string()), true);
            }

            // Try plate
            droid.give_command(Command::Move(door), false);
            let (output, command_req) = droid.execute_until_command();
            if !(command_req) {
                // Success!
                let regex = Regex::new(r"\d+").unwrap();
                let str_output = output.join("\n");
                let passkey = regex.find(&str_output).unwrap();
                return passkey.as_str().to_result();
            }

            // Take back
            for item in &drop_items {
                droid.give_command(Command::Take(item.to_string()), true);
            }
        }

        ProblemResult::Unsolved
    }

    fn solve_version02(&self, _input: ProblemInput, _is_sample: bool) -> ProblemResult {
        ProblemResult::NoPartTwo
    }
}
