#!/usr/bin/env bash
# Run one case, a whole day or a whole year. Answers go directly to stdout.
set -euo pipefail
cd "$(dirname "$0")/.."

# Parse args
if (( $# < 2 )); then
    echo 'Usage: run.sh LANG YEAR [DAY [PART]] [--sample]' >&2
    exit 2
fi
language=$1
year=$2
shift 2
positionals=()
options=()
while (( $# )); do
    case "$1" in
        --sample) options=(--sample) ;;
        --*) echo "Unknown option: $1" >&2; exit 2 ;;
        *) positionals+=("$1") ;;
    esac
    shift
done
if (( ${#positionals[@]} > 2 )); then
    echo 'Expected at most DAY and PART after LANG YEAR' >&2
    exit 2
fi
day=${positionals[0]:-}
part=${positionals[1]:-}

case "$language" in
    rust) runner=("$PWD/build/rust/release/main$year") ;;
    python) runner=("$(poetry env info --executable)" "src/$year/python/main.py") ;;
    golang) runner=("$PWD/build/golang/bin/main$year") ;;
    zig) runner=("$PWD/build/zig/out/bin/main$year") ;;
    elixir) runner=(elixir "src/$year/elixir/main.exs") ;;
    *) echo "Unsupported language: $language" >&2; exit 2 ;;
esac

# Run the cases
days=({01..25})
parts=(1 2)
[[ -z $day ]] || days=("$day")
[[ -z $part ]] || parts=("$part")
failed=0
for day in "${days[@]}"; do
    for part in "${parts[@]}"; do
        if "${runner[@]}" "$day" "$part" "${options[@]}" < /dev/null; then
            continue
        else
            code=$?
            printf '%s/%s day %s part %s failed (exit %s)\n' "$language" "$year" "$day" "$part" "$code" >&2
            failed=1
        fi
    done
done
exit "$failed"
