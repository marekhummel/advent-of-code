import os
from typing import Literal, NewType

ProblemInput = NewType("ProblemInput", list[str])


class Solution:
    day: int

    def __init__(self, day: int) -> None:
        self.day = day

    def solve(self, *, version: Literal[1, 2], use_sample: bool = False) -> int:
        data = self._get_input(version, use_sample)
        if version == 1:
            return self._solve_version01(data)
        if version == 2:
            return self._solve_version02(data)

        raise ValueError("invalid version")

    def _get_input(self, version: Literal[1, 2], use_sample: bool = False) -> ProblemInput:
        base_filename = "sample" if use_sample else "input"
        fullname = rf"2023\inputs\{base_filename}{self.day:02d}.txt"
        if not os.path.exists(fullname):
            fullname = fullname.replace(".txt", f"_{version}.txt")

        with open(fullname, "r", encoding="utf-8") as f:
            return ProblemInput(f.readlines())

    def _solve_version01(self, data: ProblemInput) -> int:
        raise NotImplementedError()

    def _solve_version02(self, data: ProblemInput) -> int:
        raise NotImplementedError()
