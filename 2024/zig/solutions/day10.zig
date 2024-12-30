const std = @import("std");
const aoc_lib = @import("aoc_lib");
const set = @import("ziglangSet");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const Grid = aoc_lib.types.Grid;
const Index = aoc_lib.cartesian.Index;

pub fn results() [4]Result {
    return .{
        Result{ .USize = 36 },
        Result{ .USize = 512 },
        Result{ .USize = 81 },
        Result{ .USize = 1045 },
    };
}

pub fn solvePart01(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;
    return Result{ .USize = try countTrails(try input.grid(), true, allocator) };
}

pub fn solvePart02(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;
    return Result{ .USize = try countTrails(try input.grid(), false, allocator) };
}

const TrailCache = std.AutoHashMap(Index, std.ArrayList(Index));

fn countTrails(topo_map: Grid(u8), only_count_ends: bool, allocator: Allocator) !usize {
    // Set up cache with capacity to prevent ptr invalidations
    var trail_cache = TrailCache.init(allocator);
    try trail_cache.ensureTotalCapacity(@intCast(topo_map.size.total()));

    // Find all trail heads
    var num_trails: usize = 0;
    var it = topo_map.iterator();
    while (it.next()) |elem| {
        if (elem.value != '0') continue;

        // Update cache
        try findTrailEndsFromHere(elem.idx, &topo_map, &trail_cache, allocator);

        // Reduce to unique ends if part 1
        const trail_ends = trail_cache.get(elem.idx).?;
        if (only_count_ends) {
            var uniqueTrailEnds = set.Set(Index).init(allocator);
            num_trails += try uniqueTrailEnds.appendSlice(trail_ends.items);
        } else {
            num_trails += trail_ends.items.len;
        }
    }

    return num_trails;
}

fn findTrailEndsFromHere(idx: Index, topo_map: *const Grid(u8), trail_cache: *TrailCache, allocator: Allocator) !void {
    var trail_ends = std.ArrayList(Index).init(allocator);
    defer trail_cache.putAssumeCapacity(idx, trail_ends);

    const curr_height = topo_map.get(idx);

    // At height 9 this is the end.
    if (curr_height == '9') {
        _ = try trail_ends.append(idx);
        return;
    }

    // Check valid neighbors with right slope and union all ends.
    for (idx.vonNeumann(topo_map.size, false)) |nb| {
        if (nb != null and topo_map.get(nb.?) -| curr_height == 1) {
            const entry = trail_cache.getOrPutAssumeCapacity(nb.?);
            if (!entry.found_existing) try findTrailEndsFromHere(nb.?, topo_map, trail_cache, allocator);
            try trail_ends.appendSlice(entry.value_ptr.items); // Ptr valid since no reallocations
        }
    }
}
