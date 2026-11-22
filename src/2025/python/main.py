from lib.python import Runner, Solution
from solutions import day10

# aoc:imports

if __name__ == "__main__":
    solutions: dict[int, Solution] = {
        10: Solution.from_module(day10),
        # aoc:solutions
    }

    runner = Runner(2025, solutions=solutions)
    runner.parse_and_run()
