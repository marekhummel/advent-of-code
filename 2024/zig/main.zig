const std = @import("std");
const aoc_lib = @import("aoc_lib");

const day01 = @import("solutions/day01.zig");

const ALL: bool = true;
const VERSION: u8 = 1;
const USE_SAMPLE: bool = false;

pub fn main() !void {
    var runner = create_runner();
    try runner.run(ALL, VERSION, USE_SAMPLE);
}

fn create_runner() aoc_lib.runner.AocRunner {
    const solutions = [_]?aoc_lib.solution.Solution{
        aoc_lib.solution.makeSolution(day01),
    };
    return aoc_lib.runner.AocRunner.init(2024, &solutions);
}

test "implementations match solutions" {
    var runner = create_runner();
    runner._allocator = std.testing.allocator;
    try std.testing.expect(runner.verifySolutions());
}