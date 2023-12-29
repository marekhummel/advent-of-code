import sys
from typing import Literal
from time import perf_counter


from solutions.day01 import Solution01
from solutions.day02 import Solution02
from solutions.day03 import Solution03
from solutions.day04 import Solution04
from solutions.day05 import Solution05
from solutions.day06 import Solution06
from solutions.day12 import Solution12
from solutions.day24 import Solution24

ALL: bool = False
VERSION: Literal[1, 2] = 2
USE_SAMPLE: bool = False


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

    arg = sys.argv[1]

    if arg == "main":
        for sc in solutions.values():
            s = sc()
            print(f"Day {s.day:02d}:")
            for version in [1, 2]:
                for sample in [True, False]:
                    try:
                        v = str(s.solve(version=version, use_sample=sample))  # type: ignore
                    except FileNotFoundError:
                        v = "failed"

                    sample_str = "samp" if sample else "real"
                    print(f"  V{version} {sample_str}:  {v}")
    else:
        day = int(arg.removeprefix("day"))

        s = solutions[day]()

        if ALL:
            total = 0.0
            for version in [1, 2]:
                for sample in [True, False]:
                    start = perf_counter()
                    v = str(s.solve(version=version, use_sample=sample))  # type: ignore
                    total += perf_counter() - start
                    sample_str = "samp" if sample else "real"
                    print(f"V{version} {sample_str}:  {v}")

            print(f"\nTotal Runtime: {total}")
        else:
            v = str(s.solve(version=VERSION, use_sample=USE_SAMPLE))
            print(v)
