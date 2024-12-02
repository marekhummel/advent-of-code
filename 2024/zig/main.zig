const std = @import("std");
const aoc_lib = @import("aoc_lib");

const day01 = @import("solutions/day01.zig");
const day02 = @import("solutions/day02.zig");

const ALL: bool = true;
const VERSION: u8 = 1;
const USE_SAMPLE: bool = true;

pub fn main() !void {
    var runner = create_runner();
    try runner.run(ALL, VERSION, USE_SAMPLE);
}

fn create_runner() aoc_lib.runner.AocRunner {
    const solutions = [_]?aoc_lib.solution.Solution{
        aoc_lib.solution.makeSolution(day01),
        aoc_lib.solution.makeSolution(day02),
    };
    return aoc_lib.runner.AocRunner.init(2024, &solutions);
}

test "implementations match solutions" {
    var runner = create_runner();
    runner._allocator = std.testing.allocator;
    try std.testing.expect(runner.verifySolutions());
}
