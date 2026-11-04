from lib.python import Runner, Solution
from solutions.day10 import Solution10

# aoc:imports

if __name__ == "__main__":
    solutions: dict[int, type[Solution]] = {
        10: Solution10,
        # aoc:solutions
    }

    runner = Runner(2025, solutions=solutions)
    runner.parse_and_run()
