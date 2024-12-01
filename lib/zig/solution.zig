const std = @import("std");
const types = @import("types.zig");

// pub fn Solution(comptime day: anytype) type {
//     return struct {
//         pub fn solve(data: types.ProblemInput, version: u8, is_sample: bool) !types.TimedResult {
//             const start_time = std.time.nanoTimestamp();
//             const result = try switch (version) {
//                 1 => solve_version01(data, is_sample),
//                 2 => solve_version02(data, is_sample),
//                 // 2 => try day2.solve(),
//                 // Add more days as needed
//                 else => types.SolvingError.InvalidVersion,
//             };
//             const end_time = std.time.nanoTimestamp();

//             return types.TimedResult.create(result, start_time, end_time);
//         }

//         pub fn results() [4]types.Result {
//             day.results();
//         }

//         fn solve_version01(input: types.ProblemInput, is_sample: bool) types.Result {
//             return day.solve_version01(input, is_sample);
//         }
//         fn solve_version02(input: types.ProblemInput, is_sample: bool) types.Result {
//             return day.solve_version02(input, is_sample);
//         }
//     };
// }

pub const Solution = struct {
    pub fn solve(self: *const Solution, allocator: std.mem.Allocator, data: types.ProblemInput, version: u8, is_sample: bool) !types.TimedResult {
        const start_time = std.time.nanoTimestamp();
        const result = try switch (version) {
            1 => self.solve_version01(allocator, data, is_sample) catch types.SolvingError.SolvingFailed,
            2 => self.solve_version02(allocator, data, is_sample) catch types.SolvingError.SolvingFailed,
            // 2 => try day2.solve(),
            // Add more days as needed
            else => types.SolvingError.InvalidVersion,
        };
        const end_time = std.time.nanoTimestamp();

        return types.TimedResult.create(result, start_time, end_time);
    }

    results: *const fn () [4]types.Result,
    solve_version01: *const fn (std.mem.Allocator, types.ProblemInput, bool) anyerror!types.Result,
    solve_version02: *const fn (std.mem.Allocator, types.ProblemInput, bool) anyerror!types.Result,
};

pub fn makeSolution(comptime day: anytype) Solution {
    return Solution{
        .results = day.results,
        .solve_version01 = day.solve_version01,
        .solve_version02 = day.solve_version02,
    };
}
