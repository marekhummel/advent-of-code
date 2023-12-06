from solution import ProblemInput, Solution

Number = tuple[int, int, int, int]
Symbol = tuple[str, int, int]


class Solution03(Solution):
    def __init__(self) -> None:
        super().__init__(3)

    def _solve_version01(self, data: ProblemInput) -> int:
        numbers, symbols = parse(data)
        return sum(n[0] for n in numbers if is_part(n, symbols))

    def _solve_version02(self, data: ProblemInput) -> int:
        numbers, symbols = parse(data)
        return sum(gear_ratio(s, numbers) for s in symbols)


def parse(lines: list[str]) -> tuple[list[Number], list[Symbol]]:
    numbers = []
    symbols = []
    for i, line in enumerate(lines):
        number = ""
        number_start = -1
        for j, char in enumerate(line):
            if char in "0123456789":
                if number == "":
                    number_start = j
                number += char
            else:
                if number != "":
                    numbers.append((int(number), i, number_start, j - 1))
                    number = ""

                if char != "." and char != "\n":
                    symbols.append((char, i, j))

    return numbers, symbols


def is_part(number: Number, symbols: list[Symbol]) -> bool:
    _, num_row, left, right = number

    for sym, sym_row, sym_col in symbols:
        if sym_row == num_row and (sym_col == left - 1 or sym_col == right + 1):
            return True
        if (sym_row == num_row - 1 or sym_row == num_row + 1) and left - 1 <= sym_col <= right + 1:
            return True

    return False


def gear_ratio(symbol: Symbol, numbers: list[Number]) -> int:
    char, sym_row, sym_col = symbol

    if char != "*":
        return 0

    part_numbers = []
    for num, num_row, num_col_start, num_col_end in numbers:
        if sym_row == num_row and (sym_col == num_col_start - 1 or sym_col == num_col_end + 1):
            part_numbers.append(num)
            continue
        if (sym_row == num_row - 1 or sym_row == num_row + 1) and num_col_start - 1 <= sym_col <= num_col_end + 1:
            part_numbers.append(num)
            continue

    if len(part_numbers) != 2:
        return 0

    return part_numbers[0] * part_numbers[1]
