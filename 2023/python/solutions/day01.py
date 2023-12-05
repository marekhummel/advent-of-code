from solution import ProblemInput, Solution


class Solution01(Solution):
    def __init__(self) -> None:
        super().__init__(1)

    def _solve_version01(self, data: ProblemInput) -> int:
        return sum(value01(line) for line in data)

    def _solve_version02(self, data: ProblemInput) -> int:
        return sum(value02(line) for line in data)


def value01(s: str) -> int:
    first = next(c for c in s if c in "0123456789")
    last = next(c for c in reversed(s) if c in "0123456789")
    return int(first + last)


digits = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", *"0123456789"]


def value02(s: str) -> int:
    first = min([(p, i % 10) for i, d in enumerate(digits) if (p := s.find(d)) >= 0], key=lambda tpl: tpl[0])[1]
    last = max([(p, i % 10) for i, d in enumerate(digits) if (p := s.rfind(d)) >= 0], key=lambda tpl: tpl[0])[1]
    return first * 10 + last
