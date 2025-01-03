const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const startsWith = aoc_lib.util.startsWith;

pub fn results() [4]Result {
    return .{
        Result{ .Int32 = 161 },
        Result{ .Int32 = 183669043 },
        Result{ .Int32 = 48 },
        Result{ .Int32 = 59097164 },
    };
}

pub fn solvePart01(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;
    _ = allocator;

    const memory = try input.string();
    const sum = computeSum(memory, false);
    return Result{ .Int32 = sum };
}

pub fn solvePart02(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;
    _ = allocator;

    const memory = try input.string();
    const sum = computeSum(memory, true);
    return Result{ .Int32 = sum };
}

fn computeSum(memory: []u8, with_conditionals: bool) i32 {
    var sum: i32 = 0;
    var offset: usize = 0;
    var enabled: bool = true;

    while (offset < memory.len) {
        const mul = find_mul(memory[offset..]);
        if (mul != null) {
            if (enabled) sum += mul.?.arg1 * mul.?.arg2;
            offset += mul.?.byte_offset - 1; // -1 to negate +1 at loop end
        } else if (with_conditionals) {
            // If conditionals are enabled, also check for do and donts if mul didnt match
            if (startsWith(u8, memory[offset..], "do()")) {
                enabled = true;
            } else if (startsWith(u8, memory[offset..], "don't()")) {
                enabled = false;
            }
        }

        offset += 1;
    }

    return sum;
}

fn find_mul(slice: []u8) ?Mul {
    var state = MulState.MulFunc;
    var i: usize = 0;
    var arg1: i32 = 0;
    var arg2: i32 = 0;

    while (true) {
        // std.debug.print("{s}\n", .{@tagName(state)});
        switch (state) {
            .MulFunc => if (!matchConstAndProceed(slice, "mul", MulState.Open, &state, &i)) return null,
            .Open => if (!matchConstAndProceed(slice, "(", MulState.Arg1, &state, &i)) return null,
            .Arg1 => arg1 = matchNumAndProceed(slice, MulState.Comma, &state, &i) orelse return null,
            .Comma => if (!matchConstAndProceed(slice, ",", MulState.Arg2, &state, &i)) return null,
            .Arg2 => arg2 = matchNumAndProceed(slice, MulState.Close, &state, &i) orelse return null,
            .Close => if (!matchConstAndProceed(slice, ")", MulState.MulFunc, &state, &i)) return null else break,
        }
    }

    return Mul{ .arg1 = arg1, .arg2 = arg2, .byte_offset = i };
}

fn matchConstAndProceed(slice: []u8, comptime match: []const u8, next_state: MulState, state_ptr: *MulState, offset_ptr: *usize) bool {
    const len = match.len;
    if (!startsWith(u8, slice[offset_ptr.*..], match)) {
        return false;
    }
    state_ptr.* = next_state;
    offset_ptr.* += len;
    return true;
}

fn matchNumAndProceed(slice: []u8, next_state: MulState, state_ptr: *MulState, offset_ptr: *usize) ?i32 {
    var num: i32 = 0;
    while (slice.len != 0) {
        if (!std.ascii.isDigit(slice[offset_ptr.*])) break;
        const digit: i32 = slice[offset_ptr.*] - '0';
        num = (num * 10) + digit;
        offset_ptr.* += 1;
    }
    if (num == 0 and slice[offset_ptr.*] != '0') {
        return null;
    }
    state_ptr.* = next_state;
    return num;
}

const MulState = enum { MulFunc, Open, Arg1, Comma, Arg2, Close };

const Mul = struct {
    arg1: i32,
    arg2: i32,
    byte_offset: usize,
};
