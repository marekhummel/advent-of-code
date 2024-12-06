const std = @import("std");
const aoc_lib = @import("aoc_lib");
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const Index = aoc_lib.cartesian.Index;
const Direction = aoc_lib.cartesian.Direction;

pub fn results() [4]Result {
    return .{
        Result.Unsolved,
        Result.Unsolved,
        Result.Unsolved,
        Result.Unsolved,
    };
}

pub fn solve_version01(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    const map = try input.grid();
    var guard = map.find('^').?;
    var dir = Direction.North;

    var visited = std.AutoHashMap(Index, void).init(allocator);
    while (true) {
        try visited.put(guard, {});

        const next = guard.move(dir, map.width, map.height) orelse break;

        if (map.cells[next.r][next.c] == '#') {
            dir = dir.right();
            continue;
        }

        guard = next;
    }

    return Result{ .USize = visited.count() };
}

pub fn solve_version02(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    var map = try input.grid();
    const guard = map.find('^').?;

    const original_path = (try computePath(&map, guard, allocator)).path;
    defer allocator.free(original_path);

    var loops: usize = 0;
    for (original_path) |idx| {
        if (map.cells[idx.r][idx.c] != '.') continue;

        map.cells[idx.r][idx.c] = '#';
        const pathloop = try computePath(&map, guard, allocator);
        defer allocator.free(pathloop.path);
        if (pathloop.loop) {
            loops += 1;
            // std.debug.print("{any}\n", .{idx});
        }
        map.cells[idx.r][idx.c] = '.';
    }

    return Result{ .USize = loops };
}

fn computePath(
    map: *aoc_lib.types.Grid(u8),
    guard_start: Index,
    allocator: std.mem.Allocator,
) !struct { path: []Index, loop: bool } {
    var guard = guard_start;
    var dir = Direction.North;
    var moves = std.AutoHashMap(struct { Index, Direction }, void).init(allocator);
    defer moves.deinit();

    var positions = std.AutoHashMap(Index, void).init(allocator);
    defer positions.deinit();
    var loop = false;

    while (true) {
        try positions.put(guard, {});
        if (try moves.fetchPut(.{ guard, dir }, {}) != null) {
            loop = true;
            break;
        }

        const next = guard.move(dir, map.width, map.height) orelse break;

        if (map.cells[next.r][next.c] == '#') {
            dir = dir.right();
            continue;
        }

        guard = next;
    }

    var path = std.ArrayList(Index).init(allocator);
    var keyIterator = positions.keyIterator();
    while (keyIterator.next()) |pos| try path.append(pos.*);
    return .{ .path = try path.toOwnedSlice(), .loop = loop };
}
