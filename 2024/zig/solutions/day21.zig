const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const Index = aoc_lib.cartesian.Index;

pub fn results() [4]Result {
    return .{
        Result{ .UInt64 = 126384 },
        Result{ .UInt64 = 205160 },
        Result{ .UInt64 = 154115708116294 },
        Result{ .UInt64 = 252473394928452 },
    };
}

pub fn solvePart01(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;

    var robots = try Robots.init(allocator);
    const total_complexity = try robots.findComplexity(input.lines, 2);
    return Result{ .UInt64 = total_complexity };
}

pub fn solvePart02(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;

    var robots = try Robots.init(allocator);
    const total_complexity = try robots.findComplexity(input.lines, 25);
    return Result{ .UInt64 = total_complexity };
}

const Robots = struct {
    const Self = @This();
    const ButtonSequence = []const u8;
    const Keypad = std.AutoHashMap(u8, Index);

    numpad: Keypad,
    arrowpad: Keypad,
    _allocator: Allocator,
    _robotCache: std.AutoHashMap(usize, std.StringHashMap(u64)), // nested hashmap, cause struct key would need custom hash fn

    fn init(allocator: Allocator) !Self {
        var arrowpad = Keypad.init(allocator);
        for ("-^A<v>", 0..) |button, idx|
            try arrowpad.put(button, .{ .r = idx / 3, .c = idx % 3 });

        var numpad = Keypad.init(allocator);
        for ("789456123-0A", 0..) |button, idx|
            try numpad.put(button, .{ .r = idx / 3, .c = idx % 3 });

        return .{
            .numpad = numpad,
            .arrowpad = arrowpad,
            ._allocator = allocator,
            ._robotCache = std.AutoHashMap(usize, std.StringHashMap(u64)).init(allocator),
        };
    }

    fn findComplexity(self: *Self, codes: []ButtonSequence, arrowpad_robots: usize) !u64 {
        var complexity: u64 = 0;
        for (codes) |code| {
            const code_len = try self.controlAllRobots(code, 0, arrowpad_robots + 1);
            complexity += code_len * (try std.fmt.parseInt(u64, code[0..3], 10));
        }
        return complexity;
    }

    fn controlAllRobots(self: *Self, buttons: ButtonSequence, curr_robot: usize, last_robot: usize) !u64 {
        // Last robot is us, we just press the buttons
        if (curr_robot == last_robot) return buttons.len;

        // Caching
        const solve_cache_entry = try self._robotCache.getOrPut(curr_robot);
        if (!solve_cache_entry.found_existing) solve_cache_entry.value_ptr.* = std.StringHashMap(u64).init(self._allocator);
        if (solve_cache_entry.value_ptr.get(buttons)) |length| return length;

        // Get list of button sequences for next robot, recurse for each of them
        const next_buttons_list = try self.controlRobot(buttons, curr_robot == 0);
        var result: u64 = 0;
        for (next_buttons_list) |next_buttons| {
            result += try self.controlAllRobots(next_buttons, curr_robot + 1, last_robot);
        }

        try self._robotCache.getPtr(curr_robot).?.put(buttons, result);
        return result;
    }

    fn controlRobot(self: *Self, buttons: ButtonSequence, use_numpad: bool) ![]ButtonSequence {
        // Could cache here as well, but evidently not needed
        const keypad = if (use_numpad) self.numpad else self.arrowpad;
        var pos = keypad.get('A').?;
        var all_presses = std.ArrayList(ButtonSequence).init(self._allocator);
        for (buttons) |button| {
            const next_pos = keypad.get(button).?;
            const presses = try self.findPath(button, pos, use_numpad);
            try all_presses.append(presses);
            pos = next_pos;
        }

        const slice = try all_presses.toOwnedSlice();
        return slice;
    }

    fn findPath(self: *Self, target: u8, pos: Index, use_numpad: bool) !ButtonSequence {
        const keypad = if (use_numpad) self.numpad else self.arrowpad;
        const target_pos = keypad.get(target).?;

        // Default order, because we distance from the A first and then return back
        var path = std.ArrayList(u8).init(self._allocator);
        for (0..pos.c -| target_pos.c) |_| try path.append('<');
        for (0..target_pos.r -| pos.r) |_| try path.append('v');
        for (0..pos.r -| target_pos.r) |_| try path.append('^');
        for (0..target_pos.c -| pos.c) |_| try path.append('>');

        // There is a set movement order, but due to one missing corner some edge cases, where we go exactly opposite
        const crossing_empty_num = (pos.r == 3 and target_pos.c == 0) or (target_pos.r == 3 and pos.c == 0);
        const crossing_empty_arrow = (pos.r == 0 and target_pos.c == 0) or (target_pos.r == 0 and pos.c == 0);
        if ((use_numpad and crossing_empty_num) or (!use_numpad and crossing_empty_arrow))
            std.mem.reverse(u8, path.items);

        // Finish with A button
        try path.append('A');
        return try path.toOwnedSlice();
    }
};
