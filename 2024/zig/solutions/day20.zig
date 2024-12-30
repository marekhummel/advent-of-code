const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const Grid = aoc_lib.types.Grid;
const Index = aoc_lib.cartesian.Index;
const Direction = aoc_lib.cartesian.Direction;
const Size = aoc_lib.cartesian.Size;

pub fn results() [4]Result {
    return .{
        Result{ .USize = 5 },
        Result{ .USize = 1502 },
        Result{ .USize = 285 },
        Result{ .USize = 1028136 },
    };
}

pub fn solvePart01(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    const min_saved: u32 = if (!is_sample) 100 else 20;
    var course = try input.grid();
    const helpful_cheats = try countCheats(&course, .{ 2, 2 }, min_saved, allocator);
    return Result{ .USize = helpful_cheats };
}

pub fn solvePart02(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    const min_saved: u32 = if (!is_sample) 100 else 50;
    var course = try input.grid();
    const helpful_cheats = try countCheats(&course, .{ 2, 20 }, min_saved, allocator);
    return Result{ .USize = helpful_cheats };
}

fn countCheats(course: *Grid(u8), cheat_length: [2]usize, min_saved_time: usize, allocator: Allocator) !usize {
    const start = course.find('S').?;
    const end = course.find('E').?;

    // Build racetrack
    var racetrack_list = std.ArrayList(Index).init(allocator);
    try racetrack_list.append(start);

    var node = start;
    while (!std.meta.eql(node, end)) {
        for (Direction.compass()) |dir| {
            if (node.scout(dir, course.size)) |move| {
                if (course.get(move) != '#') {
                    course.set(node, '#');
                    node = move;
                    try racetrack_list.append(node);
                    break;
                }
            }
        }
    }
    const racetrack = try racetrack_list.toOwnedSlice();

    // Find all cheats, exploiting that there is only one racetrack, so no need for elaborate pathfinding
    // Just take two points on the track, link them, and check how much time it saves
    var helpful_cheats: usize = 0;
    for (0..racetrack.len - 2) |i| {
        for (i + cheat_length[0] + 1..racetrack.len) |j| {
            const dist = racetrack[i].dist(racetrack[j]);
            if (cheat_length[0] <= dist and dist <= cheat_length[1]) {
                const saved: usize = j - i - dist;
                if (saved >= min_saved_time) helpful_cheats += 1;
            }
        }
    }

    return helpful_cheats;
}
