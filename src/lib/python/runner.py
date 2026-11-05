"""Run one solution and print only its answer."""

import argparse
import sys
from pathlib import Path

from lib.python.solution import ProblemInput, Solution


class Runner:
    """Runner for executing Advent of Code solutions."""

    def __init__(self, year: int, solutions: dict[int, type[Solution]]):
        """
        Creates a new runner for a given year with solution classes.

        Args:
            year: The year of the Advent of Code
            solutions: Dictionary mapping day numbers (1-25) to Solution classes
        """
        self.year = year
        self.solutions = solutions

    def parse_and_run(self) -> None:
        """
        Parse DAY, PART and MODE (real or sample), then print one answer.

        Invalid arguments exit with code 2. Missing solutions or inputs exit
        with code 1 and a message. Unexpected errors retain their traceback.
        """
        parser = argparse.ArgumentParser(description=__doc__)
        parser.add_argument("day", type=int, choices=range(1, 26))
        parser.add_argument("part", type=int, choices=(1, 2))
        parser.add_argument("mode", choices=("real", "sample"))
        args = parser.parse_args()
        sample = args.mode == "sample"
        if args.day not in self.solutions:
            sys.exit("No solution implemented for this day")
        prefix = "sample" if sample else "input"
        filename = Path(f"inputs/{self.year}/{prefix}{args.day:02}.txt")
        if not filename.exists():
            filename = filename.with_stem(f"{filename.stem}_{args.part}")
        try:
            content = filename.read_text(encoding="utf-8").strip()
        except FileNotFoundError:
            sys.exit(f"Missing input: {filename}")
        data = ProblemInput([line.rstrip("\r") for line in content.split("\n")])
        solution = self.solutions[args.day]
        solve = solution.solve_part01 if args.part == 1 else solution.solve_part02
        try:
            result = solve(data, sample)
        except NotImplementedError:
            sys.exit("No solution implemented for this part")
        if result is None:
            sys.exit("No solution implemented for this part")
        print(result)
