from functools import reduce
from math import ceil, floor, sqrt

from lib.python import ProblemInput, ProblemResult, Solution


class Solution06(Solution):
    @staticmethod
    def results() -> list[ProblemResult]:
        return [288, 293046, 71503, 35150181]

    @staticmethod
    def solve_part01(data: ProblemInput, is_sample: bool) -> ProblemResult:
        race_winners = [solve_race(r) for r in parse(data)]
        return reduce(lambda acc, x: acc * x, race_winners, 1)

    @staticmethod
    def solve_part02(data: ProblemInput, is_sample: bool) -> ProblemResult:
        return solve_race(parse2(data))


def parse(lines: ProblemInput) -> list[tuple[int, int]]:
    times = [int(t) for t in lines[0].removeprefix("Time:").strip().split()]
    dists = [int(t) for t in lines[1].removeprefix("Distance:").strip().split()]

    return list(zip(times, dists))


def parse2(lines: ProblemInput) -> tuple[int, int]:
    time = "".join(lines[0].removeprefix("Time:").strip().split())
    dist = "".join(lines[1].removeprefix("Distance:").strip().split())

    return int(time), int(dist)


def solve_race(race: tuple[int, int]) -> int:
    time, record = race
    root = sqrt(time * time - 4 * record)
    low, high = 0.5 * (time - root), 0.5 * (time + root)

    return ceil(high) - floor(low) - 1
