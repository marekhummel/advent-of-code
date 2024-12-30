const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;

pub fn results() [4]Result {
    return .{
        Result{ .USize = 55312 },
        Result{ .USize = 233875 },
        Result.NoSample,
        Result{ .USize = 277444936413293 },
    };
}

pub fn solvePart01(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;
    const num_stones = try blinkAll(try input.string(), 25, allocator);
    return Result{ .USize = num_stones };
}

pub fn solvePart02(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    if (is_sample) return Result.NoSample;

    const num_stones = try blinkAll(try input.string(), 75, allocator);
    return Result{ .USize = num_stones };
}

const CacheKey = struct { stone: u64, blinks: usize };

fn blinkAll(stone_str: []u8, blinks: usize, allocator: Allocator) !usize {
    // Parse input to slice
    var stones = std.ArrayList(u64).init(allocator);
    var stone_it = std.mem.tokenize(u8, stone_str, " ");
    while (stone_it.next()) |stone| try stones.append(try std.fmt.parseInt(u64, stone, 10));

    // Handle each stone seperately as they dont interact with each other, use cache
    var sum: usize = 0;
    var cache = std.AutoHashMap(CacheKey, usize).init(allocator);
    for (stones.items) |stone| {
        sum += try blinkStone(stone, blinks, &cache);
    }

    return sum;
}

fn blinkStone(stone: u64, blinks: usize, cache: *std.AutoHashMap(CacheKey, usize)) !usize {
    // Cached
    const key = .{ .stone = stone, .blinks = blinks };
    if (cache.get(key)) |num_new_stones| return num_new_stones;

    // -- Compute result
    var result: usize = undefined;

    if (blinks == 0) {
        // Base case, no blinking
        return 1;
    } else if (stone == 0) {
        // 0 stone
        result = try blinkStone(1, blinks - 1, cache);
    } else {
        const num_digits = std.math.log10(stone) + 1;
        if (num_digits & 1 == 0) {
            // Even length num
            const split = std.math.pow(u64, 10, num_digits / 2);
            const new_stones_left = try blinkStone(stone / split, blinks - 1, cache);
            const new_stones_right = try blinkStone(stone % split, blinks - 1, cache);
            result = new_stones_left + new_stones_right;
        } else {
            // Odd length
            result = try blinkStone(stone * 2024, blinks - 1, cache);
        }
    }

    // -- Update cache
    try cache.put(key, result);
    return result;
}
