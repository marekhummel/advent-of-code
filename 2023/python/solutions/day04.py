from solution import ProblemInput, Solution


class Solution04(Solution):
    def __init__(self) -> None:
        super().__init__(4)

    def _solve_version01(self, data: ProblemInput) -> int:
        return sum(compute_value(winning, mine) for winning, mine in parse(data))

    def _solve_version02(self, data: ProblemInput) -> int:
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
