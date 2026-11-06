"""Verify complete days against sample data and accepted Advent of Code answers."""

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
from html.parser import HTMLParser
from pathlib import Path
from urllib.error import URLError
from urllib.request import Request, urlopen

NO_SAMPLE = "<No Sample Defined>"


class PuzzlePage(HTMLParser):
    """Extract accepted answers from the authenticated puzzle page."""

    def __init__(self) -> None:
        super().__init__()
        self.answers: list[str] = []
        self.logged_out = False
        self.is_puzzle = False
        self.paragraph: list[str] | None = None
        self.code: list[str] | None = None
        self.in_code = False

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        if tag == "article" and "day-desc" in (dict(attrs).get("class") or "").split():
            self.is_puzzle = True
        if tag == "p":
            self.paragraph = []
            self.code = None
        elif tag == "code" and self.paragraph is not None:
            self.code = []
            self.in_code = True

    def handle_endtag(self, tag: str) -> None:
        if tag == "code":
            self.in_code = False
        elif tag == "p" and self.paragraph is not None:
            if "".join(self.paragraph).strip().startswith("Your puzzle answer was"):
                if self.code is not None:
                    self.answers.append("".join(self.code))
            self.paragraph = None

    def handle_data(self, data: str) -> None:
        if data.strip() == "[Log In]":
            self.logged_out = True
        if self.paragraph is not None:
            self.paragraph.append(data)
        if self.in_code and self.code is not None:
            self.code.append(data)


def sample_answer(year: int, day: int, part: int) -> str | None:
    path = Path(f"inputs/{year}/sample_solutions.json")
    data = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise TypeError(f"Expected an object in {path}")
    parts = data.get(f"{day:02}", {})
    if not isinstance(parts, dict):
        raise TypeError(f"Expected a part/answer object for day {day:02} in {path}")
    if str(part) not in parts:
        return None
    answer = parts.get(str(part))
    if answer is not None and not isinstance(answer, str):
        raise TypeError(f"Expected a string or null for day {day:02} part {part} in {path}")
    return NO_SAMPLE if answer is None else answer


def remote_answer(year: int, day: int, part: int) -> str | None:
    session = Path("scripts/aoc_cookie.txt").read_text().strip()
    if not re.fullmatch(r"[a-zA-Z0-9_-]+", session):
        raise ValueError("Invalid session credential in scripts/aoc_cookie.txt")
    # Separate accounts without writing the session token into the cache.
    account = hashlib.sha256(session.encode()).hexdigest()
    cache = Path(f"build/answers/{account}/{year}/day{day:02}.json")
    if cache.exists():
        cached = json.loads(cache.read_text(encoding="utf-8"))
        if not isinstance(cached, dict) or any(
            key not in ("1", "2") or not isinstance(value, str) for key, value in cached.items()
        ):
            raise ValueError(f"Invalid answer cache: {cache}")
        if str(part) in cached:
            return cached[str(part)]

    request = Request(
        f"https://adventofcode.com/{year}/day/{day}",
        headers={
            "Cookie": f"session={session}",
            "User-Agent": "advent-of-code-library/1.0 (local answer verification)",
        },
    )
    with urlopen(request, timeout=30) as response:
        page = response.read().decode("utf-8")
    parser = PuzzlePage()
    parser.feed(page)
    parser.close()
    if parser.logged_out:
        raise ValueError("Advent of Code rejected the session; refresh scripts/aoc_cookie.txt")
    if not parser.is_puzzle:
        raise ValueError("Advent of Code did not return a puzzle page")
    if len(parser.answers) > 2:
        raise ValueError("Unexpected accepted-answer format on the puzzle page")
    accepted = {str(index): answer for index, answer in enumerate(parser.answers, 1)}
    # Only cache accepted answers. An unsolved part must be checked again later.
    if accepted:
        cache.parent.mkdir(parents=True, exist_ok=True)
        cache.write_text(json.dumps(accepted, indent=2) + "\n", encoding="utf-8")
    return accepted.get(str(part))


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
    expected = (
        sample_answer(year, day, part) if mode == "sample" else remote_answer(year, day, part)
    )
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
    matched = False
    for year_dir in sorted(Path("src").glob("[0-9][0-9][0-9][0-9]")):
        year = int(year_dir.name)
        if args.year is not None and year != args.year:
            continue
        for lang, entry_point in entry_points.items():
            print(year, lang)
            if args.lang is not None and lang != args.lang:
                continue
            if not (year_dir / lang / entry_point).is_file():
                continue
            matched = True
            days = (
                [args.day] if args.day is not None else list(range(1, 26 if year < 2025 else 13))
            )
            for day in days:
                filename = (
                    f"day{day:02}/solution.go"
                    if lang == "golang"
                    else f"day{day:02}{'.hpp' if lang == 'cpp' else Path(entry_point).suffix}"
                )
                if not (year_dir / lang / "solutions" / filename).is_file():
                    continue
                for part in (1, 2):
                    if day == days[-1] and part == 2:
                        continue
                    for mode in ("sample", "real"):
                        try:
                            failed |= verify_case(
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
                            failed = 1
    if not matched:
        parser.error("No matching year/language entry points found")
    return failed


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, TypeError, OSError, URLError) as error:
        sys.exit(f"Error: {error}")
