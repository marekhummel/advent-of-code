const std = @import("std");
const aoc_lib = @import("aoc_lib");
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const Grid = aoc_lib.types.Grid;
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

pub fn solvePart01(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    const map = try input.grid();
    const guard = map.find('^').?;

    const pathloop = try computePath(&map, guard, null, allocator);
    return Result{ .USize = pathloop.path.len };
}

pub fn solvePart02(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    var map = try input.grid();
    const guard = map.find('^').?;

    const original_path = (try computePath(&map, guard, null, allocator)).path;

    // -- Try to put obstacles at every point the guard would've visited --

    // ** Threading solution, little overkill but good practice (runtime 0.55s -> 0.12s)
    var thread_safe_arena: std.heap.ThreadSafeAllocator = .{ .child_allocator = allocator };
    const thread_allocator = thread_safe_arena.allocator();

    const chunk_size: usize = 100;
    const thread_count = original_path.len / chunk_size + 1;

    var threads = try allocator.alloc(std.Thread, thread_count);
    var loop_counters = try allocator.alloc(usize, thread_count);

    // Create threads and run
    const specs = .{ .map = &map, .guard = guard, .indices = original_path };
    for (0..thread_count) |tid| {
        const thread_info = .{ .tid = tid, .chunk_sz = chunk_size, .allocator = thread_allocator };
        threads[tid] = try std.Thread.spawn(.{}, checkLoopThread, .{ specs, thread_info, &loop_counters[tid] });
    }

    // Join all and accumulate result pointers
    var loops: usize = 0;
    for (0..thread_count) |tid| {
        threads[tid].join();
        loops += loop_counters[tid];
    }

    // // ** Non-threading solution
    // var loops: usize = 0;
    // for (original_path) |idx| {
    //     if (map.get(idx) != '.') continue;

    //     if ((try computePath(&map, guard, idx, allocator)).loop) {
    //         loops += 1;
    //     }
    // }

    return Result{ .USize = loops };
}

fn checkLoopThread(
    specs: struct { map: *const Grid(u8), guard: Index, indices: []Index },
    thread_info: struct { tid: usize, chunk_sz: usize, allocator: std.mem.Allocator },
    loop_ctr: *usize,
) void {
    const start = thread_info.tid * thread_info.chunk_sz;
    const end = @min(start + thread_info.chunk_sz, specs.indices.len);
    var loops: usize = 0;
    for (specs.indices[start..end]) |idx| {
        if (specs.map.get(idx) != '.') continue;

        if ((computePath(specs.map, specs.guard, idx, thread_info.allocator) catch unreachable).loop) {
            loops += 1;
        }
    }
    loop_ctr.* = loops;
}

fn computePath(
    map: *const Grid(u8),
    guard_start: Index,
    obstruction: ?Index,
    allocator: std.mem.Allocator,
) !struct { path: []Index, loop: bool } {
    var guard = guard_start;
    var dir = Direction.North;

    // We use array for all positions and mark the direction at each position,
    // because this is much (x2-x4) faster than zigs sets (AutoArrayHashMap / ziglangSet)
    var pathLookup = try allocator.alloc(u4, map.size.total());
    for (0..pathLookup.len) |i| pathLookup[i] = 0;
    defer allocator.free(pathLookup);
    var positions = std.ArrayList(Index).init(allocator);

    // Walk while in bounds and no loop
    while (true) {
        const index = guard.r * map.size.width + guard.c;
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
        const next = guard.scout(dir, map.size) orelse break;
        if (map.get(next) == '#' or (obstruction != null and std.meta.eql(next, obstruction.?))) {
            dir = dir.right();
        } else {
            guard = next;
        }
    }

    return .{ .path = try positions.toOwnedSlice(), .loop = false };
}
