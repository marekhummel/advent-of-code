from functools import reduce
from solution import ProblemInput, Solution
from math import ceil, floor, sqrt


class Solution06(Solution):
    def __init__(self) -> None:
        super().__init__(6)

    def _solve_version01(self, data: ProblemInput) -> int:
        race_winners = [solve_race(r) for r in parse(data)]
        return reduce(lambda acc, x: acc * x, race_winners, 1)

    def _solve_version02(self, data: ProblemInput) -> int:
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

    low = low + 1 if low.is_integer() else low
    high = high - 1 if high.is_integer() else high

    return floor(high) - ceil(low) + 1
