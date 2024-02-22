use aoc_lib::cartesian::{Direction, Grid, Index};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use aoc_lib::specific::intcode::Program;
use itertools::Itertools;

type Robot = (Index, Direction);
type Function = Vec<Command>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    Left,
    Right,
    Forward(usize),
    Invoke(usize),
}

impl Command {
    fn is_invocation(&self) -> bool {
        matches!(self, Command::Invoke(_))
    }

    fn to_ascii(&self) -> Vec<u8> {
        match self {
            Command::Left => vec![b'L'],
            Command::Right => vec![b'R'],
            Command::Forward(s) => s.to_string().bytes().collect(),
            Command::Invoke(f) => vec![*f as u8 + b'A'],
        }
    }
}

pub struct Solution17;
impl Solution17 {
    fn read_cameras(mut robot: Program) -> (Grid<bool>, Robot) {
        robot.execute();

        // Compute rows of grid
        let mut robot_info = None;
        let mut rows = vec![];
        let mut row = vec![];
        for px in robot.output {
            match (px as u8) as char {
                '.' => row.push(false),
                '#' => row.push(true),
                r @ ('^' | '<' | '>' | 'v') => {
                    robot_info = Some((Index::new(row.len(), rows.len()), r.try_into().unwrap()));
                    row.push(true);
                }
                '\n' => {
                    rows.push(row);
                    row = vec![];
                }
                _ => unreachable!(),
            }
        }

        // Pad each row
        let max_row = rows.iter().map(|r| r.len()).max().unwrap();
        rows.iter_mut().for_each(|r| r.resize(max_row, false));
        (Grid::new(rows), robot_info.unwrap())
    }

    fn compute_path(grid: &Grid<bool>, mut pos: Index, mut dir: Direction) -> Vec<Command> {
        let mut path = vec![];
        let mut steps = 0;
        loop {
            // Try straight first
            if let Some(true) = grid.get_checked(&pos.advance(dir)) {
                steps += 1;
                pos = pos.advance(dir);
                continue;
            }

            // We can't continue forward, track steps
            if steps > 0 {
                path.push(Command::Forward(steps));
                steps = 0;
            }

            // Turn left
            if let Some(true) = grid.get_checked(&pos.advance(dir.left())) {
                path.push(Command::Left);
                dir = dir.left();
                continue;
            }

            // Turn right
            if let Some(true) = grid.get_checked(&pos.advance(dir.right())) {
                path.push(Command::Right);
                dir = dir.right();
                continue;
            }

            // No more options, end of scaffold
            return path;
        }
    }

    fn find_funcs(main: Function, funcs: Vec<Function>) -> Option<(Function, Vec<Function>)> {
        // Functions found, make sure main is not too long
        if main.iter().all(|c| c.is_invocation()) {
            return if main.len() <= 20 { Some((main, funcs)) } else { None };
        }

        // 3 functions defined, but not all movement commands covered
        if funcs.len() == 3 {
            return None;
        }

        // Try to define next function starting at the first movement command in main
        let func_start = main.iter().position(|c| !c.is_invocation()).unwrap();
        for len in 2..=20 {
            // Next invocation found, current function has to stop before
            if main[func_start + len - 1].is_invocation() {
                break;
            }

            // Define func
            let func = main[func_start..func_start + len].to_vec();

            // Define new main
            let mut i = 0;
            let mut new_main = Vec::new();
            while i < main.len() {
                if i + len <= main.len() && main[i..i + len] == func {
                    // Replace occurences of newly defined func with invocation
                    new_main.push(Command::Invoke(funcs.len()));
                    i += len;
                } else {
                    new_main.push(main[i].clone());
                    i += 1;
                }
            }

            // Recurse with new function definition
            let new_funcs = [funcs.clone(), vec![func]].concat();
            if let Some(solution) = Self::find_funcs(new_main, new_funcs) {
                return Some(solution);
            }
        }

        None
    }

    fn func_to_input(func: Function) -> Vec<i128> {
        // Intersperse ascii commands with commata and add new line
        Itertools::intersperse(func.into_iter().map(|c| c.to_ascii()), vec![b','])
            .flatten()
            .chain([b'\n'])
            .map_into()
            .collect()
    }
}

impl Solution for Solution17 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        // Create grid
        let grid = if is_sample {
            input.grid().map_elements(|c| *c != '.')
        } else {
            Self::read_cameras(Program::init(&input.string())).0
        };

        // Find intersections
        let intersections = grid
            .enumerate()
            .filter(|(idx, scaff)| {
                **scaff
                    && idx
                        .von_neumann_neighbors(grid.size)
                        .into_iter()
                        .filter(|nb| *grid.get(nb))
                        .count()
                        == 4
            })
            .collect_vec();

        // Compute alignments
        let calibration = intersections.into_iter().map(|(idx, _)| idx.i * idx.j).sum::<usize>();
        calibration.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            // -- Helper input to check path finding and function generation
            // let grid_input = input.grid();
            // let (pos, dir): (Index, Direction) = grid_input
            //     .enumerate()
            //     .find_map(|(idx, &c)| c.try_into().ok().map(|d| (idx, d)))
            //     .unwrap();
            // let grid = grid_input.map_elements(|c| *c != '.');
            return ProblemResult::NoSample;
        }

        // Create grid
        let (grid, (pos, dir)) = Self::read_cameras(Program::init(&input.string()));

        // Create path and find functions
        let path = Self::compute_path(&grid, pos, dir);

        // Find functions
        let (main, functions) = Self::find_funcs(path, vec![]).unwrap();

        // Reset robot
        let mut robot = Program::init(&input.string());
        *robot.memory.get_mut(&0).unwrap() = 2;

        // Input main, functions and video feed flag
        robot.input.extend(Self::func_to_input(main));
        for func in functions {
            robot.input.extend(Self::func_to_input(func));
        }
        robot.input.extend(vec![b'n' as i128, b'\n' as i128]);

        // Run robot (other output is camera view before and after)
        robot.execute();
        robot.output.last().unwrap().to_result()
    }
}
