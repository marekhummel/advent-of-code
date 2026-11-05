# Advent Of Code

Solutions live in `src/YEAR/LANGUAGE/`, shared code in `src/lib/LANGUAGE/`,
and inputs and sample expectations in `inputs/YEAR/`. Yearly READMEs stay in
`src/YEAR/`; screenshots live in `progress/YEAR/`, with the overall image at
`progress/total.png`. Run commands from the repository root.

## Install

Keep VS Code, the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
and Docker on the host. Open this repository in VS Code and select
**Dev Containers: Reopen in Container**. The image build installs the toolchains;
language build tasks install project dependencies when needed.

The [Dockerfile](.devcontainer/Dockerfile) starts with minimal Ubuntu 26.04,
applies system updates and installs globally, in this order:

| Group | Installed tools |
| --- | --- |
| Common | Bash, Git, curl, certificates, archive tools, C compiler/linker, sudo, Task |
| Rust | Latest stable rustup toolchain, Cargo, rustfmt and Clippy |
| Go | Go 1.27.3, including gofmt, vet and test |
| Zig | Zig 0.17.1, including its formatter |
| Elixir | Ubuntu-packaged Elixir, Erlang/OTP and Mix |
| Python | Python 3.15 from the deadsnakes PPA, pip, Ruff and mypy |

Python solution dependencies are listed in `requirements.txt`; `pyproject.toml`
configures Ruff and mypy, whose versions are set in the Dockerfile.
The `pulp[cbc]` extra supplies the CBC solver used by `COIN_CMD`.
The Python build task installs the listed packages globally using pip; packages
already at the required version are reused. The container provides isolation,
so neither Poetry nor a Python virtual environment is needed.
Python solutions use `python3.15`; helper scripts
use Ubuntu's `python3`. Toolchains are installed globally, while the editor and
terminals run as the unprivileged `ubuntu` user. Dev Containers matches its UID
to your host account. The container disables SELinux labelling so the workspace
bind mount is accessible on Fedora.

VS Code's encoding, line endings, indentation and language-specific editor
settings live in `.devcontainer/devcontainer.json`. Its Python settings select
Python 3.15 and add `src/` to the editor's import search path.

To use your personal VS Code profile, run **Profiles: Switch Profile** while
connected to the container. VS Code remembers the profile for that workspace;
container settings apply on top of the profile's settings. Select any additional
remote extensions through **Install Local Extensions in Dev Container** in the
Extensions view. Profile selection is local to VS Code.

The image build context is `.devcontainer/`. Source code, inputs and the session
cookie are available through the mounted checkout. Firefox profiles are not
mounted: paste the session token into `scripts/aoc_cookie.txt`, or run
`python3 scripts/read_cookie.py` on the host.

## Update

Edit tool versions in `.devcontainer/Dockerfile`, then select
**Dev Containers: Rebuild Container Without Cache**. This also refreshes Ubuntu
packages and the stable Rust toolchain.

For Python packages, edit `requirements.txt` and run `task build LANG=python`.
Runs and verification also execute this build step automatically.

Project dependencies remain separate: Cargo builds fetch `Cargo.lock`
automatically, and Zig builds fetch the URL/hash dependencies in `build.zig.zon`.
Update them explicitly when needed:

```sh
cargo update
# Update Zig dependency URLs/hashes in build.zig.zon, then:
task build LANG=zig YEAR=2024
```

Go and Elixir currently have no external packages. Updating compiler versions can
require source changes; keep `go.mod`, `build.zig.zon` and Python's version in
`pyproject.toml` consistent with the selected toolchains.

## Usage

Run commands in the container from the repository root. Task sets `PYTHONPATH`
to `src/`, and VS Code uses the same import root. Python entry points run as
files, so shared imports use `from lib.python import ...` and day imports use
`from solutions.day01 import Solution01`. For a direct run:

```sh
PYTHONPATH=src python3.15 src/2023/python/main.py 1 1 sample
```

```sh
task --list
task list
task build LANG=zig YEAR=2024
task run LANG=elixir YEAR=2025
task run LANG=python YEAR=2023 DAY=1 PART=1 MODE=sample
task verify LANG=zig YEAR=2024 DAY=1 PART=1 MODE=real # compare with your accepted answer
task check LANG=python                   # formats, lints and checks types
task check LANG=golang YEAR=2015          # formats, vets and tests
task newday LANG=python YEAR=2025 DAY=11
task input YEAR=2025 DAY=3                # omit DAY for all missing days
```

Languages: `rust`, `python`, `golang`, `zig`, `elixir`. Task builds before calling
`scripts/run.sh`. Omit `DAY` to try days 1-25 for 2015-2024 or days 1-12 from
2025 onwards; omit `PART` for both parts.
`MODE` accepts `real`, `sample` or `both` and defaults to `both`. Both mode runs
sample input followed by real input for each day/part. Answers go directly to
stdout, and labels and errors go to stderr. Batch runs
continue after failures and exit nonzero if any case fails.
Ctrl+C stops the batch and the active runner, including its child processes.
Elixir builds compile the shared library, the year's solutions and `main.ex`
into `build/elixir/YEAR/`. Unchanged builds are skipped. Runs load the compiled
modules, while each case still starts its own Erlang VM.
Labels include elapsed wall time, including process startup, with five decimal
places: `D01 P1 SAMP (  0.12345 secs): result`.

Verification runs one day/part with `MODE=real`, `MODE=sample` or the default
`MODE=both`, comparing each answer separately:

```sh
task verify LANG=python YEAR=2023 DAY=1 PART=1 MODE=sample
task verify LANG=rust YEAR=2022 DAY=1 PART=1 MODE=real
```

Samples use `inputs/YEAR/sample_solutions.json`, keyed by two-digit day and part.
Answers are strings; `null` marks an unavailable case and omitted entries are
unknown. Maintain this file alongside the sample inputs:

```json
{
  "01": {"1": "11", "2": "31"}
}
```

Real-input verification fetches your previously accepted answers from the puzzle
page using `scripts/aoc_cookie.txt`. Accepted answers are cached by session under
`build/answers/`; deleting that directory causes them to be fetched again.
Unsolved parts are not cached. No answers are submitted to the website.

The verifier reports `PASS`, `MISMATCH` or `UNKNOWN` (no expected/accepted answer).
Mismatches, unknown answers, runner failures and authentication/network errors
produce a nonzero exit. Comparison preserves answer whitespace except for the
single newline appended by the runner. Multiline sample answers use JSON `\n`
escapes.

Language entry points accept `DAY PART MODE`, where `MODE` is `real` or `sample`.
They print one answer followed by one newline and report errors on stderr.
The shell script expands `both` into separate calls. Exit codes are `0` for success, `1`
for execution/verification failures and `2` for invalid arguments; crashes may
use other nonzero codes.

Returned `NoSample` and `NoPartTwo` results print `<No Sample Defined>` and
`<No Part Two>` and exit successfully. Missing input and unsolved cases remain
errors. Python exposes `NO_SAMPLE` and `NO_PART_TWO` from `lib.python`; Go exposes
`lib.NoSample` and `lib.NoPartTwo` (return them with a `nil` error). Elixir uses
`:no_sample` and `:no_part_two`.
Go also treats missing days, missing part functions and returned `lib.Unsolved`
values as informational results with exit code `0`.
Elixir likewise treats missing days, missing parts and `:unsolved` results as
informational results with exit code `0`.

Scripts use `LANG YEAR DAY PART MODE` order where applicable; downloads need only
`YEAR DAY`. Direct calls expect an existing build:

```sh
task build LANG=zig YEAR=2024
bash scripts/run.sh zig 2024                         # full year, sample and real input
bash scripts/run.sh zig 2024 2 sample                # both parts of sample day 2
python3 scripts/verify.py zig 2024 2 1 sample          # verify one case
```

Empty positional arguments allow a part-only filter, e.g.
`bash scripts/run.sh zig 2024 "" 1 sample`. Task sets the Python import and
bytecode cache paths; direct Python runs need `PYTHONPATH=src` and may set
`PYTHONPYCACHEPREFIX=build/python/pycache` to keep bytecode under `build/`.

`check` formats files and accepts an optional `YEAR`.

Downloads read the session token from ignored `scripts/aoc_cookie.txt`. Put the
token alone in that file, or run `task cookie` on the host after logging in
through Firefox and closing it completely. Firefox profiles are not mounted in
the devcontainer. The importer supports native and Flatpak Firefox profiles.
Inputs are named
`inputDD.txt` and `sampleDD.txt`, with `_1.txt` / `_2.txt` part-specific fallbacks.

## Files And Local Output

| Owner | Root configuration | Generated/local output |
| --- | --- | --- |
| Common | `.devcontainer/`, `Taskfile.yml`, `.gitignore` | `.git/`, editor state, accepted-answer cache under `build/answers/` |
| Rust | `Cargo.toml`, `Cargo.lock`, `.cargo/config.toml`, `.rustfmt.toml` | `build/rust/` |
| Python | `requirements.txt`, `pyproject.toml` | `build/python/`: Ruff/mypy caches and bytecode |
| Go | `go.mod` | `build/golang/`: binaries and compilation cache; temporary builds use system temp |
| Zig | `build.zig`, `build.zig.zon` | `build/zig/`: binaries and compiler caches |
| Elixir | None | Compiled BEAM modules under `build/elixir/YEAR/`; Mix paths under `build/elixir/` |

`build/` is ignored. Cargo also maintains a dependency cache in the container
user's home directory. Task keeps Zig packages and compiler caches under
`build/zig/`; direct native commands may need the same flags/environment
variables. VS Code's editing defaults are configured in `.devcontainer/devcontainer.json`.

## Adding Years

Create `inputs/YEAR/` and `src/YEAR/LANGUAGE/` manually, using an existing year's
entry point as a reference. Set its runner's year and keep the `aoc:*` registration
markers so `task newday` can insert imports and solutions from `scripts/templates/`.
All languages register days by number, so days can be added in any order
without placeholder files or padding entries.
In Elixir maps, place `# aoc:solutions` before the first entry so inserted
entries retain a following element after formatting.

| Language | Entry point and references to add |
| --- | --- |
| Rust | `main.rs` with a day-number `HashMap`, `solutions/mod.rs`; add a `[[bin]]` in root `Cargo.toml`: `name = "mainYEAR"`, `path = "src/YEAR/rust/main.rs"` |
| Python | `main.py`, `__init__.py`, `solutions/__init__.py`; no root registration |
| Go | `main.go` with a `lib.Run(YEAR, map[int]lib.Solution{...})` registry; day packages in `solutions/dayDD/`; no root registration |
| Zig | `main.zig`; add `add_binary(b, "YEAR");` in root `build.zig` |
| Elixir | `main.ex` with a `Main.main/1` entry point and day-number map; shared modules and days are compiled by the build task; no root registration |

Put the year's README in `src/YEAR/README.md`. Link screenshots with
`../../progress/YEAR/overview.png` and `../../progress/YEAR/conclusion.png`.
Create `inputs/YEAR/sample_solutions.json` for sample expectations.
