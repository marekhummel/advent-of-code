"""Function registration and types for Advent of Code solutions."""

from collections.abc import Callable
from dataclasses import dataclass
from enum import Enum
from typing import NewType, Protocol


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


Part = Callable[[ProblemInput, bool], ProblemResult]


class DayModule(Protocol):
    """A day module exposing both correctly typed part functions."""

    def part1(self, data: ProblemInput, is_sample: bool, /) -> ProblemResult: ...

    def part2(self, data: ProblemInput, is_sample: bool, /) -> ProblemResult: ...


@dataclass(frozen=True)
class Solution:
    """The two required callable parts of a day."""

    part1: Part
    part2: Part

    @classmethod
    def from_module(cls, day: DayModule) -> "Solution":
        """Bind both parts from a module satisfying the day contract."""
        return cls(part1=day.part1, part2=day.part2)
