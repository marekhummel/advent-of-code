const std = @import("std");
const types = @import("types.zig");

pub const Solution = struct {
    pub fn solve(
        self: *const Solution,
        data: *types.ProblemInput,
        part: u8,
        is_sample: bool,
        allocator: std.mem.Allocator,
    ) !types.TimedResult {
        const start_time = std.time.nanoTimestamp();
        const result = try switch (part) {
            1 => self.solvePart01(data, is_sample, allocator) catch types.SolvingError.SolvingFailed,
            2 => self.solvePart02(data, is_sample, allocator) catch types.SolvingError.SolvingFailed,
            else => types.SolvingError.InvalidPart,
        };
        const end_time = std.time.nanoTimestamp();

        return types.TimedResult.create(result, start_time, end_time);
    }

    results: *const fn () [4]types.Result,
    solvePart01: *const fn (*types.ProblemInput, bool, std.mem.Allocator) anyerror!types.Result,
    solvePart02: *const fn (*types.ProblemInput, bool, std.mem.Allocator) anyerror!types.Result,
};

pub fn makeSolution(comptime day: anytype) Solution {
    return Solution{
        .results = day.results,
        .solvePart01 = day.solvePart01,
        .solvePart02 = day.solvePart02,
    };
}
