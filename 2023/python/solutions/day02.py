from functools import reduce
from itertools import starmap

from lib.python import ProblemInput, ProblemResult, Solution


class Solution02(Solution):
    @staticmethod
    def results() -> list[ProblemResult]:
        return [8, 2476, 2286, 54911]

    @staticmethod
    def solve_part01(data: ProblemInput, is_sample: bool) -> ProblemResult:
        return sum(g for g, reveals in parse(data) if valid(reveals, 12, 13, 14))

    @staticmethod
    def solve_part02(data: ProblemInput, is_sample: bool) -> ProblemResult:
        return sum(power(reveals) for _, reveals in parse(data))


def parse(lines: list[str]) -> list:
    record = []
    for line in lines:
        game, reveals_str = line.split(":")
        game_id = int(game.split()[1])
        reveal_lists = reveals_str.split(";")
        reveals = [r.split() for rl in reveal_lists for r in rl.split(",")]
        reveals_typed = list(starmap(lambda n, c: (int(n), c), reveals))

        record.append((game_id, reveals_typed))

    return record


def valid(reveals, reds, greens, blues):
    for n, color in reveals:
        limit = [reds, greens, blues][["red", "green", "blue"].index(color)]
        if n > limit:
            return False

    return True


def power(reveals):
    setsize = {"red": 0, "green": 0, "blue": 0}
    for n, color in reveals:
        setsize[color] = max(n, setsize[color])

    return reduce(lambda acc, x: acc * x, setsize.values(), 1)
