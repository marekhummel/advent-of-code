import re
from concurrent.futures import ThreadPoolExecutor

import pulp

from lib.python import ProblemInput, ProblemResult, Solution

type Joltages = list[int]
type Buttons = list[list[int]]
type Machine = tuple[int, Buttons, Joltages]


class Solution10(Solution):
    @staticmethod
    def results() -> list[ProblemResult]:
        return [None, None, 33, 16361]

    @staticmethod
    def solve_part01(data: ProblemInput, is_sample: bool) -> ProblemResult:
        return None

    @staticmethod
    def solve_part02(data: ProblemInput, is_sample: bool) -> ProblemResult:
        machines = parse(data)
        with ThreadPoolExecutor() as executor:
            results = executor.map(joltage_config, machines)
            return sum(results)


def parse(data: ProblemInput) -> list[Machine]:
    parsed = []
    for i, line in enumerate(data):
        button_matches = re.findall(r"\(([^)]+)\)", line)
        joltages_match = re.search(r"\{([^}]+)\}", line)

        # Buttons in ()
        buttons = []
        for button_str in button_matches:
            lights = [int(x) for x in button_str.split(",")]
            buttons.append(lights)

        # Joltages in {}
        joltages = [int(x) for x in joltages_match.group(1).split(",")] if joltages_match else []

        parsed.append((i, buttons, joltages))

    return parsed


def joltage_config(machine: Machine) -> int:
    """Solve via ILP. Find mininum of sum(x) subject to A*x = j"""

    # Setup
    ident, buttons, joltages = machine
    prob = pulp.LpProblem(f"AOC2025_DAY10_{ident}", pulp.LpMinimize)

    # Decision variables: x[i] = number of times button i is pressed
    x = [pulp.LpVariable(f"x{i}", lowBound=0, cat="Integer") for i in range(len(buttons))]

    # Objective: minimize sum of x
    prob += pulp.lpSum(x)

    # Constraints: A*x = j
    # For each wire, sum of button presses affecting it must equal the target joltage
    for j, trg_joltage in enumerate(joltages):
        ones = (x[i] for i, btn_wires in enumerate(buttons) if j in btn_wires)
        prob += pulp.lpSum(ones) == trg_joltage

    # Solve
    prob.solve(pulp.PULP_CBC_CMD(msg=False))
    assert pulp.LpStatus[prob.status] == "Optimal"

    # Return the sum of button presses (== objective value)
    obj_value = pulp.value(prob.objective)
    assert isinstance(obj_value, float)
    return int(obj_value)
