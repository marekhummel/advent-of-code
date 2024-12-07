const std = @import("std");
const aoc_lib = @import("aoc_lib");
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;

pub fn results() [4]Result {
    return .{
        Result{ .UInt32 = 11 },
        Result{ .UInt32 = 3714264 },
        Result{ .UInt32 = 31 },
        Result{ .UInt32 = 18805872 },
    };
}

pub fn solvePart01(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    var left_list = std.ArrayList(i32).init(allocator);
    var right_list = std.ArrayList(i32).init(allocator);
    defer left_list.deinit();
    defer right_list.deinit();
    try getLists(i32, input, &left_list, &right_list);

    var total: u32 = 0;
    for (left_list.items, right_list.items) |left, right| {
        total += @abs(left - right);
    }

    return Result{ .UInt32 = total };
}

pub fn solvePart02(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    var left_list = std.ArrayList(u32).init(allocator);
    var right_list = std.ArrayList(u32).init(allocator);
    try getLists(u32, input, &left_list, &right_list);

    var counter = std.AutoHashMap(u32, u32).init(allocator);

    for (right_list.items) |right| {
        const entry = try counter.getOrPut(right);
        if (!entry.found_existing) entry.value_ptr.* = 0;
        entry.value_ptr.* += 1;
    }

    var sim_score: u32 = 0;
    for (left_list.items) |left| {
        const occurances = counter.get(left) orelse 0;
        sim_score += left * occurances;
    }

    return Result{ .UInt32 = sim_score };
}

fn getLists(comptime T: type, input: *ProblemInput, left_list: *std.ArrayList(T), right_list: *std.ArrayList(T)) !void {
    for (input.lines) |line| {
        var left = true;
        var it = std.mem.split(u8, line, " ");
        while (it.next()) |x| {
            if (x.len == 0) {
                continue;
            }

            const num = try std.fmt.parseInt(T, x, 10);
            if (left) {
                try left_list.append(num);
                left = false;
            } else {
                try right_list.append(num);
            }
        }
    }
    std.debug.assert(left_list.items.len == right_list.items.len);
    std.mem.sort(T, left_list.items[0..], {}, comptime std.sort.asc(T));
    std.mem.sort(T, right_list.items[0..], {}, comptime std.sort.asc(T));
}
