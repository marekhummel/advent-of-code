const std = @import("std");
const aoc_lib = @import("aoc_lib");
const regex = @import("zigRegex");
const set = @import("ziglangSet");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const Position = aoc_lib.cartesian.Position;
const PosDelta = aoc_lib.cartesian.PosDelta;
const Size = aoc_lib.cartesian.Size;

pub fn solvePart01(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    const size = if (!is_sample) Size{ .width = 101, .height = 103 } else Size{ .width = 11, .height = 7 };

    var quadrants = [_]usize{ 0, 0, 0, 0 };
    for (input.lines) |line| {
        const robot = try Robot.parse(line, allocator);
        const pos = robot.at(100, size);

        const left = (0 <= pos.x and pos.x < size.width / 2);
        const right = (size.width / 2 < pos.x and pos.x < size.width);
        const top = (0 <= pos.y and pos.y < size.height / 2);
        const bot = (size.height / 2 < pos.y and pos.y < size.height);
        if (left and top) quadrants[0] += 1;
        if (right and top) quadrants[1] += 1;
        if (left and bot) quadrants[2] += 1;
        if (right and bot) quadrants[3] += 1;
    }

    const safety_factor = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];
    return Result{ .USize = safety_factor };
}

pub fn solvePart02(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    const size = if (!is_sample) Size{ .width = 101, .height = 103 } else return Result.NoSample;

    var robots = try std.ArrayList(Robot).initCapacity(allocator, input.lines.len);
    for (input.lines) |line| {
        robots.appendAssumeCapacity(try Robot.parse(line, allocator));
    }

    var second: usize = 1;
    time: while (true) : (second += 1) {
        // If there are no stacked robots we find the tree (stupid conditional, but works)
        var uniquePositions = set.Set(Position).empty;
        for (robots.items) |rbt| {
            if (!try uniquePositions.add(allocator, rbt.at(second, size)))
                continue :time;
        }

        // printRobots(&uniquePositions, size);
        return Result{ .USize = second };
    }
}

const Robot = struct {
    pos: Position,
    vel: PosDelta,

    fn parse(line: []u8, allocator: Allocator) !Robot {
        var rgx = try regex.Regex.compile(allocator, "p=(\\d+),(\\d+) v=(-?\\d+),(-?\\d+)");
        defer rgx.deinit();
        var captures = (try rgx.find(line)).?;
        defer captures.deinit(allocator);
        const px: i64 = try std.fmt.parseInt(i64, captures.captures[0], 10);
        const py: i64 = try std.fmt.parseInt(i64, captures.captures[1], 10);
        const vx: i64 = try std.fmt.parseInt(i64, captures.captures[2], 10);
        const vy: i64 = try std.fmt.parseInt(i64, captures.captures[3], 10);

        return Robot{ .pos = Position{ .x = px, .y = py }, .vel = PosDelta{ .dx = vx, .dy = vy } };
    }

    fn at(self: @This(), second: usize, size: Size) Position {
        const factor = @as(i64, @intCast(second));
        return self.pos.offsetWrap(self.vel.dx * factor, self.vel.dy * factor, size);
    }
};

fn printRobots(robots: *set.Set(Position), size: Size) void {
    for (0..size.height) |y| {
        for (0..size.width) |x| {
            const pos = Position{ .x = @as(i64, @intCast(x)), .y = @as(i64, @intCast(y)) };
            const char: u8 = if (robots.contains(pos)) '#' else '.';
            std.debug.print("{c}", .{char});
        }
        std.debug.print("\n", .{});
    }
}
