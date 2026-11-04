#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

language=${1:-}
python=${2:-python3}
case "$language" in ''|rust|python|golang|zig|elixir) ;; *) echo "Unknown language: $language" >&2; exit 1;; esac

failed=0
while IFS='|' read -r group binary flag guidance; do
    [[ $group == common || -z $language || $group == "$language" ]] || continue
    executable=$binary
    [[ $binary != python3 ]] || executable=$python
    location=$(command -v "$executable") || location=''
    if [[ ! -x $location ]]; then
        echo "⚠️  missing   $binary: $guidance"
        failed=1
    elif [[ $flag == none ]]; then
        echo "✅ installed $binary: $location"
    elif version=$("$location" "$flag" 2>&1); then
        echo "✅ installed $binary: ${version//$'\n'/; } ($location)"
    else
        echo "❌ broken    $binary: $version ($location); $guidance"
        failed=1
    fi
done <<'TOOLS'
common|task|--version|Task 3: https://taskfile.dev/docs/installation
common|python3|--version|Python 3.12+, selected with PYTHON
rust|cargo|--version|Stable Rust: https://rustup.rs; cargo fetch
rust|rustc|--version|Included with Rust
rust|rustfmt|--version|rustup component add rustfmt
rust|cargo-clippy|--version|rustup component add clippy
python|poetry|--version|https://python-poetry.org/docs/
python|ruff|--version|pip install ruff
python|mypy|--version|pip install mypy
golang|go|version|Go >=1.23.4: https://go.dev/dl/
golang|gofmt|none|Included with Go
zig|zig|version|Zig 0.13.x: https://ziglang.org/download/
elixir|elixir|--version|Elixir 1.19+: https://elixir-lang.org/install.html
elixir|erl|-version|Compatible Erlang/OTP
elixir|mix|--version|Included with Elixir
TOOLS
exit "$failed"
