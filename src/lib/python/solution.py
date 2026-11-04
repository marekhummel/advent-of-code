"""Base class and types for Advent of Code solutions."""

from time import perf_counter
from typing import NewType

# Type aliases
ProblemInput = NewType("ProblemInput", list[str])
ProblemResult = int | float | str | None


class Solution:
    """
    Base class for Advent of Code solutions.

    Subclasses should implement:
    - results(): Return [part1_sample, part1_real, part2_sample, part2_real]
    - solve_part01(input, is_sample): Solve part 1
    - solve_part02(input, is_sample): Solve part 2
    """

    @staticmethod
    def results() -> list[ProblemResult]:
        """
        Returns the expected results for testing.

        Returns:
            [part1_sample, part1_real, part2_sample, part2_real]
        """
        raise NotImplementedError()

    @staticmethod
    def solve_part01(data: ProblemInput, is_sample: bool) -> ProblemResult:
        """Solve part 1 of the problem."""
        raise NotImplementedError()

    @staticmethod
    def solve_part02(data: ProblemInput, is_sample: bool) -> ProblemResult:
        """Solve part 2 of the problem."""
        raise NotImplementedError()

    @staticmethod
    def solve(
        solution_class: type["Solution"], input_data: ProblemInput, part: int, is_sample: bool
    ) -> tuple[ProblemResult, float]:
        """
        Solves the problem based on the part number.
        Returns a tuple of (result, duration_in_seconds).

        Args:
            solution_class: The solution class to use
            input_data: The input data
            part: Part number (1 or 2)
            is_sample: Whether using sample input

        Returns:
            Tuple of (result, duration_in_seconds)
        """
        start = perf_counter()

        if part == 1:
            result = solution_class.solve_part01(input_data, is_sample)
        elif part == 2:
            result = solution_class.solve_part02(input_data, is_sample)
        else:
            raise ValueError(f"Invalid part: {part}")

        duration = perf_counter() - start
        return result, duration
