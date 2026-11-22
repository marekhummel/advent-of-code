#include "runner.hpp"
#include "solutions/day01.hpp"
// aoc:imports

int main(int argc, char* argv[]) {
    return aoc::run(argc, argv, 2026, {
        {1, AOC_SOLUTION(day01)},
        // aoc:solutions
    });
}
