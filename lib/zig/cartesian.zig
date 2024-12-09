const std = @import("std");

pub const Index = struct {
    r: usize,
    c: usize,

    const Self = @This();

    pub fn move(self: Self, dir: Direction, size: Size) ?Index {
        // Prevent underflow by wrapping sub
        const next = switch (dir) {
            .North => Index{ .r = self.r -% 1, .c = self.c },
            .East => Index{ .r = self.r, .c = self.c + 1 },
            .South => Index{ .r = self.r + 1, .c = self.c },
            .West => Index{ .r = self.r, .c = self.c -% 1 },
        };

        return if (next.r < size.height and next.c < size.width) next else null;
    }

    pub fn asPosition(self: Self) Position {
        return Position{ .x = @intCast(self.c), .y = @intCast(self.r) };
    }

    pub fn vonNeumann(self: Self, size: Size, allocator: std.mem.Allocator) ![]Index {
        var nbs = std.ArrayList(Index).init(allocator);
        if (self.move(Direction.North, size)) |next| try nbs.append(next);
        if (self.move(Direction.East, size)) |next| try nbs.append(next);
        if (self.move(Direction.South, size)) |next| try nbs.append(next);
        if (self.move(Direction.West, size)) |next| try nbs.append(next);

        return nbs.toOwnedSlice();
    }
};

pub const Position = struct {
    x: i64,
    y: i64,

    const Self = @This();

    pub fn move(self: Self, dir: Direction) Position {
        return switch (dir) {
            .North => Position{ .x = self.x - 1, .y = self.y },
            .East => Position{ .x = self.x, .y = self.y + 1 },
            .South => Position{ .x = self.x + 1, .y = self.y },
            .West => Position{ .x = self.x, .y = self.y - 1 },
        };
    }

    pub fn diff(self: Self, other: Self) PosDelta {
        return .{ .dx = other.x - self.x, .dy = other.y - self.y };
    }

    pub fn offset(self: Self, dx: i64, dy: i64) Position {
        return Position{ .x = self.x + dx, .y = self.y + dy };
    }

    pub fn asIndex(self: Self, size: Size) ?Index {
        if (!(0 <= self.y and self.y < size.height and 0 <= self.x and self.x < size.width)) return null;
        return Index{ .r = @intCast(self.y), .c = @intCast(self.x) };
    }
};

pub const PosDelta = struct { dx: i64, dy: i64 };

pub const Size = struct {
    width: usize,
    height: usize,
    diags: usize,

    const Self = @This();

    pub fn total(self: Self) usize {
        return self.width * self.height;
    }
};

pub const Direction = enum(u4) {
    North = 0b0001,
    East = 0b0010,
    South = 0b0100,
    West = 0b1000,

    const Self = @This();

    pub fn left(self: Self) Direction {
        return switch (self) {
            .North => .West,
            .East => .North,
            .South => .East,
            .West => .South,
        };
    }

    pub fn right(self: Self) Direction {
        return switch (self) {
            .North => .East,
            .East => .South,
            .South => .West,
            .West => .North,
        };
    }
};
