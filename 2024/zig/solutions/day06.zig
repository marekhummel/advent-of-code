const std = @import("std");
const aoc_lib = @import("aoc_lib");
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const Index = aoc_lib.cartesian.Index;
const Direction = aoc_lib.cartesian.Direction;

pub fn results() [4]Result {
    return .{
        Result{ .USize = 41 },
        Result{ .USize = 4752 },
        Result{ .USize = 6 },
        Result{ .USize = 1719 },
    };
}

pub fn solve_version01(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    const map = try input.grid();
    const guard = map.find('^').?;

    const pathloop = try computePath(&map, guard, allocator);
    return Result{ .USize = pathloop.path.len };
}

pub fn solve_version02(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    var map = try input.grid();
    const guard = map.find('^').?;

    const original_path = (try computePath(&map, guard, allocator)).path;

    // Try to put obstacles at every point the guard would've visited
    var loops: usize = 0;
    for (original_path) |idx| {
        if (map.get(idx) != '.') continue;

        map.set(idx, '#');
        if ((try computePath(&map, guard, allocator)).loop) {
            loops += 1;
        }
        map.set(idx, '.');
    }

    return Result{ .USize = loops };
}

fn computePath(
    map: *const aoc_lib.types.Grid(u8),
    guard_start: Index,
    allocator: std.mem.Allocator,
) !struct { path: []Index, loop: bool } {
    var guard = guard_start;
    var dir = Direction.North;

    // We use array for all positions and mark the direction at each position,
    // because this is much (x2-x4) faster than zigs sets (AutoArrayHashMap / ziglangSet)
    var pathLookup = try allocator.alloc(u4, map.height * map.width);
    for (0..pathLookup.len) |i| pathLookup[i] = 0;
    defer allocator.free(pathLookup);
    var positions = std.ArrayList(Index).init(allocator);

    // Walk while in bounds and no loop
    while (true) {
        const index = guard.r * map.width + guard.c;
        const samePosition = (pathLookup[index] != 0);
        const samePath = (pathLookup[index] & @intFromEnum(dir) != 0);

        // Check if guard was on same path before (same pos and dir)
        if (samePath) {
            return .{ .path = try positions.toOwnedSlice(), .loop = true };
        }
        pathLookup[index] |= @intFromEnum(dir);

        // Add position to list of visited indices if new
        if (!samePosition) {
            try positions.append(guard);
        }

        // Move guard (if next is null, we are out of bounds)
        const next = guard.move(dir, map.width, map.height) orelse break;
        if (map.cells[next.r][next.c] == '#') {
            dir = dir.right();
        } else {
            guard = next;
        }
    }

    return .{ .path = try positions.toOwnedSlice(), .loop = false };
}
