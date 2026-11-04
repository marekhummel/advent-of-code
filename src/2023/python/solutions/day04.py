from lib.python import ProblemInput, ProblemResult, Solution


class Solution04(Solution):
    @staticmethod
    def results() -> list[ProblemResult]:
        return [13, 26443, 30, 6284877]

    @staticmethod
    def solve_part01(data: ProblemInput, is_sample: bool) -> ProblemResult:
        return sum(compute_value(winning, mine) for winning, mine in parse(data))

    @staticmethod
    def solve_part02(data: ProblemInput, is_sample: bool) -> ProblemResult:
        copies = [1] * len(data)
        for i, (winning, mine) in enumerate(parse(data)):
            value = count_hits(winning, mine)
            for j in range(i + 1, i + 1 + value):
                copies[j] += copies[i]

        return sum(copies)


def parse(data: ProblemInput) -> list[tuple[set[int], set[int]]]:
    cards = []
    for line in data:
        left_str, right_str = line.split(":")[1].split("|")
        left = {int(n) for n in left_str.strip().split()}
        right = {int(n) for n in right_str.strip().split()}
        cards.append((left, right))
    return cards


def compute_value(winning: set[int], mine: set[int]) -> int:
    h = count_hits(winning, mine)
    return 1 << (h - 1) if h else 0


def count_hits(winning: set[int], mine: set[int]) -> int:
    return len(winning.intersection(mine))
