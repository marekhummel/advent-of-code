const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;

pub fn results() [4]Result {
    return .{
        Result{ .USize = 2 },
        Result{ .USize = 242 },
        Result{ .USize = 4 },
        Result{ .USize = 311 },
    };
}

pub fn solvePart01(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;

    const data = try getReports(allocator, input);
    var safe_reports: usize = 0;
    for (data) |report| {
        if (checkSafety(report, null))
            safe_reports += 1;
    }

    return Result{ .USize = safe_reports };
}

pub fn solvePart02(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;

    const data = try getReports(allocator, input);
    var safe_reports: usize = 0;
    for (data) |report| {
        // Check without skips
        if (checkSafety(report, null)) {
            safe_reports += 1;
            continue;
        }
        // Skip each level and check
        for (0..report.len) |skip| {
            if (checkSafety(report, skip)) {
                safe_reports += 1;
                break;
            }
        }
    }

    return Result{ .USize = safe_reports };
}

fn getReports(allocator: std.mem.Allocator, input: *ProblemInput) ![][]i16 {
    var matrix = std.ArrayList([]i16).init(allocator);

    for (input.lines) |line| {
        var row = std.ArrayList(i16).init(allocator);

        var tokenizer = std.mem.tokenize(u8, line, " ");
        while (tokenizer.next()) |tok| {
            const num = try std.fmt.parseInt(i16, tok, 10);
            try row.append(num);
        }

        try matrix.append(try row.toOwnedSlice());
    }

    return matrix.toOwnedSlice();
}

fn checkSafety(report: []i16, skip: ?usize) bool {
    var increasing: ?bool = null;

    for (0..report.len - 1) |i| {
        // Skip if current index is marked
        const a = i;
        if (skip != null and a == skip)
            continue;

        // Compute next index (skip one if needed)
        var b = i + 1;
        if (skip != null and b == skip) {
            b += 1;
            if (b >= report.len)
                continue;
        }

        const delta = report[b] - report[a];

        if (increasing != null and (increasing != (delta > 0)))
            return false;
        increasing = (delta > 0);

        if (delta == 0 or @abs(delta) > 3)
            return false;
    }

    return true;
}
