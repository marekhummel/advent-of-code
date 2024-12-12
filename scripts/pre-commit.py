# Call this script from hook. Can't use this as hook directly,
# because windows only spawns bash instances for the scripts and ignores shebangs.
#
# #!/bin/bash
# python pre-commit.py

import subprocess
import re


def test_rust(files: str) -> None:
    # Find changed days
    changed_days: dict[str, list[str]] = {}
    rgx = re.compile(r"(\d{4})/rust/solutions/(day\d{2})\.rs")
    for file in files.split("\n"):
        if captures := rgx.search(file):
            year = captures.group(1)
            day = captures.group(2)
            if year not in changed_days:
                changed_days[year] = []
            changed_days[year].append(day)

    # Test them
    total_tests = sum(1 for days in changed_days.values() for d in days)
    print(f"RUST: Testing {total_tests} day(s) across {len(changed_days.keys())} year(s)...")
    for year, days in changed_days.items():
        for day in days:
            proc = subprocess.run(
                ["cargo", "test", "--release", "--bin", f"main{year}", day],
                stdout=subprocess.PIPE,
                stderr=subprocess.DEVNULL,
                shell=True,
            )
            print(f"  Year {year}, Day {day.strip('day')} -> {proc.returncode}")
            if proc.returncode != 0:
                print(proc.stdout.decode("utf-8"))
                exit(proc.returncode)


def test_zig(files: str) -> None:
    changed_years: set[set] = set()
    rgx = re.compile(r"(\d{4})/zig/solutions/(day\d{2})\.zig")
    for file in files.split("\n"):
        if captures := rgx.search(file):
            year = captures.group(1)
            # day = captures.group(2)
            changed_years.add(year)

    print(f"ZIG : Testing {len(changed_years)} year(s)...")
    for year in changed_years:
        proc = subprocess.run(
            ["zig", "build", "-Doptimize=ReleaseFast", f"{year}_test"],
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            shell=True,
        )
        print(f"  Year {year} -> {proc.returncode}")
        if proc.returncode != 0:
            print(proc.stdout.decode("utf-8"))
            exit(proc.returncode)

if __name__ == "__main__":
    # Get git status
    proc = subprocess.run(
        ["git", "status", "-s", "-uall"],
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        shell=True,
    )
    files = proc.stdout.decode("utf-8")

    test_rust(files)
    test_zig(files)
