const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const Graph = aoc_lib.graph.Graph;
const contains = aoc_lib.util.contains;
const join = aoc_lib.util.join;

pub fn results() [4]Result {
    return .{
        Result{ .UInt64 = 2024 },
        Result{ .UInt64 = 46362252142374 },
        Result.NoSample,
        Result{ .String = "cbd,gmh,jmq,qrh,rqf,z06,z13,z38" },
    };
}

const Op = enum { XOR, AND, OR };
const Instruction = struct { op: Op, arg1: []const u8, arg2: []const u8, target: []const u8 };

pub fn solvePart01(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    _ = is_sample;

    // Parse
    var registers, var instructions, var program = try parse(input, allocator);

    // Sort instructions by dependencies and compute them
    const sorting = try program.topoSorting(allocator);
    for (sorting) |target| {
        if (instructions.get(target)) |inst| {
            const value = switch (inst.op) {
                .XOR => registers.get(inst.arg1).? ^ registers.get(inst.arg2).?,
                .OR => registers.get(inst.arg1).? | registers.get(inst.arg2).?,
                .AND => registers.get(inst.arg1).? & registers.get(inst.arg2).?,
            };
            try registers.put(target, value);
        }
    }

    // Eval z
    var z: u64 = 0;
    for (0..64) |i| {
        const zi = try std.fmt.allocPrint(allocator, "z{d:0>2}", .{i});
        const bit = registers.get(zi) orelse break;
        z |= (@as(u64, bit) << @intCast(i));
    }

    return Result{ .UInt64 = z };
}

pub fn solvePart02(input: *ProblemInput, is_sample: bool, allocator: Allocator) !Result {
    if (is_sample) return Result.NoSample;

    // Binary adder (z = x + y), bitwise from LSB to MSB with carry c
    // t_n = x_n XOR y_n
    // xy_n = x_n AND y_n
    // tc_n = t_n AND c_n
    // c_n = xy_(n-1) OR tc_(n-1)
    // z_n = t_n XOR c_n
    // c_0 = 0

    // Parse
    _, var instructions, _ = try parse(input, allocator);

    // Check all bits of z, skipping 0, 1 and 45, cause they are computed differently
    var suspicious = std.ArrayList([]const u8).init(allocator);
    for (2..45) |n| {
        if (try validateBit(n, &instructions, allocator)) |sus| {
            try suspicious.append(sus);
        }
    }

    // Sort and print
    const lessThan = struct {
        fn stringLessThan(_: void, lhs: []const u8, rhs: []const u8) bool {
            return std.mem.order(u8, lhs, rhs) == .lt;
        }
    }.stringLessThan;
    std.mem.sort([]const u8, suspicious.items, {}, lessThan);
    const wire_string = try join(u8, suspicious.items, ",", allocator);
    return Result{ .String = wire_string };
}

fn parse(
    input: *ProblemInput,
    allocator: Allocator,
) !struct { std.StringHashMap(u1), std.StringHashMap(Instruction), Graph([]const u8) } {
    var registers = std.StringHashMap(u1).init(allocator);
    var instructions = std.StringHashMap(Instruction).init(allocator);
    var program = Graph([]const u8).init(allocator);

    for (input.lines) |line| {
        if (contains(u8, line, ':')) {
            var it = std.mem.tokenizeAny(u8, line, ": ");
            const reg = it.next().?;
            const value = try std.fmt.parseInt(u1, it.next().?, 10);
            try registers.put(reg, value);
        } else if (line.len > 0) {
            var it = std.mem.tokenizeScalar(u8, line, ' ');
            const arg1 = it.next().?;
            const op = std.meta.stringToEnum(Op, it.next().?).?;
            const arg2 = it.next().?;
            _ = it.next(); // ->
            const target = it.next().?;

            try instructions.put(target, .{ .arg1 = arg1, .op = op, .arg2 = arg2, .target = target });
            try program.addEdge(arg1, target, false);
            try program.addEdge(arg2, target, false);
        }
    }

    return .{ registers, instructions, program };
}

fn validateBit(n: u64, instructions: *std.StringHashMap(Instruction), allocator: Allocator) !?[]const u8 {
    // z_n = t_n XOR c_n
    const zn = try std.fmt.allocPrint(allocator, "z{d:0>2}", .{n});
    const z_inst = instructions.get(zn).?;
    if (z_inst.op != Op.XOR) return zn;

    // t_n = x_n XOR y_n
    // c_n = xy_(n-1) OR tc_(n-1)
    const t_inst = instructions.get(z_inst.arg1) orelse return z_inst.arg1;
    const c_inst = instructions.get(z_inst.arg2) orelse return z_inst.arg2;

    if (t_inst.op == Op.XOR) {
        return validateTCInstructions(t_inst, c_inst, n, instructions, allocator);
    } else if (c_inst.op == Op.XOR) {
        return validateTCInstructions(c_inst, t_inst, n, instructions, allocator);
    } else {
        if (t_inst.op == Op.OR) return c_inst.target;
        if (c_inst.op == Op.OR) return t_inst.target;
        return z_inst.target;
    }

    return null;
}

fn validateTCInstructions(t_inst: Instruction, c_inst: Instruction, n: u64, instructions: *std.StringHashMap(Instruction), allocator: Allocator) !?[]const u8 {
    // t_n = x_n XOR y_n
    if (!try is_xy_inst(t_inst, n, allocator)) return t_inst.target;

    // c_n = xy_(n-1) OR tc_(n-1)
    if (c_inst.op != Op.OR) return c_inst.target;

    // xy_(n-1) = x_(n-1) AND y_(n-1)
    // tc_(n-1) = t_(n-1) AND c_(n-1)
    const xy_inst = instructions.get(c_inst.arg1) orelse return c_inst.arg1;
    const tc_inst = instructions.get(c_inst.arg2) orelse return c_inst.arg2;
    if (xy_inst.op != Op.AND) return xy_inst.target;
    if (tc_inst.op != Op.AND) return tc_inst.target;

    if (!try is_xy_inst(xy_inst, n - 1, allocator) and !try is_xy_inst(tc_inst, n - 1, allocator))
        return c_inst.target;

    return null;
}

fn is_xy_inst(inst: Instruction, n: u64, allocator: Allocator) !bool {
    const xn = try std.fmt.allocPrint(allocator, "x{d:0>2}", .{n});
    const yn = try std.fmt.allocPrint(allocator, "y{d:0>2}", .{n});

    const is_xnyn = std.mem.eql(u8, inst.arg1, xn) and std.mem.eql(u8, inst.arg2, yn);
    const is_ynxn = std.mem.eql(u8, inst.arg1, yn) and std.mem.eql(u8, inst.arg2, xn);
    return is_xnyn or is_ynxn;
}
