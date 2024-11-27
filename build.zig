const std = @import("std");

pub fn build(b: *std.Build) void {
    add_binary(b, "2024");
    // add_binary(b, "2023");
}

fn add_binary(b: *std.Build, comptime year: []const u8) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const name = "main" ++ year;

    // Add aoc lib as module
    const aoc_lib_mod = b.addModule("aoc_lib", .{ .root_source_file = b.path("lib/zig/aoc_lib.zig") });

    // Build exe
    const exe = b.addExecutable(.{
        .name = name,
        .root_source_file = b.path(year ++ "/zig/main.zig"),
        .target = target,
        .optimize = optimize,
    });
    exe.root_module.addImport("aoc_lib", aoc_lib_mod);

    // Compile step
    const install_artifact = b.addInstallArtifact(exe, .{});
    const compile_step = b.step(year ++ "_main", "Build " ++ year);
    compile_step.dependOn(&install_artifact.step);

    // Testing step
    const unit_tests = b.addTest(.{
        .root_source_file = b.path(year ++ "/zig/main.zig"),
        .target = b.resolveTargetQuery(.{}),
    });
    unit_tests.root_module.addImport("aoc_lib", aoc_lib_mod);
    const unittest_artifact = b.addRunArtifact(unit_tests);
    const test_step = b.step(year ++ "_test", "Run unit tests for " ++ year);
    test_step.dependOn(&unittest_artifact.step);

    // Run step ?
    // const run_exe = b.addRunArtifact(exe);
    // const run_step = b.step("run", "Run " ++ year);
    // run_step.dependOn(&run_exe.step);
}
