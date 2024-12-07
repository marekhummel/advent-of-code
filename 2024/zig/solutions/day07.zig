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

pub fn solve_version01(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;
    return Result{ .UInt64 = try calibrate(input, 2, allocator) };
}

pub fn solve_version02(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;
    return Result{ .UInt64 = try calibrate(input, 3, allocator) };
}

const Equation = struct { rhs: u64, values: []u64 };

/// Parse every calibration equation and sum valid ones
fn calibrate(input: *ProblemInput, num_operations: comptime_int, allocator: std.mem.Allocator) !u64 {
    var calibration_result: u64 = 0;
    for (input.lines) |line| {
        const equation = try parseEquationStr(line, allocator);
        defer allocator.free(equation.values);

        if (evalEquation(&equation, num_operations)) {
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
fn evalEquation(equation: *const Equation, num_operators: comptime_int) bool {
    const num_configs = std.math.pow(u64, num_operators, equation.values.len - 1);
    // All possible combinations of operators are given in the binary / ternary represations of integers.
    for (0..num_configs) |config| {
        var operator_lookup = config;
        var total: u64 = equation.values[0];

        for (equation.values[1..]) |val| {
            switch (operator_lookup % num_operators) {
                0 => total += val,
                1 => total *= val,
                2 => total = total * std.math.pow(u64, 10, std.math.log10(val) + 1) + val, // Unreachable in part 1
                else => unreachable,
            }

            operator_lookup /= num_operators;
        }

        if (total == equation.rhs) {
            return true;
        }
    }

    return false;
}
