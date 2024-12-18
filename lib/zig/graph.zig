const std = @import("std");
const util = @import("util.zig");
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

pub fn PathFinding(comptime T: type) type {
    comptime std.debug.assert(@hasDecl(T, "Node"));
    comptime std.debug.assert(@hasDecl(T, "next"));
    comptime std.debug.assert(@hasDecl(T, "heuristic"));
    comptime std.debug.assert(@hasDecl(T, "isEnd"));

    return struct {
        base: *const T,

        const Self = @This();
        const OpenQueueItem = struct {
            f_score: u32,
            node: T.Node,

            fn lessThan(context: *std.AutoHashMap(T.Node, u32), a: OpenQueueItem, b: OpenQueueItem) math.Order {
                // Include g score in order, so that nodes that are closer to end are preferred
                const f_score_order = math.order(a.f_score, b.f_score);
                return f_score_order.differ() orelse math.order(context.get(a.node).?, context.get(b.node).?).invert();
            }

            pub fn format(self: @This(), comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
                _ = fmt;
                _ = options;
                return std.fmt.format(writer, "({any}, {d})", .{ self.node, self.f_score });
            }
        };

        pub const Child = struct { node: T.Node, weight: u32 };

        pub const AStarResult = struct {
            cost: u32,
            reached_end_nodes: []T.Node,
            history: std.AutoHashMap(T.Node, std.ArrayList(T.Node)),

            /// Computes all paths that lead with minimal cost to an end state
            pub fn paths(self: *const @This(), allocator: std.mem.Allocator) ![][]T.Node {
                var path_list = std.ArrayList([]T.Node).init(allocator);

                // Create path for each end state
                for (self.reached_end_nodes) |end_node| {
                    const constructed_paths = try self.constructPaths(end_node, allocator);
                    try path_list.appendSlice(constructed_paths);
                    allocator.free(constructed_paths);
                }

                return path_list.toOwnedSlice();
            }

            /// Recursively constructs the paths with the history
            fn constructPaths(self: *const @This(), node: T.Node, allocator: std.mem.Allocator) ![][]T.Node {
                var path_list = std.ArrayList([]T.Node).init(allocator);
                if (self.history.get(node)) |prevs| {
                    // Take all previous nodes and create paths for each of them
                    for (prevs.items) |prev| {
                        const constructed_paths = try self.constructPaths(prev, allocator);
                        defer allocator.free(constructed_paths);
                        for (constructed_paths) |prev_path| {
                            defer allocator.free(prev_path);
                            var new_path = std.ArrayList(T.Node).init(allocator);
                            try new_path.appendSlice(prev_path);
                            try new_path.append(node);
                            try path_list.append(try new_path.toOwnedSlice());
                        }
                    }
                } else {
                    // No history, so this is the start node
                    var new_path = std.ArrayList(T.Node).init(allocator);
                    try new_path.append(node);
                    try path_list.append(try new_path.toOwnedSlice());
                }

                return try path_list.toOwnedSlice();
            }

            pub fn deinit(self: *@This(), allocator: std.mem.Allocator) void {
                allocator.free(self.reached_end_nodes);

                var values = self.history.valueIterator();
                while (values.next()) |val| val.deinit();
                self.history.deinit();
            }
        };

        /// AStar algorithm. Note that if neglect_other_paths is true, the paths construction in the result won't be complete.
        pub fn astar(self: *const Self, start: T.Node, neglect_other_paths: bool, allocator: std.mem.Allocator) !?AStarResult {
            // Link to previous nodes and list of reached final nodes for path tracking
            var history = std.AutoHashMap(T.Node, std.ArrayList(T.Node)).init(allocator);
            var end_nodes = std.ArrayList(T.Node).init(allocator);

            // Cost map
            var g_score_map = std.AutoHashMap(T.Node, u32).init(allocator);
            try g_score_map.put(start, 0);
            defer g_score_map.deinit();

            // Frontier of nodes
            var open = std.PriorityQueue(OpenQueueItem, *@TypeOf(g_score_map), OpenQueueItem.lessThan).init(allocator, &g_score_map);
            try open.add(.{ .f_score = 0, .node = start });
            defer open.deinit();

            // Best cost once found
            var best_path_cost: ?u32 = null;

            while (open.removeOrNull()) |current| {
                // Stop if lowest possible node already exceeds best cost solution (assumes admissible heuristic)
                if (best_path_cost != null and current.f_score > best_path_cost.?)
                    break;

                const g = g_score_map.get(current.node).?;
                // std.debug.print("{any} ({d} {d}) - {any}\n", .{ current.node, g, current.f_score, open.items });

                // Found the goal
                if (self.base.isEnd(current.node)) {
                    best_path_cost = best_path_cost orelse g;
                    try end_nodes.append(current.node);
                    if (neglect_other_paths) break;
                }

                // Append all successor nodes to frontier
                const children: []Child = try self.base.next(current.node, allocator);
                defer allocator.free(children);
                for (children) |child| {
                    const child_g = g + child.weight;

                    // If this child node is already evaluated, skip if this g value is worse
                    if (g_score_map.get(child.node)) |prev_child_g| {
                        if (prev_child_g <= child_g) {
                            // If g values are equal, we have multiple paths with same cost, so store both in history
                            if (prev_child_g == child_g) {
                                try history.getPtr(child.node).?.append(current.node);
                            }
                            continue;
                        }
                    }
                    try g_score_map.put(child.node, child_g);

                    // Compute heuristical value for frontier queue
                    const child_h: u32 = self.base.heuristic(child.node);
                    const child_f = child_g + child_h;
                    try open.add(.{ .f_score = child_f, .node = child.node });

                    // Since we have a better g value, update the history
                    if (history.getPtr(child.node)) |old_prev| old_prev.deinit();
                    var prevs = std.ArrayList(T.Node).init(allocator);
                    try prevs.append(current.node);
                    try history.put(child.node, prevs);
                }
            }

            // If best cost is still empty, goal was never found, otherwise create result
            if (best_path_cost) |cost| {
                return .{
                    .cost = cost,
                    .reached_end_nodes = try end_nodes.toOwnedSlice(),
                    .history = history,
                };
            } else {
                end_nodes.deinit();
                var values = history.valueIterator();
                while (values.next()) |val| val.deinit();
                history.deinit();
                return null;
            }
        }
    };
}
