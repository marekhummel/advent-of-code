const std = @import("std");
const aoc_lib = @import("aoc_lib");
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const contains = aoc_lib.util.contains;

pub fn results() [4]Result {
    return .{
        Result{ .UInt32 = 143 },
        Result{ .UInt32 = 6034 },
        Result{ .UInt32 = 123 },
        Result{ .UInt32 = 6305 },
    };
}

pub fn solve_version01(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    var rules, const updates = try parse(input, allocator);
    var result: u32 = 0;
    for (updates) |upd| {
        if (isOrdered(upd, &rules)) {
            const middle = upd[(upd.len - 1) / 2];
            result += middle;
        }
    }

    return Result{ .UInt32 = result };
}

pub fn solve_version02(allocator: std.mem.Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    var rules, const updates = try parse(input, allocator);
    var result: u32 = 0;
    for (updates) |upd| {
        if (isOrdered(upd, &rules)) continue;

        // For whatever reason the overall list of rules is not sortable,
        // only if we reduce it to the pages occuring in each update.

        // Reduce rule graph to occuring pages in update
        var graph = aoc_lib.graph.Graph(u16).init(allocator);

        for (try rules.edges(allocator)) |rule| {
            if (contains(u16, upd, rule[0]) and contains(u16, upd, rule[1])) {
                try graph.addEdge(rule[0], rule[1], false);
            }
        }

        // Find topological sorting for these pages
        const sorted_pages = try graph.topoSorting(allocator);

        // Sort by rules
        const sortFunc = struct {
            fn lessThan(context: []u16, lhs: u16, rhs: u16) bool {
                return std.mem.indexOf(u16, context, &.{lhs}).? < std.mem.indexOf(u16, context, &.{rhs}).?;
            }
        }.lessThan;
        std.sort.pdq(u16, upd, sorted_pages, sortFunc);

        // Sum middle
        const middle = upd[(upd.len - 1) / 2];
        result += middle;
    }

    return Result{ .UInt32 = result };
}

fn parse(input: *ProblemInput, allocator: std.mem.Allocator) !struct { aoc_lib.graph.Graph(u16), [][]u16 } {
    var sep: usize = undefined; // Find index where rules stop and updates start

    var rules = aoc_lib.graph.Graph(u16).init(allocator);
    for (input.lines, 0..) |rule, i| {
        if (rule.len == 0) {
            sep = i;
            break;
        }

        var tokens = std.mem.tokenizeScalar(u8, rule, '|');
        const from = try std.fmt.parseInt(u16, tokens.next().?, 10);
        const to = try std.fmt.parseInt(u16, tokens.next().?, 10);
        try rules.addEdge(from, to, false);
    }

    var updates = std.ArrayList([]u16).init(allocator);
    for (input.lines[sep + 1 ..]) |update| {
        var tokens = std.mem.tokenizeScalar(u8, update, ',');
        var pages = std.ArrayList(u16).init(allocator);
        while (tokens.next()) |page| {
            try pages.append(try std.fmt.parseInt(u16, page, 10));
        }
        try updates.append(try pages.toOwnedSlice());
    }

    return .{ rules, try updates.toOwnedSlice() };
}

fn isOrdered(update: []u16, rules: *aoc_lib.graph.Graph(u16)) bool {
    for (update, 0..) |page1, i| {
        for (update[i + 1 ..]) |page2| {
            if (!rules.hasEdge(page1, page2))
                return false;
        }
    }
    return true;
}
