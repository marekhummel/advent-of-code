#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

# Arguments: YEAR and optional DAY.
year=${1:?Usage: download_inputs.sh YEAR [DAY]}
day=${2:-}
[[ -z $day ]] || day=$((10#$day))
last=25
(( year <= 2024 )) || last=12

[[ -r scripts/aoc_cookie.txt && -s scripts/aoc_cookie.txt ]] || {
    echo 'Put your session token in scripts/aoc_cookie.txt, or run task cookie' >&2; exit 1;
}
session=$(<scripts/aoc_cookie.txt)
# Quotes or newlines would change the meaning of the curl config below.
[[ $session =~ ^[a-zA-Z0-9_-]+$ ]] || { echo 'Invalid session credential' >&2; exit 1; }

mkdir -p "inputs/$year"
for (( current=${day:-1}; current<=${day:-last}; current++ )); do
    printf -v destination 'inputs/%s/input%02d.txt' "$year" "$current"
    if [[ -e $destination || -L $destination ]]; then
        [[ -z $day ]] || { echo "Refusing to overwrite $destination" >&2; exit 1; }
        echo "exists: $destination"
        continue
    fi
    # --config - reads config from stdin; the secret stays out of curl's arguments.
    printf 'header = "Cookie: session=%s"\n' "$session" |
        curl --disable --config - --fail --remove-on-error --silent --show-error \
            --max-time 30 --output "$destination" \
            "https://adventofcode.com/$year/day/$current/input"
    echo "downloaded: $destination"
done
