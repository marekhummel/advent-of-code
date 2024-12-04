const std = @import("std");

pub const Result = union(enum) {
    NoInput,
    NoSample,
    Unsolved,
    NoPartTwo,
    USize: usize,
    Int8: i8,
    Int16: i16,
    Int32: i32,
    Int64: i64,
    Int128: i128,
    UInt8: u8,
    UInt16: u16,
    UInt32: u32,
    UInt64: u64,
    UInt128: u128,
    String: []u8,

    pub fn format(self: Result, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        return try switch (self) {
            .NoInput => std.fmt.format(writer, "No Input", .{}),
            .NoSample => std.fmt.format(writer, "No Sample", .{}),
            .Unsolved => std.fmt.format(writer, "Unsolved", .{}),
            .NoPartTwo => std.fmt.format(writer, "No Part Two", .{}),
            .USize => std.fmt.format(writer, "{}", .{self.USize}),
            .Int8 => std.fmt.format(writer, "{}", .{self.Int8}),
            .Int16 => std.fmt.format(writer, "{}", .{self.Int16}),
            .Int32 => std.fmt.format(writer, "{}", .{self.Int32}),
            .Int64 => std.fmt.format(writer, "{}", .{self.Int64}),
            .Int128 => std.fmt.format(writer, "{}", .{self.Int128}),
            .UInt8 => std.fmt.format(writer, "{}", .{self.UInt8}),
            .UInt16 => std.fmt.format(writer, "{}", .{self.UInt16}),
            .UInt32 => std.fmt.format(writer, "{}", .{self.UInt32}),
            .UInt64 => std.fmt.format(writer, "{}", .{self.UInt64}),
            .UInt128 => std.fmt.format(writer, "{}", .{self.UInt128}),
            .String => std.fmt.format(writer, "{s}", .{self.String}),
        };
    }
};

pub const TimedResult = struct {
    result: Result,
    duration: f64,

    pub fn create(result: Result, start_time_ns: i128, end_time_ns: i128) TimedResult {
        return .{ .result = result, .duration = @as(f64, @floatFromInt(end_time_ns - start_time_ns)) / 1e9 };
    }
};

pub const ProblemInput = struct {
    lines: [][]u8,
    _string: ?[]u8 = null,
    _grid: ?Grid(u8) = null,
    _allocator: std.mem.Allocator = undefined,

    pub fn read(allocator: std.mem.Allocator, filename: []const u8) !ProblemInput {
        var lines = std.ArrayList([]u8).init(allocator);
        errdefer lines.deinit();

        const file = try std.fs.cwd().openFile(filename, .{});
        defer file.close();

        var buf_reader = std.io.bufferedReader(file.reader());
        const reader = buf_reader.reader();

        var line = std.ArrayList(u8).init(allocator);
        defer line.deinit();

        while (reader.streamUntilDelimiter(line.writer(), '\n', null)) {
            defer line.clearRetainingCapacity();
            if (line.getLast() == '\r') _ = line.popOrNull();
            try lines.append(try line.toOwnedSlice());
        } else |err| switch (err) {
            error.EndOfStream => {
                if (line.items.len > 0) {
                    try lines.append(try line.toOwnedSlice());
                }
            },
            else => return err,
        }

        return ProblemInput{ .lines = try lines.toOwnedSlice(), ._allocator = allocator };
    }

    pub fn string(self: *ProblemInput) ![]u8 {
        if (self._string == null) {
            var total_length: usize = 0;
            for (self.lines) |line| {
                total_length += line.len;
            }

            self._string = try self._allocator.alloc(u8, total_length);

            var offset: usize = 0;
            for (self.lines) |line| {
                std.mem.copyForwards(u8, self._string.?[offset..], line);
                offset += line.len;
            }
        }

        return self._string.?;
    }

    pub fn grid(self: *ProblemInput) !Grid(u8) {
        if (self._grid == null) {
            var grid_cells = try self._allocator.alloc([]u8, self.lines.len);
            for (self.lines, 0..) |line, i| {
                grid_cells[i] = try self._allocator.alloc(u8, line.len);
                std.mem.copyForwards(u8, grid_cells[i][0..line.len], line);
            }

            self._grid = Grid(u8).init(grid_cells);
        }

        return self._grid.?;
    }

    pub fn deinit(self: *const ProblemInput) void {
        if (self._string != null)
            self._allocator.free(self._string.?);

        if (self._grid != null)
            self._grid.?.deinit(self._allocator);

        for (self.lines) |line| {
            self._allocator.free(line);
        }
        self._allocator.free(self.lines);
    }
};

pub fn Grid(comptime CT: type) type {
    return struct {
        const Self = @This();

        cells: [][]CT,
        width: usize,
        height: usize,
        diags: usize,

        pub fn init(cells: [][]CT) Grid(CT) {
            const w = cells[0].len;
            const h = cells.len;
            return Grid(CT){ .cells = cells, .width = w, .height = h, .diags = w + h - 1 };
        }

        pub fn deinit(self: @This(), allocator: std.mem.Allocator) void {
            for (self.cells) |line| {
                allocator.free(line);
            }
            allocator.free(self.cells);
        }

        pub fn row(self: @This(), index: usize, allocator: std.mem.Allocator) ![]CT {
            if (index >= self.height) return error.IndexOutOfBounds;

            const slice = try allocator.alloc(CT, self.width);
            std.mem.copyForwards(CT, slice, self.cells[index]);
            return slice;
        }

        pub fn col(self: @This(), index: usize, allocator: std.mem.Allocator) ![]CT {
            if (index >= self.width) return error.IndexOutOfBounds;

            const slice = try allocator.alloc(CT, self.height);
            for (self.cells, 0..) |line, r| {
                slice[r] = line[index];
            }
            return slice;
        }

        pub fn diag_major(self: @This(), index: usize, allocator: std.mem.Allocator) ![]CT {
            if (index >= self.diags) return error.IndexOutOfBounds;

            var list = std.ArrayList(CT).init(allocator);
            var r = self.height -| 1 -| index; // saturating sub -| (0 -| 1 = 0)
            var c = index + 1 -| self.height; // saturating sub -|

            while (r < self.height and c < self.width) {
                try list.append(self.cells[r][c]);
                r += 1;
                c += 1;
            }

            return list.toOwnedSlice();
        }

        pub fn diag_minor(self: @This(), index: usize, allocator: std.mem.Allocator) ![]CT {
            if (index >= self.diags) return error.IndexOutOfBounds;

            var list = std.ArrayList(CT).init(allocator);
            var r = @min(index, self.height - 1);
            var c = index + 1 -| self.height; // saturating sub -|

            while (r < self.height and c < self.width) {
                try list.append(self.cells[r][c]);
                r -%= 1;
                c += 1;
            }

            return list.toOwnedSlice();
        }
    };
}

pub const SolvingError = error{
    InvalidDay,
    InvalidVersion,
    MissingSolution,
    SolvingFailed,
};

pub fn getErrorDesc(err: SolvingError) []const u8 {
    switch (err) {
        SolvingError.InvalidDay => return "Day is invalid",
        SolvingError.InvalidVersion => return "Version is invalid",
        SolvingError.MissingSolution => return "Solution is not registered",
        SolvingError.SolvingFailed => return "Some error occured in solving method",
    }
}
