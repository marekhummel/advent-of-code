"""Copy a day template and insert registrations after explicit comment markers."""

from pathlib import Path

LANGUAGES = {
    "rust": "rs",
    "python": "py",
    "zig": "zig",
    "elixir": "ex",
    "golang": "go",
    "cpp": "hpp",
}


def insert(text: str, marker: str, entry: str) -> str:
    lines = text.splitlines(keepends=True)
    matches = [line for line in lines if line.strip() == marker]
    if len(matches) != 1:
        raise ValueError(f"Expected exactly one {marker} marker")
    if entry in (line.strip() for line in lines):
        raise ValueError(f"Already registered: {entry}")
    line = matches[0]
    indent = line[: line.index(marker)]
    return text.replace(line, f"{line.rstrip()}\n{indent}{entry}\n", 1)


def create_day(root: Path, lang: str, year: int, day: int) -> None:
    directory = root / "src" / str(year) / lang
    name = f"day{day:02}"
    destination = (
        directory / "solutions" / name / "solution.go"
        if lang == "golang"
        else directory / "solutions" / f"{name}.{LANGUAGES[lang]}"
    )
    if not directory.is_dir():
        raise ValueError("Create the year's language directory first; see README")
    if destination.exists() or destination.is_symlink():
        raise ValueError(f"Refusing to overwrite day {day:02}")
    registrations = {
        "cpp": [
            ("main.cpp", "// aoc:imports", f'#include "solutions/{name}.hpp"'),
            ("main.cpp", "// aoc:solutions", f"{{{day}, {name}::solution}},"),
        ],
        "rust": [
            ("main.rs", "// aoc:imports", f"use solutions::{name};"),
            (
                "main.rs",
                "// aoc:solutions",
                f"({day}, Box::new({name}::Solution{day:02} {{}})),",
            ),
            ("solutions/mod.rs", "// aoc:modules", f"pub mod {name};"),
        ],
        "python": [
            ("main.py", "# aoc:imports", f"from solutions.{name} import Solution{day:02}"),
            ("main.py", "# aoc:solutions", f"{day}: Solution{day:02},"),
        ],
        "zig": [
            ("main.zig", "// aoc:imports", f'const {name} = @import("solutions/{name}.zig");'),
            (
                "main.zig",
                "// aoc:solutions",
                f".{{ {day}, aoc_lib.solution.makeSolution({name}) }},",
            ),
        ],
        "elixir": [
            ("main.ex", "# aoc:solutions", f"{day} => Day{day:02},"),
        ],
        "golang": [
            ("main.go", "// aoc:imports", f'"aoc/src/{year}/golang/solutions/{name}"'),
            (
                "main.go",
                "// aoc:solutions",
                f"{day}: {{Part1: {name}.Part1, Part2: {name}.Part2}},",
            ),
        ],
    }
    template = root / "scripts/templates" / f"day_template.{LANGUAGES[lang]}"
    content = template.read_text().replace("XX", f"{day:02}").replace("YYYY", str(year))
    changes: dict[Path, str] = {}
    for filename, marker, entry in registrations[lang]:
        path = directory / filename
        text = changes[path] if path in changes else path.read_text()
        changes[path] = insert(text, marker, entry)

    # Validate every registration before creating or changing any files.
    destination.parent.mkdir(parents=True, exist_ok=True)
    with destination.open("x") as output:
        output.write(content)
    for path, text in changes.items():
        path.write_text(text)
    print(f"Created and registered {lang}/{year} day {day:02}")


if __name__ == "__main__":
    import sys

    try:
        lang, year, day = sys.argv[1:]
        create_day(Path(__file__).resolve().parents[1], lang, int(year), int(day))
    except (ValueError, OSError) as error:
        sys.exit(f"Error: {error}")
