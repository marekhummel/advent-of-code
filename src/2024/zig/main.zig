const std = @import("std");
const aoc_lib = @import("aoc_lib");
const build_options = @import("build_options");

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
    const days = .{
        .{ 1, day01 },
        .{ 2, day02 },
        .{ 3, day03 },
        .{ 4, day04 },
        .{ 5, day05 },
        .{ 6, day06 },
        .{ 7, day07 },
        .{ 8, day08 },
        .{ 9, day09 },
        .{ 10, day10 },
        .{ 11, day11 },
        .{ 12, day12 },
        .{ 13, day13 },
        .{ 14, day14 },
        .{ 15, day15 },
        .{ 16, day16 },
        .{ 17, day17 },
        .{ 18, day18 },
        .{ 19, day19 },
        .{ 20, day20 },
        .{ 21, day21 },
        .{ 22, day22 },
        .{ 23, day23 },
        .{ 24, day24 },
        .{ 25, day25 },
        // aoc:solutions
    };
    const implementations = comptime blk: {
        var selected: [days.len]struct { u8, aoc_lib.solution.Solution } = undefined;
        var count: usize = 0;
        for (days) |entry| {
            if (build_options.day == null or build_options.day.? == entry[0]) {
                selected[count] = .{ entry[0], aoc_lib.solution.makeSolution(entry[1]) };
                count += 1;
            }
        }
        break :blk selected[0..count].*;
    };
    return aoc_lib.runner.AocRunner.init(allocator, 2024, &implementations);
}
