#pragma once

#include <cstdint>
#include <string>
#include <unordered_map>
#include <variant>
#include <vector>

namespace aoc {

using Input = std::vector<std::string>;
using Result = std::variant<std::int64_t, std::string>;
using Part = Result (*)(const Input &, bool);

inline const std::string NoSample = "<No Sample Defined>";
inline const std::string NoPartTwo = "<No Part Two>";
inline const std::string NotImplemented = "<Not Implemented>";
inline const std::string Unsolved = "<Unsolved>";

struct Solution {
    Part part1 = nullptr;
    Part part2 = nullptr;
};

// Run DAY PART MODE (real|sample). Missing days/parts succeed; errors go to stderr.
int run(int argc, char *argv[], int year, const std::unordered_map<int, Solution> &solutions);

} // namespace aoc
