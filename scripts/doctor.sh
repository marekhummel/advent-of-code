#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

language=${1:-}
python=${2:-python3}
case "$language" in ''|rust|python|golang|zig|elixir) ;; *) echo "Unknown language: $language" >&2; exit 1;; esac

failed=0
while IFS='|' read -r group binary flag; do
    [[ $group == common || -z $language || $group == "$language" ]] || continue
    executable=$binary
    [[ $binary != python3 ]] || executable=$python
    location=$(command -v "$executable") || location=''
    if [[ ! -x $location ]]; then
        echo "⚠️  missing   $executable"
        failed=1
    elif [[ $flag == none ]]; then
        echo "✅ installed $executable: $location"
    # Some tools (including Elixir) read stdin even for version requests.
    elif version=$("$location" "$flag" </dev/null 2>&1); then
        case "$binary" in
            elixir|mix) version=${version##*$'\n'} ;;
            *) version=${version%%$'\n'*} ;;
        esac
        echo "✅ installed $executable: ${version//$'\r'/} ($location)"
    else
        echo "❌ broken    $executable: $version ($location)"
        failed=1
    fi
done <<'TOOLS'
common|task|--version
common|bash|--version
common|curl|--version
common|python3|--version
rust|rustup|--version
rust|cargo|--version
rust|rustc|--version
rust|rustfmt|--version
rust|cargo-clippy|--version
rust|cc|--version
python|poetry|--version
python|python3.15|--version
python|ruff|--version
python|mypy|--version
golang|go|version
golang|gofmt|none
zig|zig|version
elixir|elixir|--version
elixir|erl|-version
elixir|mix|--version
TOOLS
if [[ -z $language || $language == python ]] && command -v poetry >/dev/null; then
    if interpreter=$(poetry env info --executable 2>/dev/null) && [[ -x $interpreter ]]; then
        if version=$("$interpreter" -c 'import sys; print(sys.version.split()[0]); sys.exit(sys.version_info[:2] != (3, 15) or sys.prefix == sys.base_prefix)' </dev/null 2>&1); then
            echo "✅ installed Python venv: $version ($interpreter)"
        else
            echo "❌ broken    Python venv: $version ($interpreter)"
            failed=1
        fi
    else
        echo "⚠️  missing   Python venv"
        failed=1
    fi
fi
exit "$failed"
