const std = @import("std");
const cartesian = @import("cartesian.zig");
const Index = cartesian.Index;
const Size = cartesian.Size;

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
    String: []const u8,

    pub fn format(self: Result, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        return try switch (self) {
            .NoInput => std.fmt.format(writer, "<No Input Available>", .{}),
            .NoSample => std.fmt.format(writer, "<No Sample Defined>", .{}),
            .Unsolved => std.fmt.format(writer, "<No Solution Implemented>", .{}),
            .NoPartTwo => std.fmt.format(writer, "<No Part Two>", .{}),
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

    pub fn eql(self: Result, other: Result) bool {
        if (std.meta.activeTag(self) != std.meta.activeTag(other))
            return false;

        return switch (self) {
            .String => |str_val| std.mem.eql(u8, str_val, other.String),
            else => std.meta.eql(self, other),
        };
    }
};

pub const TimedResult = struct {
    result: Result,
    duration: f64,

    pub fn create(result: Result, start_time_ns: i128, end_time_ns: i128) TimedResult {
        return .{ .result = result, .duration = @as(f64, @floatFromInt(end_time_ns - start_time_ns)) / 1e9 };
    }

    /// In case the result is of type string, we need to duplicate the heap memory before its freed
    pub fn clone(self: *const @This(), allocator: std.mem.Allocator) !TimedResult {
        const result_copy = switch (self.result) {
            .String => |str_val| Result{ .String = try allocator.dupe(u8, str_val) },
            else => self.result,
        };
        return .{ .result = result_copy, .duration = self.duration };
    }

    pub fn deinit(self: *const @This(), allocator: std.mem.Allocator) void {
        switch (self.result) {
            .String => |str_val| allocator.free(str_val),
            else => {},
        }
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
            if (line.items.len > 0 and line.getLast() == '\r') _ = line.popOrNull();
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
        size: Size,

        pub fn init(cells: [][]CT) Grid(CT) {
            const w = cells[0].len;
            const h = cells.len;
            return Grid(CT){ .cells = cells, .size = Size{ .width = w, .height = h } };
        }

        pub fn empty(size: Size, default: CT, allocator: std.mem.Allocator) !Grid(CT) {
            var grid_cells = try allocator.alloc([]CT, size.height);
            for (0..size.height) |r| {
                grid_cells[r] = try allocator.alloc(CT, size.width);
                @memset(grid_cells[r], default);
            }

            return Grid(CT){ .cells = grid_cells, .size = size };
        }

        pub fn deinit(self: @This(), allocator: std.mem.Allocator) void {
            for (self.cells) |line| {
                allocator.free(line);
            }
            allocator.free(self.cells);
        }

        pub fn get(self: Self, index: Index) CT {
            return self.cells[index.r][index.c];
        }

        pub fn get_ref(self: Self, index: Index) *CT {
            return &self.cells[index.r][index.c];
        }

        pub fn set(self: Self, index: Index, value: CT) void {
            self.cells[index.r][index.c] = value;
        }

        pub fn row(self: Self, index: usize, allocator: std.mem.Allocator) ![]CT {
            if (index >= self.size.height) return error.IndexOutOfBounds;

            const slice = try allocator.alloc(CT, self.size.width);
            std.mem.copyForwards(CT, slice, self.cells[index]);
            return slice;
        }

        pub fn col(self: Self, index: usize, allocator: std.mem.Allocator) ![]CT {
            if (index >= self.size.width) return error.IndexOutOfBounds;

            const slice = try allocator.alloc(CT, self.size.height);
            for (self.cells, 0..) |line, r| {
                slice[r] = line[index];
            }
            return slice;
        }

        pub fn diagMajor(self: Self, index: usize, allocator: std.mem.Allocator) ![]CT {
            if (index >= self.size.diags()) return error.IndexOutOfBound;

            var list = std.ArrayList(CT).init(allocator);
            var r = self.size.height -| 1 -| index; // saturating sub -| (0 -| 1 = 0)
            var c = index + 1 -| self.size.height; // saturating sub -|

            while (r < self.size.height and c < self.size.width) {
                try list.append(self.cells[r][c]);
                r += 1;
                c += 1;
            }

            return list.toOwnedSlice();
        }

        pub fn diagMinor(self: Self, index: usize, allocator: std.mem.Allocator) ![]CT {
            if (index >= self.size.diags()) return error.IndexOutOfBound;

            var list = std.ArrayList(CT).init(allocator);
            var r = @min(index, self.size.height - 1);
            var c = index + 1 -| self.size.height; // saturating sub -|

            while (r < self.size.height and c < self.size.width) {
                try list.append(self.cells[r][c]);
                r -%= 1;
                c += 1;
            }

            return list.toOwnedSlice();
        }

        pub fn find(self: *const Self, needle: CT) ?Index {
            for (0..self.size.height) |r| {
                for (0..self.size.width) |c| {
                    if (self.cells[r][c] == needle) {
                        return Index{ .r = r, .c = c };
                    }
                }
            }

            return null;
        }

        pub fn convert(
            self: *const Self,
            allocator: std.mem.Allocator,
            comptime T: type,
            comptime mapFunc: fn (char: u8, r: usize, c: usize) T,
        ) Grid(T) {
            var new_cells = try allocator.alloc([]T, self.size.height);
            for (self.cells, 0..) |line, r| {
                new_cells[r] = try self._allocator.alloc(T, self.size.width);
                for (line, 0..) |cell, c| {
                    new_cells[r][c] = mapFunc(cell, r, c);
                }
            }

            return Grid(T).init(new_cells);
        }

        pub fn print(self: *const Self, comptime fmt: []const u8) void {
            for (0..self.size.height) |r| {
                for (0..self.size.width) |c| {
                    std.debug.print(fmt, .{self.cells[r][c]});
                }
                std.debug.print("\n", .{});
            }
        }

        pub const GridIterator = struct {
            size: Size,
            ref: [][]CT,
            _init: bool = true,
            _r: usize = 0,
            _c: usize = 0,

            pub const Item = struct { idx: Index, value: CT };

            const ItSelf = @This();

            pub fn next(self: *ItSelf) ?Item {
                if (self._init) {
                    self._init = false;
                } else if (self._c < self.size.width - 1) {
                    self._c += 1;
                } else if (self._r < self.size.height - 1) {
                    self._r += 1;
                    self._c = 0;
                } else return null;

                const value = self.ref[self._r][self._c];
                return .{ .idx = Index{ .r = self._r, .c = self._c }, .value = value };
            }
        };

        pub fn iterator(self: *const Self) GridIterator {
            return GridIterator{ .size = self.size, .ref = self.cells };
        }
    };
}

pub const SolvingError = error{
    InvalidDay,
    InvalidPart,
    MissingSolution,
    SolvingFailed,
};

pub fn getErrorDesc(err: SolvingError) []const u8 {
    switch (err) {
        SolvingError.InvalidDay => return "<Day Invalid>",
        SolvingError.InvalidPart => return "<Part Invalid>",
        SolvingError.MissingSolution => return "<No Solution Implemented>",
        SolvingError.SolvingFailed => return "<Solution raised error>",
    }
}
