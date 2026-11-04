# Advent Of Code

Solutions for 2015-2025. The primary language is Rust for 2015-2023, Zig for
2024 and Elixir for 2025. Python and Go provide additional implementations.
Solutions live in `YEAR/LANGUAGE/`, shared libraries in `lib/LANGUAGE/`, and
inputs in `YEAR/inputs/`. Each daily solution declares its expected answers.

### Setup

Run commands from the repository root. Install [Task 3](https://taskfile.dev/docs/installation)
and Python 3.12+, Bash and curl, then the toolchains for your chosen languages.

| Language | Required tools and dependencies | `check` tools |
| --- | --- | --- |
| Rust | Stable Rust, edition 2021; `cargo fetch` | rustfmt / Clippy |
| Python | Poetry; `task build YEAR=2023 LANG=python` | Ruff / mypy installed on PATH |
| Zig | Zig **0.13.x**; `zig build --fetch --cache-dir build/zig/cache` | zig fmt |
| Go (`LANG=golang`) | Go >=1.23.4 | gofmt / go vet and go test |
| Elixir | Elixir 1.19+ and compatible Erlang/OTP | mix format |

Poetry creates this project's virtual environment under `build/python/venv/`,
as configured in `poetry.toml` using [Poetry's environment settings](https://python-poetry.org/docs/configuration/#virtualenvspath).
The Python build task runs `poetry install` when the Poetry environment does not
exist, and skips installation when it already exists. Python runs call this build
step first and execute solutions with `poetry run python`. After dependency changes,
run `poetry install` explicitly to update an existing environment.
`poetry env info --path` prints the environment's location. Python checks use
Ruff and mypy directly from PATH.

```sh
task --list
task list
task doctor                               # all required local binaries
task doctor LANG=golang                   # one language plus shared tools
```

`doctor` reports executable paths, versions and setup guidance. Missing or broken
executables produce a nonzero exit code. Downloaded dependencies and puzzle inputs
are checked by the commands that use them; respect the displayed version requirements.
Ruff and mypy are checked on PATH, like the other development tools. `doctor`
does not create an environment or install packages.

Compilers and toolchain commands are installed outside the repository: Rust's
rustfmt and Clippy are toolchain components, Go includes gofmt and vet, Zig
includes its formatter, and Elixir includes Mix. Project libraries are a separate
layer: Cargo and Zig use project manifests and shared dependency caches. This Go
module has no external dependencies, and the Elixir scripts use no Mix packages.

### Running And Checking Solutions

```sh
task build YEAR=2024 LANG=zig
task run YEAR=2025 LANG=elixir
task run YEAR=2025 LANG=elixir DAY=7 PART=2 SAMPLE=true
task run YEAR=2023 LANG=python DAY=1 PART=1 SAMPLE=true
task check LANG=python
task check LANG=golang YEAR=2015
```

`YEAR` and `LANG` are required for `build` and `run`. `LANG` selects `rust`,
`python`, `zig`, `golang` or `elixir`. `check` requires `LANG` and accepts an
optional `YEAR`; otherwise it covers all years for that language. It formats
source files before running checks: Clippy for Rust, Ruff and mypy for Python,
and vet/test for Go. Zig and Elixir use their native formatters. Python years
are checked separately with mypy to avoid module-name collisions.
`task list` lists year/language directories, optionally filtered by `YEAR` and `LANG`.

Task dispatches directly to each language's native command. Without `DAY`, Rust,
Python, Zig and Elixir run their native `main` command for the whole year. With
`DAY` and `PART`, they run that part using real input, or samples with `SAMPLE=true`.
With `DAY` but no `PART`, the native `--all` option runs both parts with sample
and real inputs. Native whole-year and `--all` runs ignore `SAMPLE`.

Go requires `DAY`; each day is a separate program with fixed part/input behaviour.
Omit `PART` and `SAMPLE` for Go. Its 2024 implementation runs sample and real input
and implements only part 1.

Pass selections as Task variables such as `YEAR=2025`, `DAY=7`, `PART=2` and
`SAMPLE=true`. Native runners handle missing inputs and solution errors directly.
Environment variables hold build/cache settings.

Python runs prepare the Poetry environment if missing. Cargo runs offline, Zig
reads cached packages with `--system`, and Go disables automatic toolchain/module
fetching. Fetch dependencies during setup. `check` rewrites formatting in the
selected source files.

### Root Files

The manifests configure projects spanning several years. Their native locations
allow command-line tools and editor integrations to discover them automatically.

| Owner | Files | Purpose |
| --- | --- | --- |
| Rust | `Cargo.toml` | Package manifest, yearly binary targets and dependencies |
| Rust | `Cargo.lock` | Exact dependency versions |
| Rust | `.cargo/config.toml` | Cargo configuration: build output in `build/rust/` |
| Rust | `.rustfmt.toml` | Rust formatting configuration |
| Python | `pyproject.toml` | Dependencies and Ruff/mypy configuration |
| Python | `poetry.lock` | Exact Python dependency versions |
| Python | `poetry.toml` | Poetry configuration: environment location |
| Zig | `build.zig`, `build.zig.zon` | Build targets and pinned package dependencies |
| Go | `go.mod` | Module identity and Go version |
| Shared | `Taskfile.yml` | Developer commands |
| Shared | `.vscode/settings.json` | VS Code defaults: indentation, encoding and line endings |
| Shared | `.gitignore` | Local and generated files excluded from Git |
| Documentation | `README.md` | Setup and usage |
| Local tooling | `.git/`, other `.vscode/` files, `.agents/`, `.codex/` | Git, editor and agent state |

Rust has two configuration files plus a manifest and lockfile. Cargo and rustfmt
read their own formats; the manifest and lockfile describe the package and its
dependencies. VS Code workspace settings provide editing defaults; native
formatters apply each language's formatting rules.

### Local Environments And Build Output

`build/` is ignored by Git and contains generated output and local environments.

| Directory | Contents |
| --- | --- |
| `build/rust/` | Cargo binaries, incremental compilation and other target artefacts |
| `build/zig/out/` | Installed Zig binaries, including `bin/main2024` |
| `build/zig/cache/`, `build/zig/global/` | Zig compilation caches |
| `build/python/venv/` | Poetry virtual environments |
| `build/python/ruff/`, `build/python/mypy/` | Ruff and mypy caches |
| `build/python/pycache/` | Python bytecode generated through Task |
| `build/golang/cache/` | Go compilation cache |
| `build/golang/bin/YEAR/` | Go binaries produced by `task build` |

Go creates its cache and binary output directories automatically. `GOTMPDIR`
selects its temporary build workspace; it is unset here, so temporary files use
the system temporary directory. `task build LANG=golang YEAR=...` writes final
binaries under `build/golang/bin/YEAR/`. `task run LANG=golang ...` uses `go run`,
whose executable is temporary and is removed after the run.

Cargo, Ruff, mypy and Poetry read their paths from project configuration. Task
sets Python/Go environment variables and passes Zig cache/output flags. Direct
Python, Go and Zig commands need equivalent environment variables/flags to use
these paths. For example:

```sh
PYTHONPYCACHEPREFIX="$PWD/build/python/pycache" python3 -m 2023.python.main day 1 --part 1 --sample
```

Elixir solutions run as scripts without compilation output. Task sets
`MIX_BUILD_PATH=build/elixir/build` and `MIX_DEPS_PATH=build/elixir/deps` for Mix
commands. Shared dependency stores and download caches for Cargo, Go, Zig and
Poetry use their user-level locations. Offline Zig builds read packages from that
shared store and write compiler artefacts under `build/zig/`.

### Scripts

| Files | Purpose |
| --- | --- |
| `Taskfile.yml` | Common tasks followed by each language's build, run and check commands; build/cache environment variables |
| `scripts/doctor.sh` | Check required binaries for the selected language |
| `scripts/new_day.py`, `scripts/templates/` | Create a day from its language template and update the year's native registrations |
| `scripts/download_inputs.sh` | Download inputs with curl without overwriting existing files |
| `scripts/read_cookie.py` | Import a session from a local browser cookie database |

`task run` invokes a language's runner and forwards its output.

### Adding Days And Years

```sh
task new:day YEAR=2025 DAY=11 LANG=python
```

`new:day` requires an existing year/language project and explicit `YEAR`, `DAY`
and `LANG`. It reads the corresponding day template in `scripts/templates/`,
replaces `XX` with the two-digit day and `YYYY` with the year, and updates the
registrations listed below. Templates contain unimplemented solutions, without
puzzle inputs or answers. The task refuses existing day files and registrations,
validates the registration edits before writing, and does not download inputs.
Registration lines are inserted immediately before the `aoc:imports` and
`aoc:solutions` comment markers in the year's entry point. Rust also uses
`aoc:tests` in `main.rs` and `aoc:modules` in `solutions/mod.rs`. Keep these markers
on their own lines; their indentation determines the inserted lines' indentation.

| Language | Day template and destination | References updated by `new:day` |
| --- | --- | --- |
| Rust | `day_template.rs` to `YEAR/rust/solutions/dayDD.rs` | Add `pub mod dayDD;` in `solutions/mod.rs`; import the module and append `Box::new(dayDD::SolutionDD {})` in `main.rs`; add `test_day!(dayDD)` to its native test module |
| Python | `day_template.py` to `YEAR/python/solutions/dayDD.py` | Import `SolutionDD` in `main.py` and add `D: SolutionDD` to its `solutions` dictionary |
| Zig | `day_template.zig` to `YEAR/zig/solutions/dayDD.zig` | Import the day in `main.zig` and append `aoc_lib.solution.makeSolution(dayDD)` to the solutions array |
| Elixir | `day_template.ex` to `YEAR/elixir/solutions/dayDD.ex` | Require the file in `main.exs` after the shared library files; put `DayDD` in the solutions list at the day's position |
| Go | `day_template.go` to `YEAR/golang/dayDD/main.go` | Each day is a separate package; no yearly registry is needed |

Rust, Zig and Elixir identify days by position in the solutions collection, so
`new:day` requires sequential days for them. Python and Go support gaps between
implemented days. Each required marker must occur exactly once. The task reports
missing or duplicate markers before writing any files. Keep any Elixir `nil`
padding after the `aoc:solutions` marker so new days occupy the next list position.

New years are set up manually. Create `YEAR/inputs/` and the language directory. Use a current
year's entry point as a reference, keeping the registration markers and only the
day registrations you need:

| Language | Entry point and year/build references |
| --- | --- |
| Rust | Create `YEAR/rust/main.rs` and `solutions/mod.rs`, following `2024/rust/main.rs`. Set `AocRunner.year` and add a `[[bin]]` entry to root `Cargo.toml` with `name = "mainYEAR"` and `path = "YEAR/rust/main.rs"` |
| Python | Create `YEAR/python/main.py`, following `2023/python/main.py`, and set the year passed to `Runner`. Add empty `__init__.py` files in `python/` and `python/solutions/`. No root build registration is needed |
| Zig | Create `YEAR/zig/main.zig`, following `2024/zig/main.zig`, and set the year passed to `AocRunner.init`. Add `add_binary(b, "YEAR");` inside `pub fn build` in root `build.zig`; this provides the `YEAR_main` and `YEAR_test` build steps |
| Elixir | Create `YEAR/elixir/main.exs`, following `2025/elixir/main.exs`. Load the shared `lib/elixir/` modules, register your days and set the year passed to `AocLib.Runner.new`. No root build registration is needed |
| Go | Create `YEAR/golang/dayDD/` packages. They share root `go.mod`; no yearly entry point or root build registration is needed |

Use `task list` to inspect year/language directories and
`task run YEAR=... LANG=...` to run a new year.

### Inputs

```sh
task input YEAR=2025 DAY=3
task input YEAR=2025                       # all missing days
```

Inputs are shared across languages: `YEAR/inputs/inputDD.txt` and `sampleDD.txt`,
with `_1.txt` / `_2.txt` fallbacks when the common file is absent. Real inputs and
credentials are gitignored. Downloads read the session token from
`scripts/aoc_cookie.txt`. Put the token alone in that
file, or use `task cookie` to import it from Firefox. The importer searches native
Linux/Fedora, Flatpak and Windows profile locations for the Advent of Code session
cookie.

`task input YEAR=...` downloads missing days for that calendar. An explicit existing
day is refused. Failed downloads remove their output file. Credentials are passed
through a pipe into curl's configuration input (`--config -`), keeping the session
token out of curl's command-line arguments. Curl sends it as the `Cookie` header
over HTTPS. The shell helper also accepts `scripts/download_inputs.sh YEAR [DAY]`;

### Direct Runners And Extension Points

Native entry points run from the repository root:

- Rust: `cargo run --release --bin mainYYYY -- day N --part 1`
- Python: `poetry run python -m 2023.python.main day N --part 1`
- Elixir: `elixir 2025/elixir/main.exs day N --part 1`
- Go: `go run ./2015/golang/day07`

For Zig, build and run with explicit output paths:

```sh
zig build --cache-dir build/zig/cache --global-cache-dir build/zig/global --prefix build/zig/out -Doptimize=ReleaseFast 2024_main
./build/zig/out/bin/main2024 day 1 --part 1
```

Rust, Python, Zig and Elixir accept `main` to run the year, or `day N` with
`--part 1|2`, `--sample` and `--all`. Native `main`/`--all` commands run both samples
and real inputs; Task's `SAMPLE` argument selects one input category.

To add a language, add its native commands to `Taskfile.yml`, a day template in
`scripts/templates/`, and registration edits in `scripts/new_day.py`. Add its
required tools to `scripts/doctor.sh`.
