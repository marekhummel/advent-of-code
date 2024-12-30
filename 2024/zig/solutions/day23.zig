const std = @import("std");
const aoc_lib = @import("aoc_lib");
const set = @import("ziglangSet");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const util = aoc_lib.util;

pub fn results() [4]Result {
    return .{
        Result{ .USize = 7 },
        Result{ .USize = 1163 },
        Result{ .String = "co,de,ka,ta" },
        Result{ .String = "bm,bo,ee,fo,gt,hv,jv,kd,md,mu,nm,wx,xh" },
    };
}

const Pc = [2]u8;
const Conn = [4]u8;

pub fn solvePart01(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;

    // Create network
    var network = try createNetwork(input, allocator);
    const pcs = try util.setToSlice(Pc, &network.pcs, allocator);

    // Try all triples and check if 3-clique with "t"
    var count: usize = 0;
    for (0..pcs.len) |i| {
        for (i + 1..pcs.len) |j| {
            if (!network.conns.get(pcs[i]).?.contains(pcs[j])) continue;

            for (j + 1..pcs.len) |k| {
                if (pcs[i][0] == 't' or pcs[j][0] == 't' or pcs[k][0] == 't') {
                    const nbs = network.conns.get(pcs[k]).?;
                    if (nbs.contains(pcs[i]) and nbs.contains(pcs[j])) {
                        count += 1;
                    }
                }
            }
        }
    }

    return Result{ .USize = count };
}

pub fn solvePart02(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;

    var network = try createNetwork(input, allocator);

    // Find biggest clique
    var r = set.Set(Pc).init(allocator);
    var p = try network.pcs.clone();
    var x = set.Set(Pc).init(allocator);
    const biggest = (try findBiggestClique(&r, &p, &x, &network.conns, allocator)).?;

    // Sort elements by name
    const lessThan = struct {
        fn stringLessThan(_: void, lhs: [2]u8, rhs: [2]u8) bool {
            return std.mem.order(u8, &lhs, &rhs) == .lt;
        }
    }.stringLessThan;
    std.mem.sort(Pc, biggest, {}, lessThan);

    // Create password
    var password = try allocator.alloc(u8, biggest.len * 3);
    for (biggest, 0..) |pc, i| {
        _ = try std.fmt.bufPrint(password[i * 3 ..], "{c}{c},", .{ pc[0], pc[1] });
    }

    return Result{ .String = password[0 .. password.len - 1] };
}

fn createNetwork(input: *ProblemInput, allocator: Allocator) !struct { pcs: set.Set(Pc), conns: std.AutoHashMap(Pc, set.Set(Pc)) } {
    var pc_set = set.Set(Pc).init(allocator);
    var conns = std.AutoHashMap(Pc, set.Set(Pc)).init(allocator);
    for (input.lines) |line| {
        const pc1 = Pc{ line[0], line[1] };
        const pc2 = Pc{ line[3], line[4] };
        _ = try pc_set.add(pc1);
        _ = try pc_set.add(pc2);

        var entry1 = try conns.getOrPut(pc1);
        if (!entry1.found_existing) entry1.value_ptr.* = set.Set(Pc).init(allocator);
        _ = try entry1.value_ptr.add(pc2);

        var entry2 = try conns.getOrPut(pc2);
        if (!entry2.found_existing) entry2.value_ptr.* = set.Set(Pc).init(allocator);
        _ = try entry2.value_ptr.add(pc1);
    }

    return .{ .pcs = pc_set, .conns = conns };
}

/// Bron kerbosch algorithm with pivoting
fn findBiggestClique(r: *set.Set(Pc), p: *set.Set(Pc), x: *set.Set(Pc), conns: *std.AutoHashMap(Pc, set.Set(Pc)), allocator: Allocator) !?[]Pc {
    if (p.cardinality() == 0 and x.cardinality() == 0) {
        defer r.deinit();
        defer p.deinit();
        defer x.deinit();
        return try util.setToSlice(Pc, r, allocator);
    }

    var biggest_clique: ?[]Pc = null;
    var pivots = try p.unionOf(x.*);
    const u = pivots.pop().?;

    const neighbours_u = conns.get(u) orelse set.Set(Pc).init(allocator);
    var vs = try p.differenceOf(neighbours_u);

    while (vs.pop()) |v| {
        var new_r = try r.clone();
        _ = try new_r.add(v);

        const neighbours_v = conns.get(v) orelse set.Set(Pc).init(allocator);
        var new_p = try p.clone();
        try new_p.intersectionUpdate(neighbours_v);
        var new_x = try x.clone();
        try new_x.intersectionUpdate(neighbours_v);

        if (try findBiggestClique(&new_r, &new_p, &new_x, conns, allocator)) |clique| {
            if (biggest_clique == null or clique.len > biggest_clique.?.len) {
                biggest_clique = clique;
            }
        }

        _ = p.remove(v);
        _ = try x.add(v);
    }

    return biggest_clique;
}
