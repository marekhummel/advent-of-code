const std = @import("std");
const solution = @import("solution.zig");
const types = @import("types.zig");

pub const RunConfig = struct {
    all: bool,
    part: u8,
    use_sample: bool,
};

pub const AocRunner = struct {
    year: u16,
    solutions: [25]?solution.Solution,
    arena_enabled: bool,
    _allocator: std.mem.Allocator = undefined,

    pub fn init(year: u16, impl_solutions: []const ?solution.Solution, use_arena: bool) AocRunner {
        var solutions: [25]?solution.Solution = .{null} ** 25;
        std.mem.copyForwards(?solution.Solution, &solutions, impl_solutions);
        return .{
            .year = year,
            .solutions = solutions,
            .arena_enabled = use_arena,
        };
    }

    pub fn parseAndRun(self: *AocRunner) !void {
        var gpa = std.heap.GeneralPurposeAllocator(.{}){};
        self._allocator = gpa.allocator();
        defer {
            const deinit_status = gpa.deinit();
            if (deinit_status == .leak) std.debug.print("LEAK", .{});
        }

        const args = try std.process.argsAlloc(self._allocator);
        defer std.process.argsFree(self._allocator, args);

        if (args.len < 2) {
            std.debug.print("Usage: {s} [main | day N --part <1|2> [--sample] [--all]]\n", .{args[0]});
            return;
        }

        const cmd = args[1];

        if (std.mem.eql(u8, cmd, "main")) {
            try self.runFullYear();
            return;
        }

        if (std.mem.eql(u8, cmd, "day")) {
            if (args.len < 3) {
                std.debug.print("Error: Day number required after 'day' command\n", .{});
                return;
            }

            const day = std.fmt.parseInt(u8, args[2], 10) catch {
                std.debug.print("Error: Invalid day number: {s}\n", .{args[2]});
                return;
            };

            if (day < 1 or day > 25) {
                std.debug.print("Error: Day must be between 1 and 25, got: {d}\n", .{day});
                return;
            }

            const config = try self.parseConfig(args[3..]);

            if (config.all) {
                try self.runDay(day);
            } else {
                try self.runSingle(day, config.part, config.use_sample);
            }
            return;
        }

        std.debug.print("Error: Unknown command: {s}. Use 'main' or 'day N'\n", .{cmd});
    }

    fn parseConfig(_: *AocRunner, args: [][]const u8) !RunConfig {
        var config = RunConfig{
            .all = false,
            .part = 0,
            .use_sample = false,
        };

        var i: usize = 0;
        while (i < args.len) : (i += 1) {
            const arg = args[i];

            if (std.mem.eql(u8, arg, "--all") or std.mem.eql(u8, arg, "-a")) {
                config.all = true;
            } else if (std.mem.eql(u8, arg, "--sample") or std.mem.eql(u8, arg, "-s")) {
                config.use_sample = true;
            } else if (std.mem.eql(u8, arg, "--part") or std.mem.eql(u8, arg, "-p")) {
                if (i + 1 >= args.len) {
                    std.debug.print("Error: --part requires a value\n", .{});
                    return error.InvalidArgs;
                }
                i += 1;
                config.part = std.fmt.parseInt(u8, args[i], 10) catch {
                    std.debug.print("Error: Invalid part number: {s}\n", .{args[i]});
                    return error.InvalidArgs;
                };
                if (config.part != 1 and config.part != 2) {
                    std.debug.print("Error: --part must be 1 or 2, got: {d}\n", .{config.part});
                    return error.InvalidArgs;
                }
            }
        }

        if (!config.all and config.part == 0) {
            std.debug.print("Error: --part <1|2> is required unless --all is given\n", .{});
            return error.InvalidArgs;
        }

        return config;
    }

    pub fn run(self: *AocRunner, full_day: bool, part: u8, use_sample: bool) !void {
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
                try self.runSingle(day, part, use_sample);
            }
        } else |_| {
            std.debug.print("Argument should have format dayXX: {s}\n", .{cmd_arg});
        }
    }

    pub fn verifySolutions(self: *const AocRunner) bool {
        std.debug.print("\n----------\n", .{});
        var success = true;
        for (self.solutions, 1..) |sol, day| {
            inline for ([_]u8{ 1, 2 }) |part| {
                inline for ([_]bool{ true, false }) |use_sample| {
                    const result_union = self.getResult(@as(u8, @intCast(day)), part, use_sample);
                    if (result_union) |timed_result| {
                        defer timed_result.deinit(self._allocator);
                        std.debug.print("Testing D{0d:0>2} P{1d} '{2s}': ", .{ day, part, if (use_sample) "s" else "r" });
                        const index = comptime (part - 1) * 2 + (if (use_sample) 0 else 1);
                        const expected = sol.?.results()[index];
                        if (std.testing.expect(timed_result.result.eql(expected))) |_| {
                            std.debug.print("PASSED\n", .{});
                        } else |_| {
                            std.debug.print("FAILED: Got {0s}, expected {1s}\n", .{ timed_result.result, expected });
                            success = false;
                        }
                    } else |err| switch (err) {
                        error.MissingSolution => {},
                        else => {
                            std.debug.print("Testing D{0d:0>2} P{1d} '{2s}': ", .{ day, part, if (use_sample) "s" else "r" });
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
            for ([_]u8{ 1, 2 }) |part| {
                for ([_]bool{ true, false }) |use_sample| {
                    const timed_result = self.getResult(@as(u8, @intCast(day)), part, use_sample) catch |err| {
                        std.debug.print("  P{0d} {1s}:  {2s}\n", .{
                            part,
                            if (use_sample) "samp" else "real",
                            types.getErrorDesc(err),
                        });
                        continue;
                    };
                    defer timed_result.deinit(self._allocator);
                    day_elapsed += timed_result.duration;
                    std.debug.print("  P{0d} {1s}:  {2s}\n", .{
                        part,
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
        var expected_match = true;
        for ([_]u8{ 1, 2 }) |part| {
            for ([_]bool{ true, false }) |use_sample| {
                const timed_result = try self.getResult(day, part, use_sample);
                defer timed_result.deinit(self._allocator);
                day_elapsed += timed_result.duration;

                const index: usize = (part - 1) * 2 + (if (use_sample) @as(usize, 0) else @as(usize, 1));
                const expected = self.solutions[day - 1].?.results()[index];
                if (!timed_result.result.eql(expected)) {
                    expected_match = false;
                }

                std.debug.print("P{0d} {1s} in {2d:.4}s:    {3s}\n", .{
                    part,
                    if (use_sample) "samp" else "real",
                    timed_result.duration,
                    timed_result.result,
                });
            }
        }
        std.debug.print("\nTotal Runtime: {0d}s\n", .{day_elapsed});
        std.debug.print("Note: Results {s}match expected", .{if (expected_match) "" else "don't "});
    }

    fn runSingle(self: *const AocRunner, day: u8, part: u8, use_sample: bool) !void {
        const timed_result = try self.getResult(day, part, use_sample);
        defer timed_result.deinit(self._allocator);
        std.debug.print("Day {0d:0>2} / part {1d} / Data '{2s}' => {3d}s\n{4s}\n", .{
            day,
            part,
            if (use_sample) "samp" else "real",
            timed_result.duration,
            timed_result.result,
        });
    }

    fn getResult(self: *const AocRunner, day: u8, part: u8, use_sample: bool) types.SolvingError!types.TimedResult {
        if (day > 25 or day == 0) {
            std.debug.print("AoC only has 25 days: {d}\n", .{day});
            return types.SolvingError.InvalidDay;
        }
        const s = self.solutions[day - 1];
        if (s == null) {
            return types.SolvingError.MissingSolution;
        }

        var input = self.getInput(day, part, use_sample);
        if (input == null) {
            return .{ .result = types.Result.NoInput, .duration = 0 };
        }

        defer input.?.deinit();

        // Use arena allocator in solutions for easier coding
        if (self.arena_enabled) {
            var arena = std.heap.ArenaAllocator.init(self._allocator);
            const solution_allocator = arena.allocator();
            defer arena.deinit();

            // In case result contains heap allocated memory, clone with gpa before freeing
            var original_result = try s.?.solve(&input.?, part, use_sample, solution_allocator);
            return original_result.clone(self._allocator) catch types.SolvingError.SolvingFailed;
        } else {
            return s.?.solve(&input.?, part, use_sample, self._allocator);
        }
    }

    fn getInput(self: *const AocRunner, day: u8, part: u8, use_sample: bool) ?types.ProblemInput {
        const base_filename = if (use_sample) "sample" else "input";
        const filename = std.fmt.allocPrint(self._allocator, "{0d}\\inputs\\{1s}{2d:0>2}.txt", .{ self.year, base_filename, day }) catch return null;
        defer self._allocator.free(filename);

        const input = types.ProblemInput.read(self._allocator, filename) catch |err| switch (err) {
            std.fs.File.OpenError.FileNotFound => {
                const filename_vers = std.fmt.allocPrint(self._allocator, "{0d}\\inputs\\{1s}{2d:0>2}_{3d}.txt", .{ self.year, base_filename, day, part }) catch unreachable;
                defer self._allocator.free(filename_vers);
                return types.ProblemInput.read(self._allocator, filename_vers) catch return null;
            },
            else => return null,
        };

        return input;
    }
};
