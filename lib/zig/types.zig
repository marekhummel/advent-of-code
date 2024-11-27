const std = @import("std");

pub const Result = union(enum) {
    NoInput,
    NoSample,
    Unsolved,
    NoPartTwo,
    USize: usize,
    Int64: i64,
    UInt64: u64,
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
            .Int64 => std.fmt.format(writer, "{}", .{self.Int64}),
            .UInt64 => std.fmt.format(writer, "{}", .{self.UInt64}),
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

    pub fn deinit(self: *const ProblemInput) void {
        for (self.lines) |line| {
            self._allocator.free(line);
        }
        self._allocator.free(self.lines);
    }

    // impl ProblemInput {
    // pub fn read(filename: &str) -> Option<Self> {
    //     let file = File::open(filename).ok()?;
    //     let buf = BufReader::new(file);

    //     let lines = buf.lines().map(|l| l.expect("Could not parse line")).collect();
    //     Some(ProblemInput { lines })
    // }

    // pub fn lines(&self) -> Vec<String> {
    //     self.lines.iter().cloned().collect_vec()
    // }

    // pub fn string(&self) -> String {
    //     self.lines.join("")
    // }

    // pub fn grid(&self) -> Grid<char> {
    //     Grid::new(self.lines.iter().map(|row| row.chars().collect()).collect())
    // }
    // }
};

pub const SolvingError = error{
    InvalidDay,
    InvalidVersion,
    MissingSolution,
};

pub fn getErrorDesc(err: SolvingError) []const u8 {
    switch (err) {
        SolvingError.InvalidDay => return "Day is invalid",
        SolvingError.InvalidVersion => return "Version is invalid",
        SolvingError.MissingSolution => return "Solution is not registered",
    }
}

// #[derive(Debug, PartialEq, Eq)]
// pub enum ProblemResult {
//     NoInput,
//     NoSample,
//     Unsolved,
//     NoPartTwo,
//     I128(i128),
//     I64(i64),
//     I32(i32),
//     I16(i16),
//     I8(i8),
//     ISize(isize),
//     U128(u128),
//     U64(u64),
//     U32(u32),
//     U16(u16),
//     U8(u8),
//     USize(usize),
//     BigInt(BigInt),
//     String(String),
// }
