from solution import ProblemInput, Solution
from functools import cache


class Solution12(Solution):
    def __init__(self) -> None:
        super().__init__(12)

    def _solve_version01(self, data: ProblemInput) -> int:
        lines = parse(data)
        return sum(find_arrangements(*args, None) for args in lines)

    def _solve_version02(self, data: ProblemInput) -> int:
        lines = parse2(data)
        return sum(find_arrangements(*args, None) for args in lines)


def parse(data: ProblemInput) -> list[tuple[str, tuple[int, ...]]]:
    lines = []
    for full_line in data:
        line, groups_str = full_line.split(" ")
        groups = tuple(int(x) for x in groups_str.split(","))
        lines.append((line, groups))

    return lines


def parse2(data: ProblemInput) -> list[tuple[str, tuple[int, ...]]]:
    lines = []
    for full_line in data:
        line, groups_str = full_line.split(" ")
        unfolded_line = "?".join([line for _ in range(5)])
        groups = tuple([int(x) for x in groups_str.split(",")] * 5)
        lines.append((unfolded_line, groups))

    return lines


@cache
def find_arrangements(line: str, groups: tuple[int, ...], last_char: str | None) -> int:
    if not line:
        return 1 if len(groups) == 0 or groups == (0,) else 0

    match line[0]:
        case ".":
            if last_char is None or last_char == ".":
                return find_arrangements(line[1:], groups, ".")
            if last_char == "#" and len(groups) > 0:
                return find_arrangements(line[1:], groups[1:], ".") if groups[0] == 0 else 0
        case "#":
            if len(groups) == 0 or groups[0] == 0:
                return 0
            new_groups = tuple(g - (i == 0) for i, g in enumerate(groups))
            return find_arrangements(line[1:], new_groups, "#")
        case "?":
            return find_arrangements(f".{line[1:]}", groups, last_char) + find_arrangements(
                f"#{line[1:]}", groups, last_char
            )

    raise Exception
