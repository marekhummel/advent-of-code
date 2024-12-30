const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;

pub fn results() [4]Result {
    return .{
        Result{ .UInt64 = 37327623 },
        Result{ .UInt64 = 14119253575 },
        Result{ .UInt64 = 23 },
        Result{ .UInt64 = 1600 },
    };
}

const NUM_PRICE_CHANGES = 19 * 19 * 19 * 19;

pub fn solvePart01(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = allocator;
    _ = is_sample;

    var sum: u64 = 0;
    for (input.lines) |line| {
        var secret = try std.fmt.parseInt(u64, line, 10);
        for (0..2000) |_| secret = evolveSecret(secret);
        sum += secret;
    }

    return Result{ .UInt64 = sum };
}

pub fn solvePart02(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;

    // Build lookup tables (maps each combination of price change sequences to the banana sell value)
    var secret_sequences = std.ArrayList([NUM_PRICE_CHANGES]u8).init(allocator);
    for (input.lines) |line| {
        const secret = try std.fmt.parseInt(u64, line, 10);
        const sequence_value = sequenceValues(secret);
        try secret_sequences.append(sequence_value);
    }

    // Find best sequence to max bananas. Note that price change sequence is encoded as an index
    // Changes are mapped from -9..9 to 0..18 and then computed to base 19 number.
    var best_bananas: u64 = 0;
    for (0..NUM_PRICE_CHANGES) |encoded| {
        var total_bananas: u64 = 0;
        for (secret_sequences.items) |sequence_value| {
            total_bananas += sequence_value[encoded];
        }
        best_bananas = @max(best_bananas, total_bananas);
    }

    return Result{ .UInt64 = best_bananas };
}

fn sequenceValues(start_secret: u64) [NUM_PRICE_CHANGES]u8 {
    var sequence_values = [_]u8{0} ** NUM_PRICE_CHANGES;
    var encoded_deltas: [4]usize = undefined;

    var secret = start_secret;
    var last_price: u8 = @intCast(start_secret % 10);

    for (0..2000) |i| {
        // Compute new secret and the shifted price change
        secret = evolveSecret(secret);
        const price: u8 = @intCast(secret % 10);
        const encoded_delta: usize = price + 9 - last_price;
        last_price = price;

        // Store price change and set banana sell price for this sequence
        std.mem.rotate(usize, &encoded_deltas, 1);
        encoded_deltas[3] = encoded_delta;
        if (i > 3) {
            const encoded_seq = ((encoded_deltas[0] * 19 + encoded_deltas[1]) * 19 + encoded_deltas[2]) * 19 + encoded_deltas[3];
            if (sequence_values[encoded_seq] == 0) sequence_values[encoded_seq] = price;
        }
    }

    return sequence_values;
}

fn evolveSecret(secret: u64) u64 {
    var new_secret = secret;
    new_secret = ((new_secret << 6) ^ new_secret) & 0xffffff;
    new_secret = ((new_secret >> 5) ^ new_secret) & 0xffffff;
    new_secret = ((new_secret << 11) ^ new_secret) & 0xffffff;
    return new_secret;
}
