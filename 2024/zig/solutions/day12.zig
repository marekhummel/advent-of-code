const std = @import("std");
const aoc_lib = @import("aoc_lib");
const set = @import("ziglangSet");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const Grid = aoc_lib.types.Grid;
const Index = aoc_lib.cartesian.Index;
const Direction = aoc_lib.cartesian.Direction;

pub fn results() [4]Result {
    return .{
        Result{ .UInt64 = 1930 },
        Result{ .UInt64 = 1456082 },
        Result{ .UInt64 = 1206 },
        Result{ .UInt64 = 872382 },
    };
}

pub fn solvePart01(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;
    var garden = try input.grid();
    return Result{ .UInt64 = try computePrice(&garden, Region.price, allocator) };
}

pub fn solvePart02(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;
    var garden = try input.grid();
    return Result{ .UInt64 = try computePrice(&garden, Region.bulk_price, allocator) };
}

const Fence = struct {
    idx: Index,
    dir: Direction,

    fn lessThan(context: void, lhs: Fence, rhs: Fence) bool {
        _ = context;
        if (lhs.dir != rhs.dir) return @intFromEnum(lhs.dir) < @intFromEnum(rhs.dir);
        return switch (lhs.dir) {
            Direction.North, Direction.South => if (lhs.idx.r != rhs.idx.r) lhs.idx.r < rhs.idx.r else lhs.idx.c < rhs.idx.c,
            Direction.East, Direction.West => if (lhs.idx.c != rhs.idx.c) lhs.idx.c < rhs.idx.c else lhs.idx.r < rhs.idx.r,
        };
    }
};

const Region = struct {
    area: u64,
    fences: std.ArrayList(Fence),

    fn perimeter(self: Region) u64 {
        return self.fences.items.len;
    }

    fn countSides(self: Region) u64 {
        // Sort fences, so that fences of same side are grouped
        std.sort.block(Fence, self.fences.items, {}, Fence.lessThan);

        // Find groups (sides) by checking if indices are only off set by one
        var num_sides: u64 = 1;
        var current: Fence = self.fences.items[0];
        for (self.fences.items[1..]) |fence| {
            defer current = fence;

            if (current.dir == fence.dir) {
                switch (current.dir) {
                    Direction.North, Direction.South => if (current.idx.r == fence.idx.r and current.idx.c + 1 == fence.idx.c) continue,
                    Direction.East, Direction.West => if (current.idx.c == fence.idx.c and current.idx.r + 1 == fence.idx.r) continue,
                }
            }

            num_sides += 1;
        }

        return num_sides;
    }

    fn price(self: Region) u64 {
        return self.area * self.perimeter();
    }

    fn bulk_price(self: Region) u64 {
        return self.area * self.countSides();
    }

    fn extend(self: *Region, other: Region) !void {
        self.area += other.area;
        try self.fences.appendSlice(other.fences.items);
    }
};

const COUNTED = '.';

/// Compute price for entire garden
fn computePrice(garden: *Grid(u8), priceFn: fn (Region) u64, allocator: Allocator) !u64 {
    var total_price: u64 = 0;
    var plot_it = garden.iterator(false);
    while (plot_it.next()) |elem| {
        // Plots are marked once they've been accounted for
        if (garden.get(elem.idx) == COUNTED) continue;

        var visited = set.Set(Index).init(allocator);
        const region = try findRegion(garden, elem.idx, &visited, allocator);
        total_price += priceFn(region);
    }

    return total_price;
}

/// Recursively fill out region
fn findRegion(garden: *Grid(u8), index: Index, visited: *set.Set(Index), allocator: Allocator) !Region {
    const plant = garden.get(index);
    garden.set(index, COUNTED);
    _ = try visited.add(index);

    var region = Region{ .area = 1, .fences = std.ArrayList(Fence).init(allocator) };
    for (index.vonNeumann(garden.size, true)) |nb| {
        // Don't revisit same plots of region
        if (nb.idx != null and visited.contains(nb.idx.?)) continue;

        // If neighbor is same plant, continue, otherwise place fence
        if (nb.idx != null and garden.get(nb.idx.?) == plant) {
            try region.extend(try findRegion(garden, nb.idx.?, visited, allocator));
        } else {
            try region.fences.append(.{ .idx = index, .dir = nb.dir });
        }
    }

    return region;
}
