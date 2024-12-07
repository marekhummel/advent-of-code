const std = @import("std");
const aoc_lib = @import("aoc_lib");

const day01 = @import("solutions/day01.zig");
const day02 = @import("solutions/day02.zig");
const day03 = @import("solutions/day03.zig");
const day04 = @import("solutions/day04.zig");
const day05 = @import("solutions/day05.zig");
const day06 = @import("solutions/day06.zig");
const day07 = @import("solutions/day07.zig");

const ALL: bool = true;
const VERSION: u8 = 1;
const USE_SAMPLE: bool = true;

// Should usually be yes, but disable to catch "missed opportunities" for early frees
const USE_ARENA: bool = true;

pub fn main() !void {
    var runner = create_runner();
    try runner.run(ALL, VERSION, USE_SAMPLE);
}

fn create_runner() aoc_lib.runner.AocRunner {
    const solutions = [_]?aoc_lib.solution.Solution{
        aoc_lib.solution.makeSolution(day01),
        aoc_lib.solution.makeSolution(day02),
        aoc_lib.solution.makeSolution(day03),
        aoc_lib.solution.makeSolution(day04),
        aoc_lib.solution.makeSolution(day05),
        aoc_lib.solution.makeSolution(day06),
        aoc_lib.solution.makeSolution(day07),
    };
    return aoc_lib.runner.AocRunner.init(2024, &solutions, USE_ARENA);
}

test "implementations match solutions" {
    var runner = create_runner();
    runner._allocator = std.testing.allocator;
    try std.testing.expect(runner.verifySolutions());
}
