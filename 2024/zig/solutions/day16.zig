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
        Result{ .UInt32 = 7036 },
        Result{ .UInt32 = 143580 },
        Result{ .USize = 45 },
        Result{ .USize = 645 },
    };
}

pub fn solvePart01(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;

    const astar_result = try runAstar(try input.grid(), false, allocator);
    return Result{ .UInt32 = astar_result.cost };
}

pub fn solvePart02(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;

    const astar_result = try runAstar(try input.grid(), true, allocator);

    var visited_tiles = set.Set(Index).init(allocator);
    for (try astar_result.paths(allocator)) |result| {
        for (result) |node| {
            _ = try visited_tiles.add(node.idx);
        }
    }

    return Result{ .USize = visited_tiles.cardinality() };
}

fn runAstar(map: Grid(u8), all_paths: bool, allocator: Allocator) !PathFinding(Maze).AStarResult {
    const start = Maze.Node{ .idx = map.find('S').?, .dir = Direction.East };
    const end = map.find('E').?;

    const maze = Maze{ .map = map, .end_idx = end };
    var pathfinder = PathFinding(Maze){ .base = &maze };

    return (try pathfinder.astar(start, !all_paths, allocator)).?;
}

const Maze = struct {
    map: Grid(u8),
    end_idx: Index,

    const Self = @This();
    const Child = PathFinding(Self).Child;

    pub const Node = struct { idx: Index, dir: Direction };

    pub fn next(self: *const Self, node: Node, allocator: std.mem.Allocator) ![]Child {
        var next_list = std.ArrayList(Child).init(allocator);

        // Add turns
        try next_list.append(.{ .node = .{ .idx = node.idx, .dir = node.dir.left() }, .weight = 1000 });
        try next_list.append(.{ .node = .{ .idx = node.idx, .dir = node.dir.right() }, .weight = 1000 });

        // Add move if no wall
        const move = node.idx.scout(node.dir, self.map.size);
        if (move != null and self.map.get(move.?) != '#')
            try next_list.append(.{ .node = .{ .idx = move.?, .dir = node.dir }, .weight = 1 });

        return next_list.toOwnedSlice();
    }

    pub fn heuristic(self: *const Self, node: Node) u32 {
        // L1 dist to end, add 1000 if we know we have to turn at least once (still admissable)
        const turn_penalty: usize = if (self.end_idx.r != node.idx.r or self.end_idx.c != node.idx.c) 1000 else 0;
        return @intCast(self.end_idx.dist(node.idx) + turn_penalty);
    }

    pub fn isEnd(self: *const Self, node: Node) bool {
        return std.meta.eql(node.idx, self.end_idx);
    }
};
