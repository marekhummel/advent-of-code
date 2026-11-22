const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Solution = aoc_lib.solution.Solution;

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
// aoc:imports

pub fn main(init: std.process.Init) !void {
    var runner = try createRunner(init.arena.allocator());
    defer runner.solutions.deinit();
    try runner.parseAndRun(init);
}

fn createRunner(allocator: std.mem.Allocator) !aoc_lib.runner.AocRunner {
    return aoc_lib.runner.AocRunner.init(allocator, 2024, &.{
        .{ 1, Solution.from(day01) },
        .{ 2, Solution.from(day02) },
        .{ 3, Solution.from(day03) },
        .{ 4, Solution.from(day04) },
        .{ 5, Solution.from(day05) },
        .{ 6, Solution.from(day06) },
        .{ 7, Solution.from(day07) },
        .{ 8, Solution.from(day08) },
        .{ 9, Solution.from(day09) },
        .{ 10, Solution.from(day10) },
        .{ 11, Solution.from(day11) },
        .{ 12, Solution.from(day12) },
        .{ 13, Solution.from(day13) },
        .{ 14, Solution.from(day14) },
        .{ 15, Solution.from(day15) },
        .{ 16, Solution.from(day16) },
        .{ 17, Solution.from(day17) },
        .{ 18, Solution.from(day18) },
        .{ 19, Solution.from(day19) },
        .{ 20, Solution.from(day20) },
        .{ 21, Solution.from(day21) },
        .{ 22, Solution.from(day22) },
        .{ 23, Solution.from(day23) },
        .{ 24, Solution.from(day24) },
        .{ 25, Solution.from(day25) },
        // aoc:solutions
    });
}
