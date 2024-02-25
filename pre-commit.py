# Call this script from hook. Can't use this as hook directly,
# because windows only spawns bash instances for the scripts and ignores shebangs.
#
# #!/bin/bash
# python pre-commit.py

import subprocess
import re

# Get git status
proc = subprocess.run(
    ["git", "status", "-s", "-uall"],
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    shell=True,
)
files = proc.stdout.decode("utf-8")

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
print(f"Testing {total_tests} day(s) across {len(changed_days.keys())} year(s)...")
for year, days in changed_days.items():
    for day in days:
        proc = subprocess.run(
            ["cargo", "test", "--release", "--bin", f"main{year}", day],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            shell=True,
        )
        print(f"Year {year}, Day {day.strip('day')} -> {proc.returncode}")
        if proc.returncode != 0:
            exit(proc.returncode)
