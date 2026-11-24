"""Verify complete days against sample data and accepted Advent of Code answers."""

import argparse
import os
import subprocess
import sys
from pathlib import Path
from urllib.error import URLError

from answers import get_answer # type: ignore


def case_label(lang: str, year: int, day: int, part: int, mode: str) -> str:
    lang_str = f"<{lang}>"
    return f"{lang_str:9} on ({year} {day:02} {part}) with {mode:6}"


def verify_case(
    lang: str,
    year: int,
    day: int,
    part: int,
    mode: str,
    verbose: bool = False,
) -> int:
    label = case_label(lang, year, day, part, mode)
    command = [
        "task",
        "--silent",
        "--exit-code",
        f"case:{lang}",
        f"YEAR={year}",
        f"DAY={day}",
        f"PART={part}",
        f"MODE={mode}",
        "OPTIMIZE=fast",
    ]
    result = subprocess.run(command, stdout=subprocess.PIPE, check=False)
    if result.returncode:
        print(f"[FAILED]    {label}: runner exited with {result.returncode}", file=sys.stderr)
        return 1
    actual = result.stdout.decode("utf-8").removesuffix("\n")
    if actual in ("<Not Implemented>", "<Unsolved>"):
        if verbose:
            print(f"[SKIPPED]  {label}:  {actual}")
        return 0
    expected = get_answer(year, day, part, mode)
    if expected is None:
        print(
            f"[UNKNOWN]  {label}:  {actual}; "
            f"no {'sample' if mode == 'sample' else 'accepted'} answer",
            file=sys.stderr,
        )
        return 1
    if actual != expected:
        print(
            f"[FAILED]   {label}:  expected: {expected!r}, got: {actual!r}",
            file=sys.stderr,
        )
        return 1
    if verbose:
        print(f"[PASSED]   {label}:  {actual}")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    entry_points = {
        "rust": "main.rs",
        "python": "main.py",
        "golang": "main.go",
        "zig": "main.zig",
        "elixir": "main.ex",
        "cpp": "main.cpp",
        "lua": "main.lua",
        "dart": "main.dart",
    }
    parser.add_argument("--lang", choices=entry_points)
    parser.add_argument("--year", type=int)
    parser.add_argument("--day", type=int, choices=range(1, 26))
    parser.add_argument(
        "--verbose", action="store_true", help="Also show passing and skipped results"
    )
    args = parser.parse_args()
    os.chdir(Path(__file__).resolve().parents[1])
    failed = 0
    summaries: list[tuple[int, str, int]] = []
    matched = False
    for year_dir in sorted(Path("src").glob("[0-9][0-9][0-9][0-9]")):
        year = int(year_dir.name)
        if args.year is not None and year != args.year:
            continue
        for lang, entry_point in entry_points.items():
            if args.lang is not None and lang != args.lang:
                continue
            if not (year_dir / lang / entry_point).is_file():
                continue
            matched = True
            group_failed = 0
            last_day = 25 if year < 2025 else 12
            days = [args.day] if args.day is not None else list(range(1, last_day + 1))
            for day in days:
                filename = (
                    f"day{day:02}/solution.go"
                    if lang == "golang"
                    else f"day{day:02}{'.hpp' if lang == 'cpp' else Path(entry_point).suffix}"
                )
                if not (year_dir / lang / "solutions" / filename).is_file():
                    continue
                for part in (1, 2):
                    if day == last_day and part == 2:
                        continue
                    for mode in ("sample", "real"):
                        try:
                            group_failed += verify_case(
                                lang,
                                year,
                                day,
                                part,
                                mode,
                                args.verbose,
                            )
                        except (ValueError, TypeError, OSError, URLError) as error:
                            label = case_label(lang, year, day, part, mode)
                            print(
                                f"{'FAILED':7}:  {label}: {error}",
                                file=sys.stderr,
                            )
                            group_failed += 1
            summaries.append((year, lang, group_failed))
            failed += group_failed
    if not matched:
        parser.error("No matching year/language entry points found")
    for year, lang, group_failed in summaries:
        print(f"VERIFIED: {year} {lang:<10}:  {group_failed} failed")
    print(f"VERIFIED: {failed} failed", flush=True)
    return int(failed > 0)


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, TypeError, OSError, URLError) as error:
        sys.exit(f"Error: {error}")
