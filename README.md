# Advent Of Code
Advent Of Code solutions.
Contains solutions for all years from 2015 to 2025, most of them solved in Rust (2015 - 2023).
Each year has a main language (2015-2023 in Rust, 2024 in zig, 2025 in elixir).
Additionally, there are single day implementations in python (2023 and 2025), golang (2015-07, 2016-10, 2024-24).

Note that for rust, zig, python and elixir it made sense to implement a library for reusage of code and structures and what not. Nonetheless the important part of each day should still be in the respective daily file. Moreover, the daily files contain the solution values for that day, so that unit tests can check if some code broke a specific day.

### Run
To run solutions, use the main files in each language's directory. Pass `main` to run all days, or specify a day with options.

**Commands:**
- Rust: `cargo run --release --bin mainYYYY -- [main | day N --part <1|2> [--sample] [--all]]`
- Python: `python -m 2023.python.main [main | day N --part <1|2> [--sample] [--all]]`
- Zig: `zig build -Doptimize=ReleaseFast 2024_main && ./zig-out/bin/main2024.exe [main | day N --part <1|2> [--sample] [--all]]`
- Elixir: `elixir 2025/elixir/main.exs [main | day N --part <1|2> [--sample] [--all]]`

**Options:**
- `--part <1|2>`: Run specific part (required unless `--all`)
- `--sample`: Use sample input instead of real input
- `--all`: Run both parts (overrides `--part`)

Golang solutions are single-day only and must be run directly.

### Scripts
For development, there are two scripts to create rust, zig, or elixir template files for a new [day](./scripts/new_day.sh) (pass year, day and lang as args) or the directory structure for a new [year](./scripts/new_year.sh) (rust only).

Moreover, a script to [download all inputs](./scripts/download_inputs.sh) for a given year, or just a specific day if given two args (year day).

It makes sense to use [pre-commit.py](./scripts/pre-commit.py) as a precomit hook, which runs all unit tests for the days in rust in the stating area prior to commiting. Zig has unit tests as well, but only one per year and currently has to be invoked manually with `zig test 2024_test`.

Lastly, there is a script to [verify solutions](./scripts/verify_results.sh).
Basically it checks the results mentioned in the rust file for unit tests with the solutions submitted to the platform.

Since input files are different for each user, make sure to create a file `./scripts/aoc_cookie.txt` which contains your session cookie, before running the scripts. Check your browser's storage for that.


### Total Progress
![Total Progress](progress.png)
