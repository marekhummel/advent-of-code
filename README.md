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
| Rust | Latest stable rustup toolchain, Cargo, rust-src, rustfmt and Clippy |
| Go | Go 1.27.3, including gofmt, vet and test; gopls 0.24.0 and golangci-lint 2.15.0 |
| Zig | Zig 0.17.1, including its formatter |
| Elixir | Ubuntu-packaged Elixir, Erlang/OTP, Mix and Dialyzer |
| Python | Python 3.15 from the deadsnakes PPA, pip, Ruff and mypy |

Python solution dependencies are listed in `requirements.txt`; `pyproject.toml`
configures Ruff and mypy, whose versions are set in the Dockerfile.
The `pulp[cbc]` extra supplies the CBC solver used by `COIN_CMD`.
The Python build task installs the listed packages globally using pip; packages
already at the required version are reused. Explicit builds show pip output;
`task --silent build:python` also quiets pip through Task's `CLI_SILENT`
variable. Run this build task when Python dependencies change. The container provides isolation,
so neither Poetry nor a Python virtual environment is needed.
Python solutions use `python3.15`; helper scripts
use Ubuntu's `python3`. Toolchains are installed globally, while the editor and
terminals run as the unprivileged `ubuntu` user. Dev Containers matches its UID
to your host account. The container disables SELinux labelling so the workspace
bind mount is accessible on Fedora.

VS Code's encoding, line endings, indentation and language-specific editor
settings live in `.devcontainer/devcontainer.json`. Its Python settings select
Python 3.15 and add `src/` to the editor's import search path.
Rust-analyzer uses the root `Cargo.toml` and the installed `rust-src` component.
ElixirLS uses root `mix.exs` to analyse the shared library and 2025 sources in
the `dev` environment. Task still builds Elixir years independently.
ElixirLS maintains its own ignored `.elixir_ls/` cache.
The Go extension uses the image's preinstalled gopls and golangci-lint v2 from
`/usr/local/bin/`. Tool versions are set in the Dockerfile; extension updates
do not automatically replace them.

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

For Python packages, edit `requirements.txt` and run `task build:python`.
Runs and verification also execute this build step automatically.

Project dependencies remain separate: Cargo builds fetch `Cargo.lock`
automatically, and Zig builds fetch the URL/hash dependencies in `build.zig.zon`.
Update them explicitly when needed:

```sh
cargo update
# Update Zig dependency URLs/hashes in build.zig.zon, then:
task build:zig YEAR=2024
```

Elixir checks use the development dependency Dialyxir. Update it with
`mix deps.update dialyxir`; `task check:elixir` fetches dependencies as needed.
Go currently has no external packages. Updating compiler versions can
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
task build:zig YEAR=2024
task run:elixir YEAR=2025
task run:python YEAR=2023 DAY=1 PART=1 MODE=sample
task verify:zig YEAR=2024 DAY=1      # verify both parts, sample and real
task check:python                   # formats, lints and checks types
task check:golang YEAR=2015          # formats, vets and tests
task newday:python YEAR=2025 DAY=11
task input YEAR=2025 DAY=3                # omit DAY for all missing days
```

Build, run, check and new-day commands take the language in the task name.
`verify:LANGUAGE` checks one language; `verify` checks all languages. `list` always
shows all year/language directories without filters.

Task validates arguments before running commands. `task --list` shows which
arguments each task requires; Python's dependency-only build does not need a year.

| Argument | Allowed values |
| --- | --- |
| `YEAR` | Existing four-digit year folders under `src/` |
| `DAY` | 1-25 through 2024; 1-12 from 2025; leading zeros are accepted |
| `PART` | `1`, `2`; omitted means both |
| `MODE` | `real`, `sample`, `both` (default) |
| `OPTIMIZE` | `debug`, `fast` (default); verification always uses `fast` |

Languages: `rust`, `python`, `golang`, `zig`, `elixir`. Each case uses the
language’s build-and-run command, which reuses cached compilation work. Omit `DAY` to try days 1-25 for 2015-2024 or days 1-12 from
2025 onwards; omit `PART` for both parts.
`MODE` accepts `real`, `sample` or `both` and defaults to `both`. Both mode runs
sample input followed by real input for each day/part. Answers go directly to
stdout, and labels and errors go to stderr. Batch runs
continue after failures and exit nonzero if any case fails.
Ctrl+C stops the batch and the active runner, including its child processes.
Elixir builds compile the shared library, the year's solutions and `main.ex`
through Mix into `build/elixir/YEAR/`. Each case checks for source changes and
starts its own Erlang VM. Task passes the year as an argument to `mix aoc`;
editor checks default to 2025. Use `mix aoc YEAR` to build or
`mix aoc YEAR DAY PART MODE` to build and run.
Labels include elapsed wall time, including build checks, Task dispatch and process startup, with five decimal
places: `D01 P1 SAMP (  0.12345 secs): result`.

Verification always checks complete days: both parts, each with sample and real
input, comparing each answer separately. Use `verify:LANGUAGE` for one language
or `verify` for all languages. Omit `YEAR` for all matching years, and
omit `DAY` for every day with an existing solution file (1-25 through 2024,
1-12 from 2025). Go uses `solutions/dayDD/solution.go`; other languages use
`solutions/dayDD.EXT`. Days without files are skipped, including when `DAY` is
specified. Only projects with a language entry point are selected. Every
verification case builds and runs in release mode (`OPTIMIZE=fast`), regardless
of any `OPTIMIZE` argument supplied to Task. Compiler caches reuse unchanged work.

```sh
task verify:python YEAR=2023 DAY=1
task verify YEAR=2024 DAY=1               # every language for day 1 of 2024
task verify:rust DAY=1               # day 1 in every Rust year
task verify YEAR=2025                     # every day and language in 2025
task verify                               # every project and day
task --verbose verify YEAR=2025           # include passing and skipped results
```

`PART` and `MODE` are not verification filters. Verification continues after
case failures and returns nonzero if any case fails.
Day 25 part 2 is skipped for both sample and real input.
Verification normally prints only failures, unknown answers and errors.
Use `task --verbose verify` (or `scripts/verify.py --verbose`) to include passes
and skipped cases. This controls output only; exit statuses are unchanged.

Samples use `inputs/YEAR/sample_solutions.json`, keyed by two-digit day and part.
Answers are strings; explicit `null` expects the runner to return
`<No Sample Defined>` and passes when it does. An omitted day or part is unknown,
as is a missing accepted answer on the remote puzzle page.
Maintain this file alongside the sample inputs:

```json
{
  "01": {"1": "11", "2": "31"}
}
```

Real-input verification fetches your previously accepted answers from the puzzle
page using `scripts/aoc_cookie.txt`. Accepted answers are cached by session under
`build/answers/`; deleting that directory causes them to be fetched again.
Unsolved parts are not cached. No answers are submitted to the website.

The verifier reports aligned `PASSED`, `FAILED` or `UNKNOWN` statuses (no
expected/accepted answer), with padded language and input-mode columns.
`<Not Implemented>` and `<Unsolved>` results are reported as `SKIPPED`, without
fetching an expected answer, and do not make verification fail.
Labels use `<language> on (year day part) with sample/real`. Passes and unknown
cases print the result; mismatches print expected and actual answers, including
`<No Sample Defined>` results. Verification calls `task --silent case:LANGUAGE`
in release mode. It gets only the answer on stdout;
runner errors remain visible on stderr.
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
`<No Part Two>` and exit successfully. An unregistered day prints `<Unsolved>`;
a missing part or a part returning no result prints `<Not Implemented>`.
Explicit `Unsolved` results also print `<Unsolved>`. These are informational
results with exit code `0` in every language. Missing input and execution errors
remain failures. Python exposes `NO_SAMPLE`, `NO_PART_TWO`, `NOT_IMPLEMENTED`
and `UNSOLVED` from `lib.python`; Go exposes
`lib.NoSample` and `lib.NoPartTwo` (return them with a `nil` error). Elixir uses
`:no_sample`, `:no_part_two`, `:not_implemented` and `:unsolved`.
Go additionally exposes `lib.NotImplemented` and `lib.Unsolved`.

Language run commands live in Taskfile alongside their build commands.
`run.sh` handles batches, timing and labels; `verify.py` handles reference answers
and comparison. Both helpers use named options and are called through Task.
They invoke `case:LANGUAGE` for each configuration; the native tools rebuild
when needed. Python runs directly; install its dependencies with
`task build:python` before the first run or after changing `requirements.txt`.

```sh
task run:zig YEAR=2024 DAY=2 MODE=sample
task --silent case:zig YEAR=2024 DAY=2 PART=1 MODE=sample
```

`case:LANGUAGE` requires a day, part and a single mode (`sample` or `real`).
It uses `cargo run`, `go run`, `zig build YEAR_run`, or `mix aoc YEAR` for compiled
languages; Python uses `python3.15` directly. Task sets the Python import
and bytecode cache paths for all calls.

`check:LANGUAGE` formats and checks files; Rust, Python, Go and Zig accept an
optional `YEAR`. `check:elixir` checks the whole project, prints compiler warnings
and runs Dialyzer type analysis on
the sources selected in `mix.exs` (currently 2025 and the shared library).
The first Dialyzer run builds an analysis cache under `build/elixir/plts/` and
can take several minutes; later runs reuse it. ElixirLS-only hints such as
suggested `@spec` annotations are editor features and are not printed by Dialyzer.

Native builds default to `OPTIMIZE=fast`. The option works with `build:LANGUAGE` and
`run:LANGUAGE`; verification always uses release mode:

| Language | `fast` (default) | `debug` |
| --- | --- | --- |
| Zig | Optimised for speed | Unoptimised, with runtime safety checks |
| Rust | Cargo release profile | Cargo dev profile, binary under `build/rust/debug/` |
| Go | Normal optimised build | Optimisations and inlining disabled |

Debug builds can reduce compilation time but run slower. Python and Elixir
ignore `OPTIMIZE`.

```sh
task build:zig YEAR=2024 OPTIMIZE=debug
task run:zig YEAR=2024 DAY=1 OPTIMIZE=debug
task run:rust YEAR=2023 OPTIMIZE=debug
task build:golang YEAR=2024 OPTIMIZE=debug
```

`OPTIMIZE=debug` selects the Cargo development profile in `case:LANGUAGE`.

Zig always builds the entire year to `build/zig/out/bin/mainYEAR`. `DAY` only
filters which cases to run or verify. The build-and-run commands check for
changes on every case; compiler caches reuse unchanged work.

```sh
task build:zig YEAR=2024
task run:zig YEAR=2024 DAY=22 PART=2 MODE=real
task verify:zig YEAR=2024 DAY=22
```

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
| Elixir | `mix.exs`, `mix.lock` | Mix builds under `build/elixir/YEAR/`; dependencies and analysis caches under `build/elixir/`; editor cache under `.elixir_ls/` |

`build/` is ignored. Cargo also maintains a dependency cache in the container
user's home directory. Task keeps Zig packages and compiler caches under
`build/zig/`; direct native commands may need the same flags/environment
variables. VS Code's editing defaults are configured in `.devcontainer/devcontainer.json`.

## Adding Years

Create `inputs/YEAR/` and `src/YEAR/LANGUAGE/` manually, using an existing year's
entry point as a reference. Set its runner's year and keep the `aoc:*` registration
markers so `task newday:LANGUAGE` can insert imports and solutions from `scripts/templates/`.
All languages register days by number, so days can be added in any order
without placeholder files or padding entries.
In Elixir maps, place `# aoc:solutions` before the first entry so inserted
entries retain a following element after formatting.

| Language | Entry point and references to add |
| --- | --- |
| Rust | `main.rs` with a day-number `HashMap`, `solutions/mod.rs`; add a `[[bin]]` in root `Cargo.toml`: `name = "mainYEAR"`, `path = "src/YEAR/rust/main.rs"` |
| Python | `main.py`, `__init__.py`, `solutions/__init__.py`; no root registration |
| Go | `main.go` with a `lib.Run(YEAR, map[int]lib.Solution{...})` registry; day packages in `solutions/dayDD/`; no root registration |
| Zig | `main.zig` with day/solution pairs using `aoc_lib.solution.makeSolution(dayDD)`; add `add_binary(b, "YEAR");` in root `build.zig` |
| Elixir | `main.ex` with a `Main.main/1` entry point and day-number map; Mix compiles shared modules and days. Task passes the year to `mix aoc`; adjust the default year in root `mix.exs` for editor analysis of a different year. Years share module names and must be analysed separately. |

Put the year's README in `src/YEAR/README.md`. Link screenshots with
`../../progress/YEAR/overview.png` and `../../progress/YEAR/conclusion.png`.
Create `inputs/YEAR/sample_solutions.json` for sample expectations.
