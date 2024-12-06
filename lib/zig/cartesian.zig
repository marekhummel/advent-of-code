const std = @import("std");

pub const Index = struct {
    r: usize,
    c: usize,

    const Self = @This();

    pub fn move(self: Self, dir: Direction, width: usize, height: usize) ?Index {
        // Prevent underflow by wrapping sub
        const next = switch (dir) {
            .North => Index{ .r = self.r -% 1, .c = self.c },
            .East => Index{ .r = self.r, .c = self.c + 1 },
            .South => Index{ .r = self.r + 1, .c = self.c },
            .West => Index{ .r = self.r, .c = self.c -% 1 },
        };

        return if (next.r < height and next.c < width) next else null;
    }
};

pub const Direction = enum {
    North,
    East,
    South,
    West,

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
