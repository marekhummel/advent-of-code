import sys
from typing import Literal

from solutions.day01 import Solution01
from solutions.day02 import Solution02
from solutions.day03 import Solution03
from solutions.day04 import Solution04
from solutions.day05 import Solution05
from solutions.day06 import Solution06
from solutions.day07 import Solution07
from solutions.day08 import Solution08
from solutions.day09 import Solution09
from solutions.day10 import Solution10
from solutions.day11 import Solution11
from solutions.day12 import Solution12
from solutions.day13 import Solution13
from solutions.day14 import Solution14
from solutions.day15 import Solution15
from solutions.day16 import Solution16
from solutions.day17 import Solution17
from solutions.day18 import Solution18
from solutions.day19 import Solution19
from solutions.day20 import Solution20
from solutions.day21 import Solution21
from solutions.day22 import Solution22
from solutions.day23 import Solution23
from solutions.day24 import Solution24

VERSION: Literal[1, 2] = 1
USE_SAMPLE = False


if __name__ == "__main__":
    day = int(sys.argv[1].removeprefix("day"))
    solutions = [
        Solution01,
        Solution02,
        Solution03,
        Solution04,
        Solution05,
        Solution06,
        Solution07,
        Solution08,
        Solution09,
        Solution10,
        Solution11,
        Solution12,
        Solution13,
        Solution14,
        Solution15,
        Solution16,
        Solution17,
        Solution18,
        Solution19,
        Solution20,
        Solution21,
        Solution22,
        Solution23,
        Solution24,
    ]

    s = solutions[day - 1]()
    v = s.solve(version=VERSION, use_sample=USE_SAMPLE)
    print(v)
