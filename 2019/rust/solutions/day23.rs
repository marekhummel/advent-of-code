use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use aoc_lib::specific::intcode::Program;
use itertools::Itertools;
use std::cell::RefCell;

type Packet = (i128, i128);

struct Network {
    computers: Vec<RefCell<Program>>,
    curr_addr: usize,
    nat_enabled: bool,
    nat_packet: Option<Packet>,
}

impl Network {
    fn new(intcode: String, enable_nat: bool) -> Self {
        let computers = vec![RefCell::new(Program::init(&intcode)); 50];

        Network {
            computers,
            curr_addr: 0,
            nat_enabled: enable_nat,
            nat_packet: None,
        }
    }

    fn init(&mut self) {
        for (addr, comp_ref) in self.computers.iter().enumerate() {
            let mut comp = comp_ref.borrow_mut();
            comp.default_input = Some(-1);
            comp.input.push_back(addr as i128);
        }
    }

    fn progress_one(&mut self, addr: usize) -> Option<(Packet, bool)> {
        let comp = &self.computers[addr];

        // Step
        comp.borrow_mut().execute_next_instruction();

        // Check if packet was sent
        let output = comp.borrow().output.clone();
        if let [dest, x, y] = output[..] {
            comp.borrow_mut().output.clear();

            if dest == 255 {
                // NAT packet found
                self.nat_packet = Some((x, y));

                if !self.nat_enabled {
                    return Some(((x, y), false));
                }
            } else {
                // Distribute packet to input queue
                let mut dest_comp = self.computers[dest as usize].borrow_mut();
                dest_comp.input.push_back(x);
                dest_comp.input.push_back(y);
            }
        }

        // Check idle
        if self.nat_enabled && self.is_idle() {
            let mut comp0 = self.computers[0].borrow_mut();

            if let Some((x, y)) = self.nat_packet {
                comp0.input.push_back(x);
                comp0.input.push_back(y);

                self.nat_packet = None;
                return Some(((x, y), true));
            }
        }

        None
    }

    fn is_idle(&self) -> bool {
        self.computers.iter().all(|c| c.borrow().input.is_empty())
    }
}

impl Iterator for Network {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.curr_addr = (self.curr_addr + 1) % 50;

            if let Some((result, nat_type)) = self.progress_one(self.curr_addr) {
                if nat_type == self.nat_enabled {
                    return Some(result);
                }
            }
        }
    }
}

pub struct Solution23;
impl Solution23 {}

impl Solution for Solution23 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let intcode = input.string();
        let mut network = Network::new(intcode, false);
        network.init();

        network.next().unwrap().1.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let intcode = input.string();
        let mut network = Network::new(intcode, true);
        network.init();

        network
            .map(|(_, y)| y)
            .tuple_windows()
            .find(|(py, y)| py == y)
            .unwrap()
            .0
            .to_result()
    }
}
