const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;

pub fn results() [4]Result {
    return .{
        Result{ .String = "4,6,3,5,6,3,5,2,1,0" },
        Result{ .String = "7,4,2,0,5,0,5,3,7" },
        Result.NoSample,
        Result{ .UInt64 = 202991746427434 },
    };
}

pub fn solvePart01(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;

    var computer = try Computer.init(input.lines, allocator);
    try computer.run();
    return Result{ .String = try computer.finalOutput(allocator) };
}

pub fn solvePart02(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    if (is_sample) return Result.NoSample;

    const computer = try Computer.init(input.lines, allocator);
    std.mem.reverse(u3, computer.program); // last output determines MSB, so start there
    return Result{ .UInt64 = findAAnalytical(computer.program, 0, 0).? };

    // var computer = try Computer.init(input.lines, allocator);
    // return Result{ .UInt64 = (try findASearch(&computer, 0, 0)).? };
}

const Computer = struct {
    program: []u3,
    registers: [3]u64,
    pc: usize = 0,
    output: std.ArrayList(u3),
    _allocator: Allocator,

    const Self = @This();

    pub fn init(input: [][]u8, allocator: Allocator) !Self {
        var registers = [3]u64{ 0, 0, 0 };

        // Parse input
        for (0..3) |reg| {
            var it = std.mem.splitBackwardsSequence(u8, input[reg], ": ");
            const value = try std.fmt.parseInt(u64, it.next().?, 10);
            registers[reg] = value;
        }

        var program_it = std.mem.tokenizeScalar(u8, input[4][9..], ',');
        var program = std.ArrayList(u3).init(allocator);
        while (program_it.next()) |item| try program.append(try std.fmt.parseInt(u3, item, 10));

        return .{
            .program = try program.toOwnedSlice(),
            .registers = registers,
            .output = std.ArrayList(u3).init(allocator),
            ._allocator = allocator,
        };
    }

    pub fn reset(self: *Self) void {
        self.pc = 0;
        self.output.clearRetainingCapacity();
        self.registers = [3]u64{ 0, 0, 0 };
    }

    pub fn run(self: *Self) !void {
        while (self.pc < self.program.len) {
            const opcode = self.program[self.pc];
            const operand = self.program[self.pc + 1];
            const literal_operand: u64 = operand;
            const combo_operand: u64 = if (4 <= operand and operand <= 6) self.registers[operand - 4] else literal_operand;

            var jumped = false;
            switch (opcode) {
                0 => self.registers[0] = self.registers[0] >> @truncate(combo_operand),
                1 => self.registers[1] ^= literal_operand,
                2 => self.registers[1] = combo_operand % 8,
                3 => if (self.registers[0] != 0) {
                    self.pc = literal_operand;
                    jumped = true;
                },
                4 => self.registers[1] ^= self.registers[2],
                5 => try self.output.append(@truncate(combo_operand % 8)),
                6 => self.registers[1] = self.registers[0] >> @truncate(combo_operand),
                7 => self.registers[2] = self.registers[0] >> @truncate(combo_operand),
            }

            if (!jumped) self.pc += 2;
        }
    }

    pub fn finalOutput(self: Self, allocator: Allocator) ![]const u8 {
        const buffer = try allocator.alloc(u8, self.output.items.len * 2);
        for (self.output.items, 0..) |val, i| {
            buffer[2 * i] = '0' + @as(u8, val);
            buffer[2 * i + 1] = ',';
        }
        return buffer[0 .. buffer.len - 1];
    }
};

/// Finds A value by analyzing the programs code (how output is computed).
fn findAAnalytical(output: []u3, index: usize, current_a: u48) ?u48 {
    // Program: 2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0
    // (2,4) B = A % 8
    // (1,1) B = B ^ 1
    // (7,5) C = A >> B
    // (4,4) B = B ^ C
    // (1,4) B = B ^ 4
    // (0,3) A = A >> 3
    // (5,5) OUT (B % 8)
    // (3,0) IF A != 0 JUMP 0

    // Each iteration A is rightshifted by 3, so since we have 16 outputs, we need 16*3 = 48 bits in A
    // OUT = ((A % 8) ^ (A >> ((A % 8) ^ 1)) ^ 5) % 8

    // We check in groups of 3 (bits ijk). Due to the bitshift we need to consider the 7 bits before that as well.
    // So we test all possible ijk, they determine what bits form the output (relevant), and if it works, we recurse.
    // We might have a group later, where no ijk works, so we need backtracking.

    // A = abcd efgh ijk,    ijk == A % 8
    //
    //          offset  relevant    xor      OUT (uppercase means flipped)
    //    ijk | ijk^1 | A>>offset | ijk^5 |  relevant^xor
    // 0  000 | 001   | hij       | 101   |  HiJ
    // 1  001 | 000   | ijk       | 100   |  Ijk
    // 2  010 | 011   | fgh       | 111   |  FGH
    // 3  011 | 010   | ghi       | 110   |  GHi
    // 4  100 | 101   | def       | 001   |  deF
    // 5  101 | 100   | efg       | 000   |  efg
    // 6  110 | 111   | bcd       | 011   |  bCD
    // 7  111 | 110   | cde       | 010   |  cDe

    if (index >= output.len) return current_a;

    const out = output[index];

    for (0..8) |ijk| {
        const ijk_bits: u3 = @truncate(ijk);
        const offset: u3 = ijk_bits ^ 1;
        const xor: u3 = ijk_bits ^ 5;
        const updated_a = (current_a << 3) | ijk_bits;
        if ((((updated_a >> offset) & 0b111) ^ xor) == out) {
            if (findAAnalytical(output, index + 1, updated_a)) |a|
                return a;
        }
    }
    return null;
}

/// Finds A by trial and error. Note that the bits in A have independent effects on the output.
/// So if the suffix of the output is already correct, changing the higher bits of A wont change that.
/// While this does not need exact details of the program, we still assume the general structure of it.
/// Meaning that A is solely responsible for the output, decreases by 3 bits every iteration etc.
fn findASearch(computer: *Computer, a: u64, index: usize) !?u64 {
    if (index == computer.program.len) {
        return a;
    }

    for (0..8) |i| {
        computer.reset();
        computer.registers[0] = (a << 3) + i;
        try computer.run();

        if (std.mem.eql(u3, computer.output.items, computer.program[computer.program.len - index - 1 ..])) {
            if (try findASearch(computer, (a << 3) + i, index + 1)) |solved_a|
                return solved_a;
        }
    }

    return null;
}
