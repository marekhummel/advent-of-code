const std = @import("std");
const aoc_lib = @import("aoc_lib");
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;

pub fn results() [4]Result {
    return .{
        Result{ .UInt64 = 3749 },
        Result{ .UInt64 = 1430271835320 },
        Result{ .UInt64 = 11387 },
        Result{ .UInt64 = 456565678667482 },
    };
}

pub fn solvePart01(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;
    return Result{ .UInt64 = try calibrate(input, false, allocator) };
}

pub fn solvePart02(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;
    return Result{ .UInt64 = try calibrate(input, true, allocator) };
}

const Equation = struct { rhs: u64, values: []u64 };

/// Parse every calibration equation and sum valid ones
fn calibrate(input: *ProblemInput, allow_concat: bool, allocator: std.mem.Allocator) !u64 {
    var calibration_result: u64 = 0;
    for (input.lines) |line| {
        const equation = try parseEquationStr(line, allocator);
        defer allocator.free(equation.values);

        if (evalEquation(equation, allow_concat, 0)) {
            calibration_result += equation.rhs;
        }
    }

    return calibration_result;
}

/// Parses line to equation struct
fn parseEquationStr(line: []u8, allocator: std.mem.Allocator) !Equation {
    var num_strs = std.mem.tokenizeAny(u8, line, ": ");

    var values = std.ArrayList(u64).init(allocator);

    while (num_strs.next()) |num_str| {
        const num = try std.fmt.parseInt(u64, num_str, 10);
        try values.append(num);
    }
    const rhs = values.orderedRemove(0);

    return .{ .rhs = rhs, .values = try values.toOwnedSlice() };
}

/// Evaluate an equation, true if combination of operators found
/// Use recursion instead of looping over all operator combinations, because this avoids
/// recomputing partial results (v[0] + v[1] is computed once here, but in every third computation otherwise)
fn evalEquation(equation: Equation, allow_concat: bool, acc: u64) bool {
    if (equation.values.len == 0) {
        return equation.rhs == acc;
    }

    const next_equation = .{ .rhs = equation.rhs, .values = equation.values[1..] };
    return (evalEquation(next_equation, allow_concat, acc + equation.values[0]) or
        evalEquation(next_equation, allow_concat, acc * equation.values[0]) or
        (allow_concat and evalEquation(next_equation, allow_concat, acc * std.math.pow(u64, 10, std.math.log10(equation.values[0]) + 1) + equation.values[0])));
}
