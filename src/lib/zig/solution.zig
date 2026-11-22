const std = @import("std");
const types = @import("types.zig");

pub const Part = *const fn (*types.ProblemInput, bool, std.mem.Allocator) anyerror!types.Result;

pub const Solution = struct {
    part1: Part,
    part2: Part,

    pub fn from(comptime day: type) Solution {
        return .{ .part1 = day.part1, .part2 = day.part2 };
    }
};
