const std = @import("std");
const aoc_lib = @import("aoc_lib");
const set = @import("ziglangSet");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Grid = aoc_lib.types.Grid;
const Result = aoc_lib.types.Result;
const PathFinding = aoc_lib.graph.PathFinding;
const Index = aoc_lib.cartesian.Index;
const Direction = aoc_lib.cartesian.Direction;

pub fn results() [4]Result {
    return .{
        Result{ .Int64 = 7036 },
        Result{ .Int64 = 143580 },
        Result{ .USize = 45 },
        Result{ .USize = 645 },
    };
}

pub fn solvePart01(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    const map = try input.grid();
    const start = Node{ .idx = map.find('S').?, .dir = Direction.East };
    const end = map.find('E').?;

    const maze = Maze{ .map = map, .end_idx = end };
    var pathfinder = PathFinding(Maze, Node){ .base = maze };

    const shortest_path = try pathfinder.astar_any(start, allocator);

    return Result{ .Int64 = shortest_path.?.cost };
}

pub fn solvePart02(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    const map = try input.grid();
    const start = Node{ .idx = map.find('S').?, .dir = Direction.East };
    const end = map.find('E').?;

    const maze = Maze{ .map = map, .end_idx = end };
    var pathfinder = PathFinding(Maze, Node){ .base = maze };

    const shortest_paths = try pathfinder.astar_all(start, allocator);

    var visited_tiles = set.Set(Index).init(allocator);
    for (shortest_paths) |result| {
        // std.debug.print("{d} {d}\n", .{ result.cost, result.path.len });
        for (result.path) |node| {
            _ = try visited_tiles.add(node.idx);
        }
    }

    return Result{ .USize = visited_tiles.cardinality() };
}

const Node = struct { idx: Index, dir: Direction };
const Maze = struct {
    map: Grid(u8),
    end_idx: Index,

    const Self = @This();
    const Child = PathFinding(Maze, Node).Child;

    pub fn next(self: *Self, node: Node, allocator: std.mem.Allocator) ![]Child {
        var next_list = std.ArrayList(Child).init(allocator);

        // Add turns
        try next_list.append(.{ .node = .{ .idx = node.idx, .dir = node.dir.left() }, .weight = 1000 });
        try next_list.append(.{ .node = .{ .idx = node.idx, .dir = node.dir.right() }, .weight = 1000 });

        // Add move
        const move = node.idx.scout(node.dir, self.map.size);
        if (move != null and self.map.get(move.?) != '#')
            try next_list.append(.{ .node = .{ .idx = move.?, .dir = node.dir }, .weight = 1 });

        return next_list.toOwnedSlice();
    }

    pub fn heuristic(self: *Self, node: Node) i64 {
        return @intCast(self.end_idx.dist(node.idx));
    }

    pub fn isEnd(self: *Self, node: Node) bool {
        return std.meta.eql(node.idx, self.end_idx);
    }
};
