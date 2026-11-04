use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use aoc_lib::specific::intcode::Program;

pub struct Solution21;
impl Solution21 {}

impl Solution for Solution21 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::NoSample,
            ProblemResult::I128(19359533),
            ProblemResult::NoSample,
            ProblemResult::I128(1140310551),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        // Rules @ABCD:
        //   @.??. -> FAIL / DON'T CARE
        //   @.??? -> JUMP
        //   @???. -> NO JUMP
        //   @#..# -> JUMP
        //   @#.## -> JUMP
        //   @##.# -> JUMP
        //   @#### -> NO JUMP
        // KV SOLVER => (~A) | (~B & D) | (~C & D)
        let springscript = [
            "NOT A J\n",
            "NOT B T\n",
            "AND D T\n",
            "OR T J\n",
            "NOT C T\n",
            "AND D T\n",
            "OR T J\n",
            "WALK\n",
        ];

        let mut droid = Program::init(&input.string());
        droid
            .input
            .extend(springscript.into_iter().flat_map(|cmd| cmd.bytes().parsed::<i128>()));

        droid.execute();
        // droid.print_as_ascii();
        droid.output.last().unwrap().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        // Rules @ABCD:
        //   @ABCDEFGHI
        //   @.??.????? -> FAIL / DONT CARE
        //   @.???????? -> JUMP
        //   @???.????? -> NO JUMP
        //   @#..#????? -> JUMP
        //   @#.##????? -> JUMP
        //   @##.#..??? -> JUMP
        //   @##.#.#..? -> NO JUMP
        //   @##.#.#.#? -> JUMP
        //   @##.#.##?? -> NO JUMP
        //   @##.##???? -> DONT CARE
        //   @####????? -> NO JUMP
        //
        // KV Solver => Find solution which converts to <15 instructions.
        // Can't have double negative in more than one minterm since we only have one temp register.
        //
        // Solution J = (~A) | (D & ~B) | (D & ~C & ~F) | (D & ~C & ~G & H)
        // Compute ~J = (A) & (~D | B) & (~D | C | F) & (~D | C | G | ~H)
        let springscript = [
            "NOT H J\n",
            "NOT D T\n",
            "OR T J\n",
            "OR C J\n",
            "OR G J\n",
            "NOT D T\n",
            "OR C T\n",
            "OR F T\n",
            "AND T J\n",
            "NOT D T\n",
            "OR B T\n",
            "AND T J\n",
            "AND A J\n",
            "NOT J J\n", // Invert J as we computed ~J
            "RUN\n",
        ];

        assert!(springscript.len() <= 15);
        let mut droid = Program::init(&input.string());
        droid
            .input
            .extend(springscript.into_iter().flat_map(|cmd| cmd.bytes().parsed::<i128>()));

        droid.execute();
        // droid.print_as_ascii();
        droid.output.last().unwrap().to_result()
    }
}
