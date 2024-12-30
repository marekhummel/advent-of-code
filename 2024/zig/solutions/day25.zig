const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const Grid = aoc_lib.types.Grid;

pub fn results() [4]Result {
    return .{
        Result{ .USize = 3 },
        Result{ .USize = 3360 },
        Result.NoPartTwo,
        Result.NoPartTwo,
    };
}

pub fn solvePart01(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    var locks = std.ArrayList([]u8).init(allocator);
    var keys = std.ArrayList([]u8).init(allocator);
    var i: usize = 0;
    while (true) {
        // Collect full grid
        var current = std.ArrayList([]u8).init(allocator);
        while (true) {
            if (i >= input.lines.len or input.lines[i].len == 0) break;
            try current.append(input.lines[i]);
            i += 1;
        }

        // Parse
        var grid = Grid(u8).init(try current.toOwnedSlice());
        defer grid.deinit(allocator);

        var pins = try allocator.alloc(u8, grid.size.width);
        if (grid.cells[0][0] == '#') {
            // Lock
            for (0..grid.size.width) |c| {
                const col = try grid.col(c, allocator);
                const height = std.mem.lastIndexOf(u8, col, "#").?;
                pins[c] = @intCast(height);
            }
            try locks.append(pins);
        } else {
            // Key
            for (0..grid.size.width) |c| {
                const col = try grid.col(c, allocator);
                const height = std.mem.indexOf(u8, col, "#").?;
                pins[c] = @intCast(grid.size.height - height - 1);
            }
            try keys.append(pins);
        }

        // Next iteration
        i += 1;
        if (i >= input.lines.len) break;
    }

    // Check all combinations
    var total_fit: usize = 0;
    for (locks.items) |lock| {
        for (keys.items) |key| {
            var fits = true;
            for (0..lock.len) |p| fits = fits and (lock[p] + key[p] < 6);
            if (fits) total_fit += 1;
            // std.debug.print("Lock {any} and key {any}: {}\n", .{ lock, key, fits });
        }
    }

    return Result{ .USize = total_fit };
}

pub fn solvePart02(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = allocator;
    _ = input;
    _ = is_sample;

    return Result.NoPartTwo;
}
