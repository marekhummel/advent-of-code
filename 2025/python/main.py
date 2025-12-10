from lib.python import Runner, Solution

from .solutions.day10 import Solution10

if __name__ == "__main__":
    solutions: dict[int, type[Solution]] = {
        10: Solution10,
    }

    runner = Runner(2025, solutions=solutions)
    command, options = Runner.parse_args()
    runner.run(command, options["all"], options["part"], options["use_sample"])
