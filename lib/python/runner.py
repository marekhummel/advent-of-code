"""Advent of Code runner with CLI parsing."""

import argparse
import os
import sys
import traceback
from typing import Any

from .solution import ProblemInput, ProblemResult, Solution


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

    @staticmethod
    def parse_args(args: list[str] | None = None) -> tuple[str | tuple[str, int], dict[str, Any]]:
        """
        Parses command line arguments and returns configuration.

        Returns:
            Tuple of (command, options_dict) where options contains:
            - all: bool
            - part: int (1 or 2)
            - use_sample: bool
        """
        parser = argparse.ArgumentParser(description="Advent of Code - Python Solutions")
        subparsers = parser.add_subparsers(dest="command", help="Command to run")

        # Main command
        subparsers.add_parser("main", help="Run all implemented days")

        # Day command
        day_parser = subparsers.add_parser("day", help="Run a specific day")
        day_parser.add_argument("day", type=int, help="Day number (1-25)")
        day_parser.add_argument("-s", "--sample", action="store_true", help="Use sample input")
        day_parser.add_argument(
            "-p",
            "--part",
            type=int,
            choices=[1, 2],
            help="Run only part 1 or part 2 (required unless --all)",
        )
        day_parser.add_argument(
            "-a",
            "--all",
            action="store_true",
            help="Run both parts (overrides --part)",
        )

        parsed = parser.parse_args(args)

        if parsed.command == "main":
            return "main", {"all": True, "part": 1, "use_sample": False}

        elif parsed.command == "day":
            day = parsed.day
            if not (1 <= day <= 25):
                print("Error: Day must be between 1 and 25")
                sys.exit(1)

            if not parsed.all and parsed.part is None:
                print("Error: --part <1|2> is required unless --all is given")
                sys.exit(1)

            options = {
                "all": parsed.all if parsed.all else False,
                "part": parsed.part if parsed.part else 1,
                "use_sample": parsed.sample if parsed.sample else False,
            }
            return ("day", day), options

        else:
            parser.print_help()
            sys.exit(1)

    def run(
        self, command: str | tuple[str, int], full_day: bool, part: int, use_sample: bool
    ) -> None:
        """
        Runs the solutions based on parsed configuration.

        Args:
            command: Command string ("main" or "dayXX")
            full_day: Whether to run all parts
            part: Part number (1 or 2)
            use_sample: Whether to use sample input
        """
        if command is None:
            print("Usage: python -m <year>.python.main <command>")
            return

        if command == "main":
            self._run_full_year()
        elif command[0] == "day":
            try:
                day = int(command[1])
                if full_day:
                    self._run_day(day)
                else:
                    self._run_single(day, part, use_sample)
            except ValueError:
                print(f"Invalid command format: {command}")
        else:
            print(f"Unknown command: {command}")

    def verify_solution(self, day: int, part: int, use_sample: bool) -> tuple[bool, str | None]:
        """
        Verifies that a solution matches the expected result.

        Returns:
            Tuple of (success, error_message)
        """
        if day not in self.solutions:
            return False, f"No solution implemented for day {day}"

        solution_class = self.solutions[day]
        expected_index = (part - 1) * 2 + (0 if use_sample else 1)
        expected = solution_class.results()[expected_index]

        try:
            result, _ = self._get_result(day, part, use_sample)
            if result == expected:
                return True, None
            else:
                got = self._format_result(result)
                exp = self._format_result(expected)
                return False, f"Got {got}, expected {exp}"
        except Exception as e:
            return False, f"Error: {str(e)}"

    def verify_solutions(self) -> bool:
        """
        Verifies all solutions against expected results.

        Returns:
            True if all tests pass
        """
        print("\n----------")
        all_success = True

        for day in sorted(self.solutions.keys()):
            for part in [1, 2]:
                for use_sample in [True, False]:
                    sample_label = "s" if use_sample else "r"
                    success, error = self.verify_solution(day, part, use_sample)

                    print(f"Testing D{day:02d} P{part} '{sample_label}': ", end="")

                    if success:
                        print("PASSED")
                    else:
                        print(f"FAILED: {error}")
                        all_success = False

        print("----------")
        return all_success

    def _get_input(self, day: int, part: int, use_sample: bool) -> ProblemInput:
        """Load input file for the problem."""
        base_filename = "sample" if use_sample else "input"
        filename = f"{self.year}/inputs/{base_filename}{day:02d}.txt"

        if not os.path.exists(filename):
            # Try versioned filename
            filename = f"{self.year}/inputs/{base_filename}{day:02d}_{part}.txt"

        if not os.path.exists(filename):
            raise FileNotFoundError(f"Input file not found: {filename}")

        with open(filename, encoding="utf-8") as f:
            content = f.read().strip()  # Remove leading/trailing whitespace
            lines = [line.rstrip("\r") for line in content.split("\n")]
            return ProblemInput(lines)

    def _get_result(self, day: int, part: int, use_sample: bool) -> tuple[ProblemResult, float]:
        """
        Get the result for a specific day/part/sample combination.

        Returns:
            Tuple of (result, duration_in_seconds)
        """
        if day not in self.solutions:
            raise ValueError(f"No solution implemented for day {day}")

        solution_class = self.solutions[day]
        input_data = self._get_input(day, part, use_sample)

        # Use Solution.solve static method
        return Solution.solve(solution_class, input_data, part, use_sample)

    @staticmethod
    def _format_result(result: ProblemResult) -> str:
        """Format a result for display."""
        if result is None:
            return "<No Solution>"
        return str(result)

    def _run_full_year(self) -> None:
        """Run all implemented days."""
        print()
        total_time = 0.0

        for day in sorted(self.solutions.keys()):
            print(f"Day {day:02d}:")
            day_time = 0.0

            for part in [1, 2]:
                for use_sample in [True, False]:
                    try:
                        result, duration = self._get_result(day, part, use_sample)
                        day_time += duration
                        sample_str = "samp" if use_sample else "real"
                        print(f"  P{part} {sample_str}:  {self._format_result(result)}")
                    except FileNotFoundError:
                        sample_str = "samp" if use_sample else "real"
                        print(f"  P{part} {sample_str}:  <Error>")

            print(f"  > Runtime: {day_time:.4f}s\n")
            total_time += day_time

        print(f"\n\nTotal Runtime: {total_time:.4f}s")

    def _run_day(self, day: int) -> None:
        """Run all parts of a specific day."""
        if day not in self.solutions:
            print(f"No solution implemented for day {day:02d} in year {self.year}")
            return

        solution_class = self.solutions[day]
        expected_results = solution_class.results()

        day_elapsed = 0.0
        matches_expected = True

        for part in [1, 2]:
            for use_sample in [True, False]:
                sample_str = "samp" if use_sample else "real"

                try:
                    result, duration = self._get_result(day, part, use_sample)
                    day_elapsed += duration

                    sample_idx = 0 if use_sample else 1
                    expected_idx = (part - 1) * 2 + sample_idx
                    expected = expected_results[expected_idx]
                    matches_expected = matches_expected and (result == expected)

                    formatted = self._format_result(result)
                    print(f"P{part} {sample_str} in {duration:10.4f}s:    {formatted}")
                except Exception as e:
                    print(f"P{part} {sample_str}:  <Error: {str(e)}>")

        print(f"\nTotal Runtime: {day_elapsed:.4f}s")
        match_str = "match" if matches_expected else "don't match"
        print(f"Note: Results {match_str} expected")

    def _run_single(self, day: int, part: int, use_sample: bool) -> None:
        """Run a single part of a specific day."""
        try:
            result, elapsed = self._get_result(day, part, use_sample)
            sample_str = "samp" if use_sample else "real"

            print(f"Day {day:02d} / part {part} / Data '{sample_str}' => {elapsed:.4f}s")
            print(self._format_result(result))
        except Exception:
            traceback.print_exc()
