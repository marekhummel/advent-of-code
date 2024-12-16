const std = @import("std");
const set = @import("ziglangSet");
const math = std.math;

pub fn Graph(comptime T: type) type {
    return struct {
        const Self = @This();
        adjacency: std.AutoHashMap(T, std.AutoArrayHashMap(T, i32)),
        _vertices: std.AutoArrayHashMap(T, void),
        _allocator: std.mem.Allocator,

        pub fn init(allocator: std.mem.Allocator) Graph(T) {
            return .{
                .adjacency = std.AutoHashMap(T, std.AutoArrayHashMap(T, i32)).init(allocator),
                ._vertices = std.AutoArrayHashMap(T, void).init(allocator),
                ._allocator = allocator,
            };
        }

        pub fn deinit(self: *Self) void {
            self._vertices.deinit();
            var values = self.adjacency.valueIterator();
            while (values.next()) |val| val.deinit();
            self.adjacency.deinit();
        }

        pub fn hasEdge(self: *const Self, from: T, to: T) bool {
            const adjecent = self.adjacency.get(from);
            return adjecent != null and adjecent.?.contains(to);
        }

        pub fn addEdge(self: *Self, from: T, to: T, bidirectional: bool) !void {
            if (bidirectional) try self.addEdge(to, from, false);
            try self._vertices.put(from, {});
            try self._vertices.put(to, {});

            const entry = try self.adjacency.getOrPut(from);
            if (!entry.found_existing)
                entry.value_ptr.* = std.AutoArrayHashMap(T, i32).init(self._allocator);
            try entry.value_ptr.*.put(to, 0);
        }

        /// Slice of all outgoing nodes from this node
        pub fn outgoing(self: *const Self, node: T, allocator: std.mem.Allocator) ![]T {
            var nodes = std.ArrayList(T).init(allocator);

            if (self.adjacency.contains(node)) {
                for (self.adjacency.get(node).?.keys()) |trg| {
                    try nodes.append(trg);
                }
            }

            return nodes.toOwnedSlice();
        }

        /// True for nodes that have no outgoing edges
        pub fn isSink(self: *const Self, node: T) bool {
            return (!self.adjacency.contains(node) or self.adjacency.get(node).?.count() == 0);
        }

        /// Slice of all edges of graph
        pub fn edges(self: *const Self, allocator: std.mem.Allocator) ![][2]T {
            var edgelist = std.ArrayList([2]T).init(allocator);

            var node_iterator = self.adjacency.keyIterator();
            while (node_iterator.next()) |from| {
                for (self.adjacency.get(from.*).?.keys()) |to| {
                    try edgelist.append(.{ from.*, to });
                }
            }

            return edgelist.toOwnedSlice();
        }

        /// Inverting all edges
        pub fn invert(self: *Self, allocator: std.mem.Allocator) !Graph(T) {
            var inverted = Self.init(allocator);

            var node_iterator = self.adjacency.keyIterator();
            while (node_iterator.next()) |from| {
                for (self.adjacency.get(from.*).?.keys()) |to| {
                    try inverted.addEdge(to, from.*, false);
                }
            }

            return inverted;
        }

        /// Topological sorting with Kahn's algorithm
        pub fn topoSorting(self: *Self, allocator: std.mem.Allocator) ![]T {
            // Invert graph, because we need to check for node with no incoming edges
            var inverted = try self.invert(self._allocator);
            defer inverted.deinit();

            var sorted = std.ArrayList(T).init(allocator);

            // Start with all sinks (in inverted graph)
            var nodes = std.AutoArrayHashMap(T, void).init(allocator);
            defer nodes.deinit();
            for (inverted._vertices.keys()) |node| {
                if (inverted.isSink(node)) {
                    try nodes.put(node, {});
                }
            }

            // Loop will sinks left
            while (nodes.count() > 0) {
                // Add sink to list, remove their edges, find new sinks
                const n = nodes.pop().key;
                try sorted.append(n);

                const targets = try self.outgoing(n, allocator);
                defer allocator.free(targets);
                for (targets) |m| {
                    _ = inverted.adjacency.getPtr(m).?.swapRemove(n);
                    if (inverted.isSink(m)) {
                        try nodes.put(m, {});
                    }
                }
            }

            return sorted.toOwnedSlice();
        }
    };
}

pub fn PathFinding(comptime T: anytype, comptime NT: type) type {
    return struct {
        base: T,

        const Self = @This();
        const OpenQueueItem = struct {
            dist: i64,
            node: NT,
            path: []NT,

            fn lessThan(context: void, a: OpenQueueItem, b: OpenQueueItem) math.Order {
                _ = context;
                return math.order(a.dist, b.dist);
            }
        };
        const AStarResult = struct { cost: i64, path: []NT };

        const AStarIterator = struct {
            pf: *PathFinding(T, NT),
            start: NT,
            open: std.PriorityQueue(OpenQueueItem, void, OpenQueueItem.lessThan),
            closed: set.Set(NT),
            g_score_map: std.AutoHashMap(NT, i64),
            allocator: std.mem.Allocator,
            best_path_cost: ?i64 = null,

            const ItSelf = @This();

            fn init(pf: *PathFinding(T, NT), start: NT, allocator: std.mem.Allocator) !AStarIterator {
                var open = std.PriorityQueue(OpenQueueItem, void, OpenQueueItem.lessThan).init(allocator, {});
                const closed = set.Set(NT).init(allocator);

                var path = try allocator.alloc(NT, 1);
                path[0] = start;
                try open.add(.{ .dist = 0, .node = start, .path = path });

                var g_score_map = std.AutoHashMap(NT, i64).init(allocator);
                try g_score_map.put(start, 0);

                return .{
                    .pf = pf,
                    .start = start,
                    .open = open,
                    .closed = closed,
                    .g_score_map = g_score_map,
                    .allocator = allocator,
                };
            }

            fn deinit(self: *ItSelf) void {
                self.open.deinit();
                self.closed.deinit();
                self.g_score_map.deinit();
            }

            fn next(self: *ItSelf) !?AStarResult {
                while (self.open.removeOrNull()) |current| {
                    // std.debug.print("{any}\n", .{current.node});

                    // Stop if lowest possible node already exceeds best cost solution
                    if (self.best_path_cost != null and current.dist > self.best_path_cost.?)
                        break;

                    // _ = try self.closed.add(current.node);

                    const g = self.g_score_map.get(current.node).?;

                    // Found the goal
                    if (self.pf.isEnd(current.node)) {
                        self.best_path_cost = self.best_path_cost orelse g;
                        return .{ .cost = g, .path = current.path };
                    }
                    defer self.allocator.free(current.path);

                    const children = try self.pf.next(current.node, self.allocator);
                    defer self.allocator.free(children);
                    for (children) |child| {
                        // if (self.closed.contains(child.node)) continue;

                        const child_g = g + child.weight;
                        if (self.g_score_map.contains(child.node) and self.g_score_map.get(child.node).? < child_g) continue;
                        try self.g_score_map.put(child.node, child_g);

                        const child_h = self.pf.heuristic(child.node);
                        const child_f = child_g + child_h;

                        var child_path = try self.allocator.alloc(NT, current.path.len + 1);
                        std.mem.copyForwards(NT, child_path, current.path);
                        child_path[current.path.len] = child.node;
                        try self.open.add(.{ .dist = child_f, .node = child.node, .path = child_path });
                    }
                }

                return null;
            }
        };

        pub fn astar_any(self: *Self, start: NT, allocator: std.mem.Allocator) !?AStarResult {
            var iterator = try AStarIterator.init(self, start, allocator);
            return iterator.next();
        }

        pub fn astar_all(self: *Self, start: NT, allocator: std.mem.Allocator) ![]AStarResult {
            var results = std.ArrayList(AStarResult).init(allocator);
            var iterator = try AStarIterator.init(self, start, allocator);
            while (try iterator.next()) |result| try results.append(result);
            return results.toOwnedSlice();
        }

        pub const Child = struct { node: NT, weight: i64 };
        fn next(self: *Self, node: NT, allocator: std.mem.Allocator) ![]Child {
            return try self.base.next(node, allocator);
        }

        fn heuristic(self: *Self, node: NT) i64 {
            return self.base.heuristic(node);
        }

        fn isEnd(self: *Self, node: NT) bool {
            return self.base.isEnd(node);
        }
    };
}
