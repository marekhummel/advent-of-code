const std = @import("std");
const solution = @import("solution.zig");
const types = @import("types.zig");

pub const AocRunner = struct {
    year: u16,
    solutions: [25]?solution.Solution,
    _allocator: std.mem.Allocator = undefined,

    pub fn init(year: u16, impl_solutions: []const ?solution.Solution) AocRunner {
        var solutions: [25]?solution.Solution = .{null} ** 25;
        std.mem.copyForwards(?solution.Solution, &solutions, impl_solutions);
        return .{
            .year = year,
            .solutions = solutions,
        };
    }

    pub fn run(self: *AocRunner, full_day: bool, version: u8, use_sample: bool) !void {
        var gpa = std.heap.GeneralPurposeAllocator(.{}){};
        self._allocator = gpa.allocator();
        defer {
            const deinit_status = gpa.deinit();
            if (deinit_status == .leak) std.debug.print("LEAK", .{});
        }

        const args = try std.process.argsAlloc(self._allocator);
        defer std.process.argsFree(self._allocator, args);
        if (args.len != 2) {
            std.debug.print("Usage: {s} <day>\n", .{args[0]});
            return;
        }
        const cmd_arg = args[1];

        if (std.mem.eql(u8, cmd_arg, "main")) {
            try self.runFullYear();
            return;
        }

        if (std.fmt.parseInt(u8, cmd_arg[3..], 10)) |day| {
            if (full_day) {
                try self.runDay(day);
            } else {
                try self.runSingle(day, version, use_sample);
            }
        } else |_| {
            std.debug.print("Argument should have format dayXX: {s}\n", .{cmd_arg});
        }
    }

    pub fn verifySolutions(self: *const AocRunner) bool {
        std.debug.print("\n----------\n", .{});
        var success = true;
        for (self.solutions, 1..) |sol, day| {
            inline for ([_]u8{ 1, 2 }) |version| {
                inline for ([_]bool{ true, false }) |use_sample| {
                    const result_union = self.getResult(@as(u8, @intCast(day)), version, use_sample);
                    if (result_union) |timed_result| {
                        std.debug.print("Testing D{0d:0>2} V{1d} '{2s}': ", .{ day, version, if (use_sample) "s" else "r" });
                        const index = comptime (version - 1) * 2 + (if (use_sample) 0 else 1);
                        const expected = sol.?.results()[index];
                        if (std.testing.expect(std.meta.eql(timed_result.result, expected))) |_| {
                            std.debug.print("PASSED\n", .{});
                        } else |_| {
                            std.debug.print("FAILED: Got {0s}, expected {1s}\n", .{ timed_result.result, expected });
                            success = false;
                        }
                    } else |err| switch (err) {
                        error.MissingSolution => {},
                        else => {
                            std.debug.print("Testing D{0d:0>2} V{1d} '{2s}': ", .{ day, version, if (use_sample) "s" else "r" });
                            std.debug.print("ERROR RESULT ({!})\n", .{result_union});
                            success = false;
                        },
                    }
                }
            }
        }
        std.debug.print("----------\n", .{});
        return success;
    }

    fn runFullYear(self: *const AocRunner) !void {
        var year_elapsed: f64 = 0.0;
        for (self.solutions, 1..) |_, day| {
            std.debug.print("Day {0d:0>2}\n", .{day});
            var day_elapsed: f64 = 0.0;
            for ([_]u8{ 1, 2 }) |version| {
                for ([_]bool{ true, false }) |use_sample| {
                    const timed_result = self.getResult(@as(u8, @intCast(day)), version, use_sample) catch |err| {
                        std.debug.print("  V{0d} {1s}:  {2s}\n", .{
                            version,
                            if (use_sample) "samp" else "real",
                            types.getErrorDesc(err),
                        });
                        continue;
                    };
                    day_elapsed += timed_result.duration;
                    std.debug.print("  V{0d} {1s}:  {2s}\n", .{
                        version,
                        if (use_sample) "samp" else "real",
                        timed_result.result,
                    });
                }
            }
            year_elapsed += day_elapsed;
            std.debug.print("  > Runtime: {0d}\n\n", .{day_elapsed});
        }
        std.debug.print("\n\nTotal Runtime: {0d}\n", .{year_elapsed});
    }

    fn runDay(self: *const AocRunner, day: u8) !void {
        var day_elapsed: f64 = 0.0;
        for ([_]u8{ 1, 2 }) |version| {
            for ([_]bool{ true, false }) |use_sample| {
                const timed_result = try self.getResult(day, version, use_sample);
                day_elapsed += timed_result.duration;
                std.debug.print("V{0d} {1s} in {2d:.4}s:    {3s}\n", .{
                    version,
                    if (use_sample) "samp" else "real",
                    timed_result.duration,
                    timed_result.result,
                });
            }
        }
        std.debug.print("\nTotal Runtime: {0d}s\n", .{day_elapsed});
    }

    fn runSingle(self: *const AocRunner, day: u8, version: u8, use_sample: bool) !void {
        const timed_result = try self.getResult(day, version, use_sample);
        std.debug.print("Day {0d:0>2} / Version {1d} / Data '{2s}' => {3d}s\n{4s}\n", .{
            day,
            version,
            if (use_sample) "samp" else "real",
            timed_result.duration,
            timed_result.result,
        });
    }

    fn getResult(self: *const AocRunner, day: u8, version: u8, use_sample: bool) types.SolvingError!types.TimedResult {
        if (day > 25 or day == 0) {
            std.debug.print("AoC only has 25 days: {d}\n", .{day});
            return types.SolvingError.InvalidDay;
        }
        const s = self.solutions[day - 1];
        if (s == null) {
            return types.SolvingError.MissingSolution;
        }

        const input = self.getInput(day, version, use_sample);
        if (input == null) {
            return .{ .result = types.Result.NoInput, .duration = 0 };
        }

        defer input.?.deinit();
        return s.?.solve(self._allocator, input.?, version, use_sample);
    }

    fn getInput(self: *const AocRunner, day: u8, version: u8, use_sample: bool) ?types.ProblemInput {
        const base_filename = if (use_sample) "sample" else "input";
        const filename = std.fmt.allocPrint(self._allocator, "{0d}\\inputs\\{1s}{2d:0>2}.txt", .{ self.year, base_filename, day }) catch return null;
        defer self._allocator.free(filename);

        const input = types.ProblemInput.read(self._allocator, filename) catch |err| switch (err) {
            std.fs.File.OpenError.FileNotFound => {
                const filename_vers = std.fmt.allocPrint(self._allocator, "{0d}\\inputs\\{1s}{2d:0>2}_{3d}.txt", .{ self.year, base_filename, day, version }) catch unreachable;
                defer self._allocator.free(filename_vers);
                return types.ProblemInput.read(self._allocator, filename_vers) catch return null;
            },
            else => return null,
        };

        return input;
    }
};
