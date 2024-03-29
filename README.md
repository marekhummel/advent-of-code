# Advent Of Code
Advent Of Code solutions.
Contains solutions for all years from 2015 to 2023, all of them solved in Rust. 
Addionally, some extra implementations are there in python and golang.

To run, check the main files per year in each languages' directory. There are general entry points for each year in Rust, for 2023 there is also one for Python. Moreover, there can be single day solutions in any other language, like 2015-07 in Golang which has to be called directly.

Run them and pass either `dayXX` as an argument or `main` to run all of them.
- Rust: `cargo run --release --bin mainYYYY -- dayXX` (or `main` instead of the day)
- Python: `python -u ./YYYY/python/main.py dayXX`

Within the main files there are three config flags which are relevant when running a single day. Use `VERSION` to select part 1 or 2 and `USE_SAMPLE` to choose between sample or real input. Set `ALL` to true to run through all 4 results of that day.

### Scripts
For development, there are two scripts to create rust template files for a new [day](./scripts/new_day.sh) (pass year and day as args) or the directory structure for a new [year](./scripts/new_year.sh).

Moreover, a script to [download all inputs](./scripts/download_inputs.sh) for a given year, or just a specific day if given two args (year day).

Lastly, there is a script to [verify solutions](./scripts/verify_results.sh). 
Basically it checks the results mentioned in the file for unit tests with the solutions submitted to the platform.

Since input files are different for each user, make sure to create a file `./scripts/aoc_cookie.txt` which contains your session cookie, before running the scripts. Check your browser's storage for that. 


### Total Progress
![Total Progress](progress.png)