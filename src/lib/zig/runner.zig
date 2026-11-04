const std = @import("std");
const solution = @import("solution.zig");
const types = @import("types.zig");

pub const AocRunner = struct {
    year: u16,
    solutions: std.AutoHashMap(u8, solution.Solution),

    /// Build the registry from day/solution pairs; registration order does not matter.
    pub fn init(allocator: std.mem.Allocator, year: u16, implementations: []const struct { u8, solution.Solution }) !AocRunner {
        var solutions = std.AutoHashMap(u8, solution.Solution).init(allocator);
        errdefer solutions.deinit();
        for (implementations) |entry| {
            try solutions.put(entry[0], entry[1]);
        }
        return .{ .year = year, .solutions = solutions };
    }

    pub fn parseAndRun(self: *const AocRunner, process: std.process.Init) !void {
        const allocator = process.arena.allocator();
        const args = try process.minimal.args.toSlice(allocator);
        if (args.len != 3 and args.len != 4) usage();
        const day = std.fmt.parseInt(u8, args[1], 10) catch usage();
        const part = std.fmt.parseInt(u8, args[2], 10) catch usage();
        if (day < 1 or day > 25 or part < 1 or part > 2) usage();
        const sample = args.len == 4;
        if (sample and !std.mem.eql(u8, args[3], "--sample")) usage();
        const implementation = self.solutions.get(day) orelse return error.MissingSolution;
        const prefix = if (sample) "sample" else "input";
        const filename = try std.fmt.allocPrint(allocator, "inputs/{d}/{s}{d:0>2}.txt", .{ self.year, prefix, day });
        var input = types.ProblemInput.read(process.io, allocator, filename) catch |err| switch (err) {
            error.FileNotFound => blk: {
                const fallback = try std.fmt.allocPrint(allocator, "inputs/{d}/{s}{d:0>2}_{d}.txt", .{ self.year, prefix, day, part });
                break :blk try types.ProblemInput.read(process.io, allocator, fallback);
            },
            else => return err,
        };
        const solve = if (part == 1) implementation.solvePart01 else implementation.solvePart02;
        const result = try solve(&input, sample, allocator);
        switch (result) {
            .NoInput => return error.NoInput,
            .NoSample => return error.NoSample,
            .Unsolved => return error.Unsolved,
            .NoPartTwo => return error.NoPartTwo,
            else => {},
        }
        var buffer: [4096]u8 = undefined;
        var stdout = std.Io.File.stdout().writer(process.io, &buffer);
        try stdout.interface.print("{f}\n", .{result});
        try stdout.interface.flush();
    }

    fn usage() noreturn {
        std.debug.print("Usage: mainYEAR DAY PART [--sample]\n", .{});
        std.process.exit(2);
    }
};
