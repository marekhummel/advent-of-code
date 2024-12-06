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
