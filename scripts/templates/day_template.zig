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
    _ = input;
    _ = is_sample;

    return Result.Unsolved;
}

pub fn solve_version02(allocator: std.mem.Allocator, input: ProblemInput, is_sample: bool) Result {
    _ = allocator;
    _ = input;
    _ = is_sample;

    return Result.Unsolved;
}
