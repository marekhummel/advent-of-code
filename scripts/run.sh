#!/usr/bin/env bash
# Run one case, a whole day or a whole year. Labels go to stderr, answers to stdout.
set -euo pipefail
cd "$(dirname "$0")/.."

# Parse args
if (( $# < 2 )); then
    echo 'Usage: run.sh LANG YEAR [DAY [PART]] [real|sample|both] (default: both)' >&2
    exit 2
fi
language=$1
year=$2
shift 2
positionals=()
mode=both
while (( $# )); do
    case "$1" in
        real|sample|both) mode=$1 ;;
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

# Define runner
case "$language" in
    rust) runner=("$PWD/build/rust/release/main$year") ;;
    python) runner=(python3.15 "src/$year/python/main.py") ;;
    golang) runner=("$PWD/build/golang/bin/main$year") ;;
    zig) runner=("$PWD/build/zig/out/bin/main$year") ;;
    elixir) runner=(elixir -pa "build/elixir/$year" -e 'Main.main(System.argv())' --) ;;
    *) echo "Unsupported language: $language" >&2; exit 2 ;;
esac

# Validate runner exists
case "$language" in
    python) main_file=${runner[1]} ;;
    elixir) main_file="build/elixir/$year/Elixir.Main.beam" ;;
    *) main_file=${runner[0]} ;;
esac
if [[ ! -f $main_file ]]; then
    echo "Missing runner: $main_file" >&2
    exit 1
fi

# Prep the cases
output_file=$(mktemp)
error_file=$(mktemp)
trap 'rm -f "$output_file" "$error_file"' EXIT
runner_pid=
cancel() {
    trap '' INT TERM
    if [[ -n $runner_pid ]]; then
        kill -TERM -- "-$runner_pid" 2>/dev/null || kill -TERM "$runner_pid" 2>/dev/null || true
        wait "$runner_pid" 2>/dev/null || true
    fi
    exit "$1"
}
trap 'cancel 130' INT
trap 'cancel 143' TERM

# Prep loop variables
modes=(sample real)
[[ $mode == both ]] || modes=("$mode")
days=({01..25})
(( year < 2025 )) || days=({01..12})
parts=(1 2)
[[ -z $day ]] || days=("$day")
[[ -z $part ]] || parts=("$part")

# Run the cases
failed=0
for day in "${days[@]}"; do
    for part in "${parts[@]}"; do
        for mode in "${modes[@]}"; do
            label=REAL
            [[ $mode != sample ]] || label=SAMP
            start=${EPOCHREALTIME/./}
            # A separate process group lets cancellation stop the runner and its children.
            setsid "${runner[@]}" "$day" "$part" "$mode" < /dev/null > "$output_file" 2> "$error_file" &
            runner_pid=$!
            if wait "$runner_pid"; then
                code=0
            else
                code=$?
            fi
            runner_pid=
            case "$code" in
                130|143) exit "$code" ;;
                0) ;;
                *) failed=1 ;;
            esac
            elapsed=$(( ${EPOCHREALTIME/./} - start ))
            printf 'D%02d P%s %s (%2d.%05d secs): ' \
                "$((10#$day))" "$part" "$label" \
                "$((elapsed / 1000000))" "$((elapsed % 1000000 / 10))" >&2
            cat "$error_file" >&2
            cat "$output_file"
        done
    done
done
exit "$failed"
