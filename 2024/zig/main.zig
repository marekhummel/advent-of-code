const std = @import("std");
const aoc_lib = @import("aoc_lib");

const day01 = @import("solutions/day01.zig");
const day02 = @import("solutions/day02.zig");
const day03 = @import("solutions/day03.zig");
const day04 = @import("solutions/day04.zig");
const day05 = @import("solutions/day05.zig");
const day06 = @import("solutions/day06.zig");
const day07 = @import("solutions/day07.zig");
const day08 = @import("solutions/day08.zig");
const day09 = @import("solutions/day09.zig");
const day10 = @import("solutions/day10.zig");
const day11 = @import("solutions/day11.zig");
const day12 = @import("solutions/day12.zig");
const day13 = @import("solutions/day13.zig");
const day14 = @import("solutions/day14.zig");
const day15 = @import("solutions/day15.zig");
const day16 = @import("solutions/day16.zig");
const day17 = @import("solutions/day17.zig");
const day18 = @import("solutions/day18.zig");
const day19 = @import("solutions/day19.zig");
const day20 = @import("solutions/day20.zig");
const day21 = @import("solutions/day21.zig");
const day22 = @import("solutions/day22.zig");
const day23 = @import("solutions/day23.zig");
const day24 = @import("solutions/day24.zig");
const day25 = @import("solutions/day25.zig");

const ALL: bool = true;
const PART: u8 = 1;
const USE_SAMPLE: bool = true;

// Should usually be yes, but disable to catch "missed opportunities" for early frees
const USE_ARENA: bool = true;

pub fn main() !void {
    var runner = createRunner();
    try runner.run(ALL, PART, USE_SAMPLE);
}

fn createRunner() aoc_lib.runner.AocRunner {
    const solutions = [_]?aoc_lib.solution.Solution{
        aoc_lib.solution.makeSolution(day01),
        aoc_lib.solution.makeSolution(day02),
        aoc_lib.solution.makeSolution(day03),
        aoc_lib.solution.makeSolution(day04),
        aoc_lib.solution.makeSolution(day05),
        aoc_lib.solution.makeSolution(day06),
        aoc_lib.solution.makeSolution(day07),
        aoc_lib.solution.makeSolution(day08),
        aoc_lib.solution.makeSolution(day09),
        aoc_lib.solution.makeSolution(day10),
        aoc_lib.solution.makeSolution(day11),
        aoc_lib.solution.makeSolution(day12),
        aoc_lib.solution.makeSolution(day13),
        aoc_lib.solution.makeSolution(day14),
        aoc_lib.solution.makeSolution(day15),
        aoc_lib.solution.makeSolution(day16),
        aoc_lib.solution.makeSolution(day17),
        aoc_lib.solution.makeSolution(day18),
        aoc_lib.solution.makeSolution(day19),
        aoc_lib.solution.makeSolution(day20),
        aoc_lib.solution.makeSolution(day21),
        aoc_lib.solution.makeSolution(day22),
        aoc_lib.solution.makeSolution(day23),
        aoc_lib.solution.makeSolution(day24),
        aoc_lib.solution.makeSolution(day25),
    };
    return aoc_lib.runner.AocRunner.init(2024, &solutions, USE_ARENA);
}

test "implementations match solutions" {
    var runner = createRunner();
    runner._allocator = std.testing.allocator;
    try std.testing.expect(runner.verifySolutions());
}
