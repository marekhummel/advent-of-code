const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const Grid = aoc_lib.types.Grid;
const Index = aoc_lib.cartesian.Index;
const Direction = aoc_lib.cartesian.Direction;
const Size = aoc_lib.cartesian.Size;
const PathFinding = aoc_lib.graph.PathFinding;
const freeNested = aoc_lib.util.freeNested;

pub fn results() [4]Result {
    return .{
        Result{ .UInt32 = 22 },
        Result{ .UInt32 = 416 },
        Result{ .String = "6,1" },
        Result{ .String = "50,23" },
    };
}

pub fn solvePart01(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    const size: Size = if (!is_sample) Size{ .width = 71, .height = 71 } else Size{ .width = 7, .height = 7 };
    const sim_length: usize = if (!is_sample) 1024 else 12;

    const falling_bytes = try parseInput(input, allocator);
    const escapeSteps = try escape(size, falling_bytes[0..sim_length], allocator);
    return Result{ .UInt32 = escapeSteps.? };
}

pub fn solvePart02(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    const size: Size = if (!is_sample) Size{ .width = 71, .height = 71 } else Size{ .width = 7, .height = 7 };
    const falling_bytes = try parseInput(input, allocator);

    // Binary search to find blocking byte
    var lo: usize = 0;
    var hi: usize = input.lines.len - 1;
    while (hi - lo > 1) {
        const mid = (lo + hi) / 2;
        const escapeSteps = try escape(size, falling_bytes[0 .. mid + 1], allocator);
        if (escapeSteps != null) lo = mid else hi = mid;
    }

    const blocking_byte = falling_bytes[hi];
    return Result{ .String = try std.fmt.allocPrint(allocator, "{d},{d}", .{ blocking_byte.c, blocking_byte.r }) };
}

fn parseInput(input: *ProblemInput, allocator: Allocator) ![]Index {
    const falling_bytes = try allocator.alloc(Index, input.lines.len);
    for (input.lines, 0..) |line, i| {
        var coord_it = std.mem.tokenizeScalar(u8, line, ',');
        const c = try std.fmt.parseInt(usize, coord_it.next().?, 10);
        const r = try std.fmt.parseInt(usize, coord_it.next().?, 10);
        falling_bytes[i] = Index{ .r = r, .c = c };
    }
    return falling_bytes;
}

fn escape(size: Size, falling_bytes: []Index, allocator: Allocator) !?u32 {
    const start = Index{ .r = 0, .c = 0 };
    const end = Index{ .r = size.height - 1, .c = size.width - 1 };

    // Let bytes fall
    const grid = try Grid(bool).empty(size, true, allocator);
    defer grid.deinit(allocator);
    for (falling_bytes) |byte| grid.set(byte, false);

    // Use astar
    var mem_space = MemorySpace{ .map = grid, .end_idx = end };
    var pathfinder = PathFinding(MemorySpace){ .base = &mem_space };
    var astar_result = try pathfinder.astar(start, true, allocator);
    defer if (astar_result != null) astar_result.?.deinit(allocator);
    return if (astar_result) |res| res.cost else null;
}

const MemorySpace = struct {
    map: Grid(bool),
    end_idx: Index,

    const Self = @This();
    const Child = PathFinding(Self).Child;

    pub const Node = Index;

    pub fn next(self: *const Self, node: Node, allocator: std.mem.Allocator) ![]Child {
        var next_list = std.ArrayList(Child).init(allocator);

        for (Direction.compass()) |dir| {
            const move = node.scout(dir, self.map.size);
            if (move != null and self.map.get(move.?)) {
                try next_list.append(.{ .node = move.?, .weight = 1 });
            }
        }

        return next_list.toOwnedSlice();
    }

    pub fn heuristic(self: *const Self, node: Node) u32 {
        return @intCast(self.end_idx.dist(node));
    }

    pub fn isEnd(self: *const Self, node: Node) bool {
        return std.meta.eql(node, self.end_idx);
    }
};
