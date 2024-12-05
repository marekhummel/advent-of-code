const std = @import("std");

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
