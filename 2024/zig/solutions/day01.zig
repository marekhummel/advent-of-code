const std = @import("std");
const aoc_lib = @import("aoc_lib");
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;

pub fn results() [4]Result {
    return .{
        Result.Unsolved,
        Result.Unsolved,
        Result.Unsolved,
        Result.Unsolved,
    };
}

pub fn solve_version01(allocator: std.mem.Allocator, input: ProblemInput, is_sample: bool) Result {
    _ = allocator;
    // _ = input;
    _ = is_sample;
    // std.debug.print("D1 V1 {s}\n", .{if (is_sample) "samp" else "real"});
    // std.debug.print("{s}\n", .{input.lines[0]});

    return Result{ .USize = input.lines.len };
}

pub fn solve_version02(allocator: std.mem.Allocator, input: ProblemInput, is_sample: bool) Result {
    _ = allocator;
    _ = input;
    _ = is_sample;
    // std.debug.print("D1 V2 {s}\n", .{if (is_sample) "samp" else "real"});
    return Result.Unsolved;
}
