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

pub fn solvePart01(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = allocator;
    _ = input;
    _ = is_sample;

    return Result.Unsolved;
}

pub fn solvePart02(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = allocator;
    _ = input;
    _ = is_sample;

    // Binary adder (z = x + y), bitwise from LSB to MSB with carry c
    // t_n = x_n XOR y_n
    // xy_n = x_n AND y_n
    // tc_n = t_n AND c_n
    // c_n = xy_(n-1) OR tc_(n-1)
    // z_n = t_n XOR c_n
    // c_0 = 0

    return Result.Unsolved;
}
