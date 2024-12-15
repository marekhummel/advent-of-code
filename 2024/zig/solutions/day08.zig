const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const Grid = aoc_lib.types.Grid;
const Index = aoc_lib.cartesian.Index;
const Position = aoc_lib.cartesian.Position;
const PosDelta = aoc_lib.cartesian.PosDelta;
const set = @import("ziglangSet");

pub fn results() [4]Result {
    return .{
        Result{ .UInt32 = 14 },
        Result{ .UInt32 = 396 },
        Result{ .UInt32 = 34 },
        Result{ .UInt32 = 1200 },
    };
}

pub fn solvePart01(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    const antenna_map = try input.grid();
    const antenna_lookup = try createAntennaLookup(&antenna_map, allocator);

    var antinodes = set.Set(Index).init(allocator);
    var antenna_it = AntennaIterator.init(antenna_lookup);
    while (antenna_it.next()) |elem| {
        _ = try addAntinode(elem.antenna, elem.delta, 2, &antinodes, &antenna_map);
    }

    return Result{ .UInt32 = antinodes.cardinality() };
}

pub fn solvePart02(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    const antenna_map = try input.grid();
    const antenna_lookup = try createAntennaLookup(&antenna_map, allocator);

    var antinodes = set.Set(Index).init(allocator);
    var antenna_it = AntennaIterator.init(antenna_lookup);
    while (antenna_it.next()) |elem| {
        var n: i64 = 0;
        while (try addAntinode(elem.antenna, elem.delta, n, &antinodes, &antenna_map)) : (n += 1) {}
    }

    return Result{ .UInt32 = antinodes.cardinality() };
}

const AntennaLookup = std.AutoHashMap(u8, std.ArrayList(Index));

/// Each element of this iterator is a line through two antennas of same frequency.
/// Element is tuple of antenna and delta, where antenna + 1*delta = other_antenna
/// Note that each line is visited twice, once for each antenna.
const AntennaIterator = struct {
    antenna_lookup: AntennaLookup,
    _freqs_it: AntennaLookup.KeyIterator,
    _freq: u8 = undefined,
    _i: usize = 0,
    _j: usize = 0, // technically with this impl (0, 0) is skipped, but we dont want that anyways

    const Self = @This();

    fn init(antenna_lookup: AntennaLookup) Self {
        var freqs_it = antenna_lookup.keyIterator();
        const first_freq = freqs_it.next().?.*;
        return .{ .antenna_lookup = antenna_lookup, ._freqs_it = freqs_it, ._freq = first_freq };
    }

    fn next(self: *Self) ?struct { antenna: Position, delta: PosDelta } {
        var antennas = self.antenna_lookup.get(self._freq).?;
        if (self._j < antennas.items.len - 1) {
            self._j += 1;
            if (self._i == self._j) return self.next();
        } else if (self._i < antennas.items.len - 1) {
            self._i += 1;
            self._j = 0;
        } else {
            if (self._freqs_it.next()) |freq| self._freq = freq.* else return null;
            self._i = 0;
            self._j = 1;
            antennas = self.antenna_lookup.get(self._freq).?;
        }

        const antenna1 = antennas.items[self._i].asPosition();
        const antenna2 = antennas.items[self._j].asPosition();
        const delta = antenna1.diff(antenna2);
        return .{ .antenna = antenna1, .delta = delta };
    }
};

fn createAntennaLookup(antenna_map: *const Grid(u8), allocator: Allocator) !AntennaLookup {
    var antenna_lookup = AntennaLookup.init(allocator);

    var antenna_map_it = antenna_map.iterator();
    while (antenna_map_it.next()) |elem| {
        if (elem.value == '.') continue;

        const entry = try antenna_lookup.getOrPut(elem.value);
        if (!entry.found_existing) entry.value_ptr.* = std.ArrayList(Index).init(allocator);
        try entry.value_ptr.*.append(elem.idx);
    }

    return antenna_lookup;
}

/// Given base = antenna1: 0 -> antenna1, 1 -> antenna2,
fn addAntinode(base: Position, d: PosDelta, n: i64, antinodes: *set.Set(Index), antenna_map: *const Grid(u8)) !bool {
    const antinode = base.offset(n * d.dx, n * d.dy);
    if (antinode.asIndex(antenna_map.size)) |idx| {
        _ = try antinodes.add(idx);
        return true;
    }
    return false;
}
