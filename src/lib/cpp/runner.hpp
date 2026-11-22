#pragma once

#include <cstdint>
#include <iosfwd>
#include <string>
#include <unordered_map>
#include <variant>
#include <vector>

namespace aoc {

using Input = std::vector<std::string>;
enum class Status { NoSample, NoPartTwo, NotImplemented, Unsolved };
using Signed = std::intmax_t;
using Unsigned = std::uintmax_t;
using Result = std::variant<Signed, Unsigned, std::string, Status>;
using Part = Result (*)(const Input &, bool);

struct Solution {
    Part part1;
    Part part2;

    Solution(Part first, Part second) : part1(first), part2(second) {}
};

std::ostream &operator<<(std::ostream &output, Status status);

// Run DAY PART MODE (real|sample). Unregistered/unfinished solutions succeed; errors go to stderr.
int run(int argc, char *argv[], int year, const std::unordered_map<int, Solution> &solutions);

} // namespace aoc

#define AOC_SOLUTION(day) ::aoc::Solution{day::part1, day::part2}
