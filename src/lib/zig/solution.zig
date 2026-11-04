const std = @import("std");
const types = @import("types.zig");

pub const Solution = struct {
    solvePart01: *const fn (*types.ProblemInput, bool, std.mem.Allocator) anyerror!types.Result,
    solvePart02: *const fn (*types.ProblemInput, bool, std.mem.Allocator) anyerror!types.Result,
};

pub fn makeSolution(comptime day: anytype) Solution {
    return .{
        .solvePart01 = day.solvePart01,
        .solvePart02 = day.solvePart02,
    };
}
