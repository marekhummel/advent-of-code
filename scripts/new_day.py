"""Copy a day template and insert registrations before explicit comment markers."""

from pathlib import Path

LANGUAGES = {"rust": "rs", "python": "py", "zig": "zig", "elixir": "ex", "golang": "go"}


def insert(text: str, marker: str, entry: str) -> str:
    lines = text.splitlines(keepends=True)
    matches = [line for line in lines if line.strip() == marker]
    if len(matches) != 1:
        raise ValueError(f"Expected exactly one {marker} marker")
    if entry in (line.strip() for line in lines):
        raise ValueError(f"Already registered: {entry}")
    line = matches[0]
    indent = line[: line.index(marker)]
    return text.replace(line, f"{indent}{entry}\n{line}", 1)


def create_day(root: Path, year: int, day: int, lang: str) -> None:
    directory = root / str(year) / lang
    name = f"day{day:02}"
    destination = (
        directory / name / "main.go"
        if lang == "golang"
        else directory / "solutions" / f"{name}.{LANGUAGES[lang]}"
    )
    if not directory.is_dir():
        raise ValueError("Create the year's language directory first; see README")
    if destination.exists() or destination.is_symlink():
        raise ValueError(f"Refusing to overwrite day {day:02}")
    if lang in {"rust", "zig", "elixir"}:
        days = sorted(
            int(path.stem[3:])
            for path in (directory / "solutions").glob(f"day[0-9][0-9].{LANGUAGES[lang]}")
        )
        if days != list(range(1, day)):
            raise ValueError(
                f"{lang} uses positional registration; add days in order without gaps"
            )

    registrations = {
        "rust": [
            ("main.rs", "// aoc:imports", f"use solutions::{name};"),
            ("main.rs", "// aoc:solutions", f"Box::new({name}::Solution{day:02} {{}}),"),
            ("main.rs", "// aoc:tests", f"test_day!({name});"),
            ("solutions/mod.rs", "// aoc:modules", f"pub mod {name};"),
        ],
        "python": [
            ("main.py", "# aoc:imports", f"from .solutions.{name} import Solution{day:02}"),
            ("main.py", "# aoc:solutions", f"{day}: Solution{day:02},"),
        ],
        "zig": [
            ("main.zig", "// aoc:imports", f'const {name} = @import("solutions/{name}.zig");'),
            ("main.zig", "// aoc:solutions", f"aoc_lib.solution.makeSolution({name}),"),
        ],
        "elixir": [
            (
                "main.exs",
                "# aoc:imports",
                f'Code.require_file(Path.join([__DIR__, "solutions", "{name}.ex"]))',
            ),
            ("main.exs", "# aoc:solutions", f"Day{day:02},"),
        ],
        "golang": [],
    }
    template = root / "scripts/templates" / f"day_template.{LANGUAGES[lang]}"
    content = template.read_text().replace("XX", f"{day:02}").replace("YYYY", str(year))
    changes = {}
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
    print(f"Created and registered {year}/{lang} day {day:02}")


if __name__ == "__main__":
    import sys

    try:
        year, day, lang = sys.argv[1:]
        create_day(Path(__file__).resolve().parents[1], int(year), int(day), lang)
    except (ValueError, OSError) as error:
        sys.exit(f"Error: {error}")
