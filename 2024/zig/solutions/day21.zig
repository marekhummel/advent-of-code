const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
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

pub fn solvePart01(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = input;
    _ = is_sample;
    _ = allocator;

    return Result.Unsolved;
}

pub fn solvePart02(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = input;
    _ = is_sample;
    _ = allocator;

    return Result.Unsolved;
}
