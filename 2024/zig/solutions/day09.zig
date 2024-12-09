const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;

pub fn results() [4]Result {
    return .{
        Result.Unsolved,
        Result.Unsolved,
        Result.Unsolved,
        Result.Unsolved,
    };
}

pub fn solvePart01(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    const disk_map = try input.string();
    for (disk_map) |*c| c.* = c.* - '0';

    var total_disk_size: usize = 0;
    for (disk_map) |d| total_disk_size += d;

    var disk = try allocator.alloc(?usize, total_disk_size);
    defer allocator.free(disk);

    var index: usize = 0;
    for (disk_map, 0..) |d, i| {
        const is_file = (i & 1 == 0);
        const value = if (is_file) i / 2 else null;
        for (0..d) |j| disk[index + j] = value;
        index += d;
    }

    // std.debug.print("{any}\n", .{disk});

    var i: usize = 0;
    var j: usize = disk.len - 1;
    while (true) {
        while (disk[j] == null and i < j) j -= 1;
        while (disk[i] != null and i < j) i += 1;
        if (i >= j) break;

        disk[i] = disk[j];
        disk[j] = null;
    }

    var checksum: usize = 0;
    for (disk, 0..) |file, pos| {
        if (file == null) break;
        checksum += file.? * pos;
    }

    // std.debug.print("{any}\n", .{disk});

    return Result{ .USize = checksum };
}

pub fn solvePart02(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    // _ = is_sample;

    const disk_map = try input.string();
    for (disk_map) |*c| c.* = c.* - '0';

    var files = std.ArrayList(Block).init(allocator);
    var free_spans = std.ArrayList(Block).init(allocator);

    var index: usize = 0;
    for (disk_map, 0..) |d, i| {
        const is_file = (i & 1 == 0);
        if (is_file) {
            try files.append(Block{ .id = i / 2, .position = index, .length = d });
        } else {
            try free_spans.append(Block{ .id = null, .position = index, .length = d });
        }
        index += d;
    }

    var i: usize = files.items.len;
    while (i > 0) {
        i -= 1;
        const file = &files.items[i];

        for (free_spans.items) |*free| {
            if (free.*.length >= file.*.length) {
                try free_spans.append(Block{ .id = null, .position = file.*.position, .length = file.*.length });
                file.*.position = free.*.position;
                free.*.position += file.*.length;
                free.*.length -= file.*.length;
                break;
            }
        }
    }

    var checksum: usize = 0;
    for (files.items) |file| {
        for (0..file.length) |offset| checksum += (file.position + offset) * file.id.?;
    }

    if (is_sample) {
        var total_disk_size: usize = 0;
        for (disk_map) |d| total_disk_size += d;

        outer: for (0..total_disk_size) |di| {
            for (files.items) |f| {
                if (f.position <= di and di < f.position + f.length) {
                    std.debug.print("{d}", .{f.id.?});
                    continue :outer;
                }
            }
            std.debug.print(".", .{});
        }
        std.debug.print("\n", .{});

        outer: for (0..total_disk_size) |di| {
            for (free_spans.items) |f| {
                if (f.position <= di and di < f.position + f.length) {
                    std.debug.print(".", .{});
                    continue :outer;
                }
            }
            std.debug.print("X", .{});
        }
        std.debug.print("\n", .{});

        std.debug.print("{any}\n", .{free_spans.items});
    }

    return Result{ .USize = checksum };
}

const Block = struct {
    id: ?usize,
    position: usize,
    length: usize,

    pub fn format(self: @This(), comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        try std.fmt.format(writer, "[{any}: {d} {d}]", .{ self.id, self.position, self.length });
    }
};
