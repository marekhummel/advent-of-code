"""Run solutions and compare their output with sample or accepted answers."""

import argparse
import os
import signal
import subprocess
import sys
import time
from collections import Counter
from pathlib import Path
from urllib.error import URLError

from answers import get_answer  # type: ignore

ENTRY_POINTS = {
    "rust": "main.rs",
    "python": "main.py",
    "golang": "main.go",
    "zig": "main.zig",
    "elixir": "main.ex",
    "cpp": "main.cpp",
    "lua": "main.lua",
    "dart": "main.dart",
}


def execute(command: list[str]) -> subprocess.CompletedProcess[str]:
    """Keep the task and its children in a group that can be cancelled together."""
    with subprocess.Popen(
        command,
        stdin=subprocess.DEVNULL,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        start_new_session=True,
    ) as process:
        try:
            stdout, stderr = process.communicate()
        except BaseException:
            if process.poll() is None:
                try:
                    os.killpg(process.pid, signal.SIGTERM)
                    process.communicate(timeout=5)
                except subprocess.TimeoutExpired:
                    os.killpg(process.pid, signal.SIGKILL)
                    process.communicate()
                except ProcessLookupError:
                    process.communicate()
            raise
    return subprocess.CompletedProcess(command, process.returncode, stdout or "", stderr or "")


def run_case(
    language: str, year: int, day: int, part: int, mode: str, optimize: str
) -> tuple[str, str, float]:
    start = time.perf_counter()
    result = execute(
        [
            "task",
            "--silent",
            "--exit-code",
            f"case:{language}",
            f"YEAR={year}",
            f"DAY={day:02}",
            f"PART={part}",
            f"MODE={mode}",
            f"OPTIMIZE={optimize}",
        ]
    )
    elapsed = time.perf_counter() - start
    actual = result.stdout.removesuffix("\n")
    if result.returncode in (130, 143):
        raise SystemExit(result.returncode)
    if result.returncode:
        detail = f"runner exited with {result.returncode}"
        if actual:
            detail += f"; got: {actual!r}"
        return "FAILED", detail + (f"\n{result.stderr.rstrip()}" if result.stderr else ""), elapsed
    if actual in ("<Not Implemented>", "<Unsolved>"):
        status, detail = "NOT SOLVED", actual
    else:
        try:
            expected = get_answer(year, day, part, mode)
        except (FileNotFoundError, URLError) as error:
            status, detail = "UNKNOWN", f"{actual}; reference unavailable: {error}"
        except (ValueError, TypeError, OSError) as error:
            status, detail = "FAILED", f"cannot read reference: {error}; got: {actual!r}"
        else:
            if expected is None:
                status, detail = "UNKNOWN", f"{actual}; no reference answer (not solved)"
            elif actual == expected:
                status, detail = "PASS", actual
            else:
                status, detail = "FAILED", f"expected: {expected!r}, got: {actual!r}"
    if result.stderr:
        detail += f"\n{result.stderr.rstrip()}"
    return status, detail, elapsed


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lang", choices=ENTRY_POINTS)
    parser.add_argument("--year", type=int)
    parser.add_argument("--day", type=int)
    parser.add_argument("--part", type=int, choices=(1, 2))
    parser.add_argument("--mode", choices=("real", "sample", "both"), default="both")
    parser.add_argument("--optimize", choices=("debug", "fast"), default="fast")
    parser.add_argument(
        "--failures-only", action="store_true", help="Show failures and totals only"
    )
    args = parser.parse_args()
    os.chdir(Path(__file__).resolve().parents[1])
    projects = [
        (int(directory.name), language, directory / language)
        for directory in sorted(Path("src").glob("[0-9][0-9][0-9][0-9]"))
        if args.year is None or int(directory.name) == args.year
        for language, entry in ENTRY_POINTS.items()
        if (args.lang is None or language == args.lang)
        and (directory / language / entry).is_file()
    ]
    if not projects:
        parser.error("No matching year/language entry points found")
    totals: Counter[str] = Counter()
    modes = ("sample", "real") if args.mode == "both" else (args.mode,)
    parts = (args.part,) if args.part is not None else (1, 2)
    for year, language, project in projects:
        last_day = 25 if year < 2025 else 12
        days = [args.day] if args.day is not None else list(range(1, last_day + 1))
        if args.day is None:
            suffix = ".hpp" if language == "cpp" else Path(ENTRY_POINTS[language]).suffix
            days = [
                day
                for day in days
                if (
                    project
                    / "solutions"
                    / (
                        f"day{day:02}/solution.go"
                        if language == "golang"
                        else f"day{day:02}{suffix}"
                    )
                ).is_file()
            ]
        days = [day for day in days if 1 <= day <= last_day]
        prefix = f"[{language:6}] {year} " if len(projects) > 1 else ""
        for day in days:
            for part in parts:
                if day == last_day and part == 2 and args.part is None:
                    continue
                for mode in modes:
                    status, detail, elapsed = run_case(
                        language, year, day, part, mode, args.optimize
                    )
                    totals[status] += 1
                    if not args.failures_only or status == "FAILED":
                        label = "SAMP" if mode == "sample" else "REAL"
                        status_label = f"[{status}]:"
                        print(
                            f"{prefix}D{day:02} P{part} {label} ({elapsed:9.5f} secs) "
                            f"{status_label:13}  {detail}",
                            flush=True,
                        )
    print(
        f"VERIFIED: {totals['PASS']} passed, {totals['FAILED']} failed, "
        f"{totals['NOT SOLVED']} not solved, {totals['UNKNOWN']} unknown",
        flush=True,
    )
    return int(totals["FAILED"] > 0)


if __name__ == "__main__":
    signal.signal(signal.SIGTERM, lambda signum, _frame: sys.exit(128 + signum))
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        sys.exit(130)
    except (ValueError, TypeError, OSError, URLError) as error:
        sys.exit(f"Error: {error}")
