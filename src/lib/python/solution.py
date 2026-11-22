"""Base class and types for Advent of Code solutions."""

from enum import Enum
from typing import NewType


# Valid results for cases that do not have an answer.
class Status(Enum):
    """Informational outcomes, distinct from text answers."""

    NO_SAMPLE = "<No Sample Defined>"
    NO_PART_TWO = "<No Part Two>"
    NOT_IMPLEMENTED = "<Not Implemented>"
    UNSOLVED = "<Unsolved>"

    def __str__(self) -> str:
        return self.value


# Type aliases
ProblemInput = NewType("ProblemInput", list[str])
ProblemResult = int | str | Status


class Solution:
    """
    Base class for Advent of Code solutions.

    Subclasses should implement:
    - solve_part01(input, is_sample): Solve part 1
    - solve_part02(input, is_sample): Solve part 2
    """

    @staticmethod
    def solve_part01(data: ProblemInput, is_sample: bool) -> ProblemResult:
        """Solve part 1 of the problem."""
        raise NotImplementedError()

    @staticmethod
    def solve_part02(data: ProblemInput, is_sample: bool) -> ProblemResult:
        """Solve part 2 of the problem."""
        raise NotImplementedError()
