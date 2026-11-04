# Advent Of Code

Solutions live in `src/YEAR/LANGUAGE/`, shared code in `src/lib/LANGUAGE/`,
and inputs and sample expectations in `inputs/YEAR/`. Yearly READMEs stay in
`src/YEAR/`; screenshots live in `progress/YEAR/`, with the overall image at
`progress/total.png`. Run commands from the repository root.

## Install Tools

These instructions target current stable tools, independently of the project's
version settings. Python, its tools, Elixir and Erlang use apt/dnf, which provide
the latest packaged versions for your distribution and can lag behind upstream.
Choose your distribution's command, then install only the languages you need. Nothing here requires mise or uv.

Downloads below target Linux x86-64. For another architecture, select its archive
on the linked download page. Downloaded archives use ignored `build/tools/`;
installed toolchains live under `~/.local/opt/` (Rust uses `~/.cargo/` and
`~/.rustup/`). For PATH setup below, run only the Bash or Zsh commands matching
your shell. They persist the setting and reload it for the current shell.

### Common

Bash, curl, certificates, archive utilities and Python for helper scripts:

```sh
# Debian/Ubuntu
sudo apt update
sudo apt install -y bash curl ca-certificates tar xz-utils unzip python3
# Fedora
sudo dnf install -y bash curl ca-certificates tar xz unzip python3
```

Install the latest [Task](https://taskfile.dev/docs/installation) from its official repository:

```sh
# Debian/Ubuntu
curl -1sLf https://dl.cloudsmith.io/public/task/task/setup.deb.sh | sudo -E bash
sudo apt install -y task
# Fedora
curl -1sLf https://dl.cloudsmith.io/public/task/task/setup.rpm.sh | sudo -E bash
sudo dnf install -y task
```

### Rust

Install a C compiler/linker through the system package manager, then use [rustup](https://rust-lang.org/tools/install/) for the latest stable Rust, Cargo, rustfmt and Clippy.

```sh
# Debian/Ubuntu
sudo apt install -y build-essential
# Fedora
sudo dnf install -y gcc

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain stable
. "$HOME/.cargo/env"
rustup component add rustfmt clippy
```



### Python

Install Python 3.15 and the globally available Poetry, Ruff and mypy commands
through your system package manager:

```sh
# Debian/Ubuntu (when available in your release's repositories)
sudo apt install -y python3.15 python3.15-venv python3-poetry ruff mypy
# Fedora
sudo dnf install -y python3.15 python3-poetry ruff python3-mypy
```

[Python 3.15 on Fedora](https://packages.fedoraproject.org/pkgs/python3.15/python3.15/)
provides the `python3.15` executable. Package availability and patch versions
depend on your distribution's repositories. On supported Ubuntu releases missing
Python 3.15, use the [deadsnakes PPA](https://launchpad.net/~deadsnakes/+archive/ubuntu/ppa);
it is not a Debian repository.

### Go (`LANG=golang`)

Install **1.27.3**, the latest stable [Go](https://go.dev/dl/) release. gofmt, vet and test are included.

```sh
mkdir -p build/tools "$HOME/.local/opt/go-1.27.3"
curl -fL https://go.dev/dl/go1.27.3.linux-amd64.tar.gz -o build/tools/go.tar.gz
tar -xzf build/tools/go.tar.gz -C "$HOME/.local/opt/go-1.27.3" --strip-components=1

rcfile=~/.bashrc # ~/.zshrc
echo 'export PATH="$HOME/.local/opt/go-1.27.3/bin:$PATH"' >> $rcfile
source $rcfile
```


### Zig

Install **0.17.1**, the latest stable [Zig](https://ziglang.org/download/) release, including its formatter:

```sh
mkdir -p build/tools "$HOME/.local/opt"
curl -fL https://ziglang.org/download/0.17.1/zig-x86_64-linux-0.17.1.tar.xz -o build/tools/zig.tar.xz
tar -xJf build/tools/zig.tar.xz -C "$HOME/.local/opt"

rcfile=~/.bashrc # ~/.zshrc
echo 'export PATH="$HOME/.local/opt/zig-x86_64-linux-0.17.1:$PATH"' >> $rcfile
source $rcfile
```


### Elixir And Erlang

Install the latest Elixir and Erlang/OTP available in your distribution's normal
repositories, following [Elixir's package installation options](https://elixir-lang.org/install/):

```sh
# Debian/Ubuntu
sudo add-apt-repository ppa:rabbitmq/rabbitmq-erlang
sudo apt update
sudo apt install -y elixir erlang
# Fedora
sudo dnf install -y elixir erlang elixir-doc erlang-doc
```


### Project Packages

Rust tasks download locked dependencies automatically. For Python, select the
interpreter and install the locked dependencies (including PuLP):

```sh
poetry env use python3.15
poetry install
```

This synchronises the environment without upgrading the lockfile. Zig builds
download the set and regex dependencies into `build/zig/packages/` when missing.
Their archive URLs and content hashes are recorded in `build.zig.zon`.

## Update

Run from the repository root. Package upgrades below update manifest constraints
and lockfiles, including new major releases that may require source changes.

### Common

Update system packages, including Python, Poetry, Ruff, mypy, Elixir, Erlang and Task:

```sh
# Debian/Ubuntu
sudo apt update
sudo apt upgrade
# Fedora
sudo dnf upgrade --refresh
```

### Rust

Update the compiler and components, then project requirements and locked packages.
`cargo-edit` provides the `cargo upgrade` command:

```sh
rustup update stable
cargo install cargo-edit --locked       # one-time helper installation
cargo upgrade --incompatible allow      # update Cargo.toml, including major versions
cargo update                           # update Cargo.lock
```

### Python

System tools are updated by apt/dnf above. Update the project's Python interpreter
selection, dependency requirements and lockfile separately:

```sh
poetry env use python3.15
poetry add pulp@latest
poetry add --group dev ruff@latest mypy@latest
poetry update
```

These Ruff/mypy packages belong to the project environment; checks currently use
the globally installed commands. `poetry update` alone respects existing version
constraints; `@latest` replaces them.

### Go

Update the manually installed compiler using the Go installation instructions
with the newest release archive, then update PATH. Record the active compiler
version in `go.mod`:

```sh
go version
go mod edit -go="$(go env GOVERSION | sed 's/^go//')"
go mod tidy
```

There are currently no external Go packages.

### Zig

Update the compiler using the Zig installation instructions with the newest
release archive, then update PATH. The project targets Zig 0.17.1.
Update the set dependency's pinned URL/hash in `build.zig.zon` with
`zig fetch --pkg-dir build/zig/packages --save=ziglangSet ARCHIVE_URL`.
For regex updates, remove its hash in `build.zig.zon`, build to fetch the current
`main` archive, and record the hash reported by Zig. Build and verify answers
after either dependency changes:

```sh
task build LANG=zig YEAR=2024
task verify LANG=zig YEAR=2024 DAY=1 PART=1
```

Real-input verification requires the matching downloaded input and your session
cookie; sample verification uses only the local sample files.

### Elixir And Erlang

The apt/dnf update above updates Elixir, Erlang and Mix. These scripts have no
external packages or `mix.lock` to update.

After updating, check executable availability and review dependency changes:

```sh
task doctor
git diff -- Cargo.toml Cargo.lock pyproject.toml poetry.lock go.mod build.zig.zon
```

## Usage

Helper scripts use `python3` without Poetry; override it with `PYTHON` if needed.
Python solutions require Python 3.15 in Poetry's project environment. The Python
build selects `python3.15` and installs dependencies if that environment is missing
or uses a different interpreter. Runs use the selected environment's Python
executable. `doctor` checks both the helper interpreter and, for Python, the
project environment. Ruff and mypy continue to run globally from PATH.

Task sets `PYTHONPATH` to `src/`, and VS Code uses the same import root. Python
entry points run as files, so shared imports use `from lib.python import ...`
and day imports use `from solutions.day01 import Solution01`. For a direct run:

```sh
PYTHONPATH=src poetry run python3.15 src/2023/python/main.py 1 1 --sample
```

```sh
task doctor                              # all tools and the Python 3.15 project environment
task doctor LANG=rust                    # common tools and Rust
task --list
task list
task build LANG=zig YEAR=2024
task run LANG=elixir YEAR=2025
task run LANG=python YEAR=2023 DAY=1 PART=1 SAMPLE=true
task verify LANG=zig YEAR=2024 DAY=1 PART=1 # compare with your accepted answer
task check LANG=python                   # formats, lints and checks types
task check LANG=golang YEAR=2015          # formats, vets and tests
task new:day LANG=python YEAR=2025 DAY=11
task input YEAR=2025 DAY=3                # omit DAY for all missing days
```

Languages: `rust`, `python`, `golang`, `zig`, `elixir`. For Rust, Python, Zig
and Elixir, Task builds before calling `scripts/run.sh`. Omit `DAY` to try days
1-25; omit `PART` for both parts. Real input is the default; use `SAMPLE=true`
for samples. Answers go directly to stdout, and errors go to stderr. Batch runs
continue after failures and exit nonzero if any case fails.

Verification runs exactly one day/part and compares its stdout with an expected
answer:

```sh
task verify LANG=python YEAR=2023 DAY=1 PART=1 SAMPLE=true
task verify LANG=rust YEAR=2022 DAY=1 PART=1
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

Language entry points accept `DAY PART [--sample]`, print one answer followed by
one newline and report errors on stderr. Exit codes are `0` for success, `1`
for execution/verification failures and `2` for invalid arguments; crashes may
use other nonzero codes.

Scripts use `LANG YEAR DAY PART` order where applicable; downloads need only
`YEAR DAY`. Direct calls expect an existing build:

```sh
task build LANG=zig YEAR=2024
bash scripts/run.sh zig 2024                         # full year, real input
bash scripts/run.sh zig 2024 2 --sample              # both parts of sample day 2
python3 scripts/verify.py zig 2024 2 1 --sample       # verify one case
```

Empty positional arguments allow a part-only filter, e.g.
`bash scripts/run.sh zig 2024 "" 1 --sample`. Task sets the Python import and
bytecode cache paths; direct Python runs need `PYTHONPATH=src` and may set
`PYTHONPYCACHEPREFIX=build/python/pycache` to keep bytecode under `build/`.

Go requires `DAY` and retains each program's fixed part/input settings; it does
not use the shared runner yet. `check` formats files and accepts an optional
`YEAR`.

Downloads read the session token from ignored `scripts/aoc_cookie.txt`. Put the
token alone in that file, or run `task cookie` after logging in through Firefox.
The importer supports native and Flatpak Firefox profiles. Inputs are named
`inputDD.txt` and `sampleDD.txt`, with `_1.txt` / `_2.txt` part-specific fallbacks.

## Files And Local Output

| Owner | Root configuration | Generated/local output |
| --- | --- | --- |
| Common | `Taskfile.yml`, `.gitignore`, `.vscode/settings.json` | `.git/`, editor state, accepted-answer cache under `build/answers/` |
| Rust | `Cargo.toml`, `Cargo.lock`, `.cargo/config.toml`, `.rustfmt.toml` | `build/rust/` |
| Python | `pyproject.toml`, `poetry.lock`, `poetry.toml` | `build/python/`: environments, Ruff/mypy caches and bytecode |
| Go | `go.mod` | `build/golang/`: binaries and compilation cache; temporary builds use system temp |
| Zig | `build.zig`, `build.zig.zon` | `build/zig/`: binaries and compiler caches |
| Elixir | None (scripts) | Mix paths reserved under `build/elixir/` |

`build/` is ignored. System prerequisites and Task use apt/dnf; toolchains follow
the locations above.
Cargo and Poetry also maintain dependency/download caches in your home directory.
Task keeps Zig package and compiler caches under `build/zig/` and configures build
paths; direct native commands may need the same
flags/environment variables. VS Code uses `.vscode/settings.json` for editing defaults.

## Adding Years

Create `inputs/YEAR/` and `src/YEAR/LANGUAGE/` manually, using an existing year's
entry point as a reference. Set its runner's year and keep the `aoc:*` registration
markers so `task new:day` can insert imports and solutions from `scripts/templates/`.
Rust, Zig and Elixir days must be added in order without gaps.

| Language | Entry point and references to add |
| --- | --- |
| Rust | `main.rs`, `solutions/mod.rs`; add a `[[bin]]` in root `Cargo.toml`: `name = "mainYEAR"`, `path = "src/YEAR/rust/main.rs"` |
| Python | `main.py`, `__init__.py`, `solutions/__init__.py`; no root registration |
| Go | `dayDD/main.go` packages under the existing root module; no yearly entry point |
| Zig | `main.zig`; add `add_binary(b, "YEAR");` in root `build.zig` |
| Elixir | `main.exs` loading shared modules and days; no root registration |

Put the year's README in `src/YEAR/README.md`. Link screenshots with
`../../progress/YEAR/overview.png` and `../../progress/YEAR/conclusion.png`.
Create `inputs/YEAR/sample_solutions.json` for sample expectations.
