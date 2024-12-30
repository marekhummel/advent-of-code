const std = @import("std");
const aoc_lib = @import("aoc_lib");
const regex = @import("zigRegex");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;

pub fn results() [4]Result {
    return .{
        Result{ .Int64 = 480 },
        Result{ .Int64 = 32026 },
        Result.NoSample,
        Result{ .Int64 = 89013607072065 },
    };
}

pub fn solvePart01(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;
    return Result{ .Int64 = try computeTokens(input, 0, allocator) };
}

pub fn solvePart02(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    if (is_sample) return Result.NoSample;
    return Result{ .Int64 = try computeTokens(input, 10_000_000_000_000, allocator) };
}

fn computeTokens(input: *ProblemInput, error_correction: i64, allocator: Allocator) !i64 {
    var delta_rgx = try regex.Regex.compile(allocator, "Button [AB]: X\\+(\\d+), Y\\+(\\d+)");
    var prize_rgx = try regex.Regex.compile(allocator, "Prize: X=(\\d+), Y=(\\d+)");

    var tokens: i64 = 0;
    var machine_it = std.mem.window([]u8, input.lines, 3, 4);
    while (machine_it.next()) |wndw| {
        // Parse input
        var captures_a = (try delta_rgx.captures(wndw[0])).?;
        const dxa: i64 = try std.fmt.parseInt(i64, captures_a.sliceAt(1).?, 10);
        const dya: i64 = try std.fmt.parseInt(i64, captures_a.sliceAt(2).?, 10);

        var captures_b = (try delta_rgx.captures(wndw[1])).?;
        const dxb: i64 = try std.fmt.parseInt(i64, captures_b.sliceAt(1).?, 10);
        const dyb: i64 = try std.fmt.parseInt(i64, captures_b.sliceAt(2).?, 10);

        var captures_prize = (try prize_rgx.captures(wndw[2])).?;
        const px: i64 = try std.fmt.parseInt(i64, captures_prize.sliceAt(1).?, 10) + error_correction;
        const py: i64 = try std.fmt.parseInt(i64, captures_prize.sliceAt(2).?, 10) + error_correction;

        // Compute moves (solve 2x2 lin. eq.)
        // [px] = [a] * [dxa  dxb]
        // [py]   [b]   [dya  dyb]
        const det_x = dxa * dyb - dya * dxb;
        const a = std.math.divExact(i64, px * dyb - py * dxb, det_x) catch continue;
        const b = std.math.divExact(i64, py * dxa - px * dya, det_x) catch continue;

        // Only count if results are integers and positive
        if (a >= 0 and b >= 0) {
            tokens += a * 3 + b;
        }
    }

    return tokens;
}
