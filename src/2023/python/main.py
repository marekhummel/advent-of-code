from lib.python import Runner, Solution
from solutions import day01, day02, day03, day04, day05, day06, day12, day24

# aoc:imports

if __name__ == "__main__":
    solutions = {
        1: Solution.from_module(day01),
        2: Solution.from_module(day02),
        3: Solution.from_module(day03),
        4: Solution.from_module(day04),
        5: Solution.from_module(day05),
        6: Solution.from_module(day06),
        12: Solution.from_module(day12),
        24: Solution.from_module(day24),
        # aoc:solutions
    }

    runner = Runner(2023, solutions=solutions)
    runner.parse_and_run()
