#!/usr/bin/env bash
# Run one case, a whole day or a whole year. Labels go to stderr, answers to stdout.
set -euo pipefail
cd "$(dirname "$0")/.."

# Get CLI args
language=
year=
day=
part=
mode=both
optimize=fast
while (( $# )); do
    case "$1" in
        --lang) language=$2 ;;
        --year) year=$2 ;;
        --day) day=$2 ;;
        --part) part=$2 ;;
        --mode) mode=$2 ;;
        --optimize) optimize=$2 ;;
        *) echo "Unknown option: $1" >&2; exit 2 ;;
    esac
    shift 2
done

# Build executable if needed
task --exit-code "build:$language" "YEAR=$year" "OPTIMIZE=$optimize"

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
            setsid task --silent --exit-code "case:$language" "YEAR=$year" \
                "DAY=$day" "PART=$part" "MODE=$mode" "OPTIMIZE=$optimize" \
                < /dev/null > "$output_file" 2> "$error_file" &
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
