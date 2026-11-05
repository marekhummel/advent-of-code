"""Run one case and compare its answer with sample data or Advent of Code."""

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
    answer = parts.get(str(part))
    if answer is not None and not isinstance(answer, str):
        raise TypeError(f"Expected a string or null for day {day:02} part {part} in {path}")
    return answer


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


def verify_case(lang: str, year: int, day: int, part: int, mode: str, optimize: str) -> int:
    command = [
        "bash",
        "scripts/run.sh",
        lang,
        str(year),
        str(day),
        str(part),
        mode,
        "--optimize",
        optimize,
    ]
    result = subprocess.run(command, stdout=subprocess.PIPE, check=False)
    if result.returncode:
        return 1
    actual = result.stdout.decode("utf-8").removesuffix("\n")
    expected = (
        sample_answer(year, day, part) if mode == "sample" else remote_answer(year, day, part)
    )
    label = f"{lang}/{year} day {day:02} part {part} {mode}"
    if expected is None:
        print(
            f"UNKNOWN: {label} has no {'sample' if mode == 'sample' else 'accepted'} answer",
            file=sys.stderr,
        )
        return 1
    if actual != expected:
        print(f"MISMATCH: {label}: expected {expected!r}, got {actual!r}", file=sys.stderr)
        return 1
    print(f"PASS: {label}")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("lang", choices=("rust", "python", "golang", "zig", "elixir"))
    parser.add_argument("year", type=int)
    parser.add_argument("day", type=int, choices=range(1, 26))
    parser.add_argument("part", type=int, choices=(1, 2))
    parser.add_argument("mode", nargs="?", choices=("real", "sample", "both"), default="both")
    parser.add_argument("--optimize", default="fast", choices=("debug", "fast", "safe", "small"))
    args = parser.parse_args()
    os.chdir(Path(__file__).resolve().parents[1])
    modes = ("sample", "real") if args.mode == "both" else (args.mode,)
    failed = 0
    for mode in modes:
        if verify_case(args.lang, args.year, args.day, args.part, mode, args.optimize):
            failed = 1
    return failed


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, TypeError, OSError, URLError) as error:
        sys.exit(f"Error: {error}")
