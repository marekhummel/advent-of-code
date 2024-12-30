const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const startsWith = aoc_lib.util.startsWith;

pub fn results() [4]Result {
    return .{
        Result{ .USize = 6 },
        Result{ .USize = 206 },
        Result{ .USize = 16 },
        Result{ .USize = 622121814629343 },
    };
}

pub fn solvePart01(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;

    const design_combs = try findAllDesignCombs(input, allocator);
    var possible: usize = 0;
    for (design_combs) |combs| possible += if (combs > 0) 1 else 0;
    return Result{ .USize = possible };
}

pub fn solvePart02(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;

    const design_combs = try findAllDesignCombs(input, allocator);
    var total: usize = 0;
    for (design_combs) |combs| total += combs;
    return Result{ .USize = total };
}

fn findAllDesignCombs(input: *ProblemInput, allocator: Allocator) ![]usize {
    var towels = std.ArrayList([]const u8).init(allocator);
    var towel_it = std.mem.tokenizeSequence(u8, input.lines[0], ", ");
    while (towel_it.next()) |towel| try towels.append(towel);

    var design_combinations = std.ArrayList(usize).init(allocator);
    var cache = std.StringHashMap(usize).init(allocator);
    defer cache.deinit();
    for (input.lines[2..]) |design| {
        try design_combinations.append(try createDesign(design, towels.items, &cache));
    }

    return try design_combinations.toOwnedSlice();
}

fn createDesign(design: []u8, towels: [][]const u8, cache: *std.StringHashMap(usize)) !usize {
    if (design.len == 0) return 1;
    if (cache.get(design)) |possible| return possible;

    var total: usize = 0;
    for (towels) |towel| {
        if (towel.len <= design.len and startsWith(u8, design, towel)) {
            total += try createDesign(design[towel.len..], towels, cache);
        }
    }

    try cache.put(design, total);
    return total;
}
