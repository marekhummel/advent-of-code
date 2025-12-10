from lib.python import Runner

from .solutions.day01 import Solution01
from .solutions.day02 import Solution02
from .solutions.day03 import Solution03
from .solutions.day04 import Solution04
from .solutions.day05 import Solution05
from .solutions.day06 import Solution06
from .solutions.day12 import Solution12
from .solutions.day24 import Solution24

if __name__ == "__main__":
    solutions = {
        1: Solution01,
        2: Solution02,
        3: Solution03,
        4: Solution04,
        5: Solution05,
        6: Solution06,
        12: Solution12,
        24: Solution24,
    }

    runner = Runner(2023, solutions=solutions)
    command, options = Runner.parse_args()
    runner.run(command, options["all"], options["part"], options["use_sample"])
