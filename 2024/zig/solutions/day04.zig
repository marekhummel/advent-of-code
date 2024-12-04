const std = @import("std");
const aoc_lib = @import("aoc_lib");
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;

pub fn results() [4]Result {
    return .{
        Result{ .USize = 18 },
        Result{ .USize = 2545 },
        Result{ .USize = 9 },
        Result{ .USize = 1886 },
    };
}

pub fn solve_version01(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    const grid = try input.grid();
    var counter: usize = 0;

    // Loop over all rows, cols, major and minor diagonals.
    for (0..grid.height) |r| {
        const row = try grid.row(r, allocator);
        defer allocator.free(row);
        search(row, &counter);
    }

    for (0..grid.width) |c| {
        const col = try grid.col(c, allocator);
        defer allocator.free(col);
        search(col, &counter);
    }

    for (0..grid.diags) |d| {
        const diag_maj = try grid.diag_major(d, allocator);
        const diag_min = try grid.diag_minor(d, allocator);
        defer allocator.free(diag_maj);
        defer allocator.free(diag_min);

        search(diag_maj, &counter);
        search(diag_min, &counter);
    }

    return Result{ .USize = counter };
}

pub fn solve_version02(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = allocator;
    _ = is_sample;

    const grid = try input.grid();
    var counter: usize = 0;

    for (1..grid.height - 1) |r| {
        for (1..grid.width - 1) |c| {
            if (grid.cells[r][c] != 'A')
                continue;

            const diag_maj = (grid.cells[r - 1][c - 1] == 'M' and grid.cells[r + 1][c + 1] == 'S') or (grid.cells[r - 1][c - 1] == 'S' and grid.cells[r + 1][c + 1] == 'M');
            const diag_min = (grid.cells[r - 1][c + 1] == 'M' and grid.cells[r + 1][c - 1] == 'S') or (grid.cells[r - 1][c + 1] == 'S' and grid.cells[r + 1][c - 1] == 'M');
            if (diag_maj and diag_min) counter += 1;
        }
    }

    return Result{ .USize = counter };
}

/// Searches for all the occurences of XMAS or SAMX in a line
fn search(line: []u8, counter: *usize) void {
    var start: usize = 0;
    while (true) {
        start = std.mem.indexOfPos(u8, line, start, "XMAS") orelse break;
        start += 4;
        counter.* += 1;
    }

    start = 0;
    while (true) {
        start = std.mem.indexOfPos(u8, line, start, "SAMX") orelse break;
        start += 4;
        counter.* += 1;
    }
}
