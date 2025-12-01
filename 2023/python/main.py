import sys
import argparse
from time import perf_counter


from solutions.day01 import Solution01
from solutions.day02 import Solution02
from solutions.day03 import Solution03
from solutions.day04 import Solution04
from solutions.day05 import Solution05
from solutions.day06 import Solution06
from solutions.day12 import Solution12
from solutions.day24 import Solution24


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Advent of Code 2023 - Python Solutions"
    )
    subparsers = parser.add_subparsers(dest="command", help="Command to run")

    # Main command
    subparsers.add_parser(
        "main", help="Run all implemented days (ignores all options)"
    )

    # Day command
    day_parser = subparsers.add_parser("day", help="Run a specific day")
    day_parser.add_argument("day", type=int, help="Day number (1-25)")
    day_parser.add_argument(
        "-s", "--sample", action="store_true", help="Use sample input"
    )
    day_parser.add_argument(
        "-p",
        "--part",
        type=int,
        choices=[1, 2],
        help="Run only part 1 or part 2 (required unless --all)",
    )
    day_parser.add_argument(
        "-a",
        "--all",
        action="store_true",
        help="Run both parts (overrides --part)",
    )

    args = parser.parse_args()

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

    if args.command == "main":
        for sc in solutions.values():
            s = sc()
            print(f"Day {s.day:02d}:")
            for version in [1, 2]:
                for sample in [True, False]:
                    try:
                        v = str(s.solve(version=version, use_sample=sample))
                    except FileNotFoundError:
                        v = "failed"

                    sample_str = "samp" if sample else "real"
                    print(f"  V{version} {sample_str}:  {v}")

    elif args.command == "day":
        day = args.day

        if day not in solutions:
            print(f"Error: No solution implemented for day {day}")
            sys.exit(1)

        if not args.all and args.part is None:
            print("Error: --part <1|2> is required unless --all is given")
            sys.exit(1)

        s = solutions[day]()

        if args.all:
            total = 0.0
            for version in [1, 2]:
                for sample in [True, False]:
                    start = perf_counter()
                    v = str(s.solve(version=version, use_sample=sample))
                    total += perf_counter() - start
                    sample_str = "samp" if sample else "real"
                    print(f"V{version} {sample_str}:  {v}")

            print(f"\nTotal Runtime: {total}")
        else:
            v = str(s.solve(version=args.part, use_sample=args.sample))
            print(v)

    else:
        parser.print_help()
        sys.exit(1)
