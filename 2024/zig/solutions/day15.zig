const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;
const Grid = aoc_lib.types.Grid;
const Direction = aoc_lib.cartesian.Direction;
const Index = aoc_lib.cartesian.Index;

pub fn results() [4]Result {
    return .{
        Result{ .USize = 10092 },
        Result{ .USize = 1479679 },
        Result{ .USize = 9021 },
        Result{ .USize = 1509780 },
    };
}

pub fn solvePart01(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;
    var warehouse, var robot, const moves = try parseInput(input, false, allocator);
    return Result{ .USize = rearrange_warehouse(&warehouse, &robot, moves) };
}

pub fn solvePart02(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;
    var warehouse, var robot, const moves = try parseInput(input, true, allocator);
    return Result{ .USize = rearrange_warehouse(&warehouse, &robot, moves) };
}

fn parseInput(input: *ProblemInput, extend: bool, allocator: Allocator) !struct { Grid(u8), Index, []Direction } {
    // Find split in input file
    var blank_line: usize = undefined;
    for (input.lines, 0..) |line, i| {
        if (line.len == 0) {
            blank_line = i;
            break;
        }
    }

    // Create warehouse
    var warehouse = Grid(u8).init(input.lines[0..blank_line]);
    var robot = warehouse.find('@').?;
    warehouse.set(robot, '.');

    // Extend if part 2
    if (extend) {
        var new_cells = try allocator.alloc([]u8, warehouse.size.height);
        for (warehouse.cells, 0..) |row, j| {
            new_cells[j] = try allocator.alloc(u8, row.len * 2);
            for (row, 0..) |cell, i| {
                switch (cell) {
                    '.' => std.mem.copyForwards(u8, new_cells[j][2 * i ..], ".."),
                    '#' => std.mem.copyForwards(u8, new_cells[j][2 * i ..], "##"),
                    'O' => std.mem.copyForwards(u8, new_cells[j][2 * i ..], "[]"),
                    else => unreachable,
                }
            }
        }
        warehouse = Grid(u8).init(new_cells);
        robot.c *= 2;
    }

    // Create move list
    var moves = std.ArrayList(Direction).init(allocator);
    for (input.lines[blank_line + 1 ..]) |move_line| {
        for (move_line) |mv| {
            try moves.append(try Direction.from_char(mv));
        }
    }

    return .{ warehouse, robot, try moves.toOwnedSlice() };
}

fn rearrange_warehouse(warehouse: *Grid(u8), robot: *Index, moves: []Direction) usize {
    for (moves) |mv| {
        const next_idx = robot.scout(mv, warehouse.size).?;
        if (push(warehouse, next_idx, mv))
            robot.* = next_idx;
    }

    var coord_sum: usize = 0;
    var box_it = warehouse.iterator();
    while (box_it.next()) |elem| {
        if (elem.value == 'O' or elem.value == '[') {
            coord_sum += elem.idx.r * 100 + elem.idx.c;
        }
    }
    return coord_sum;
}

fn push(warehouse: *Grid(u8), idx: Index, dir: Direction) bool {
    const cell = warehouse.get(idx);

    // ** Default if we try to push where no box is
    if (!is_box(cell)) return cell == '.';

    const next_idx = idx.scout(dir, warehouse.size).?;

    // ** Part 1 and Part 2 in horizontal direction are simple in-line pushes
    if (cell == 'O' or ((cell == '[' or cell == ']') and dir.isHorizontal())) {
        const next_cell = warehouse.get(next_idx);
        if (next_cell == '#') return false;

        if (next_cell == '.' or (is_box(next_cell) and push(warehouse, next_idx, dir))) {
            warehouse.set(next_idx, cell);
            warehouse.set(idx, '.');
            return true;
        }

        return false;
    }

    // ** Pushing north south is now more eloborate, we gotta check first
    // ** because the call stack is no longer a line, but rather a tree
    // Always push left half
    if (cell == ']')
        return push(warehouse, idx.scout(Direction.West, warehouse.size).?, dir);

    // If we cant push, abort
    if (!canPush(warehouse, idx, dir)) return false;

    // Push left and right side recursively
    const next_idx_r = next_idx.scout(Direction.East, warehouse.size).?;
    _ = push(warehouse, next_idx, dir);
    _ = push(warehouse, next_idx_r, dir);

    // Push current box
    const idx_r = idx.scout(Direction.East, warehouse.size).?;
    warehouse.set(next_idx, '[');
    warehouse.set(next_idx_r, ']');
    warehouse.set(idx, '.');
    warehouse.set(idx_r, '.');
    return true;
}

fn canPush(warehouse: *Grid(u8), idx: Index, dir: Direction) bool {
    const cell = warehouse.get(idx);
    if (cell == ']')
        return canPush(warehouse, idx.scout(Direction.West, warehouse.size).?, dir);

    const next_idx = idx.scout(dir, warehouse.size).?;
    const next_idx_r = next_idx.scout(Direction.East, warehouse.size).?;
    const next_cell_l = warehouse.get(next_idx);
    const next_cell_r = warehouse.get(next_idx_r);

    // False if wall behind box
    if (next_cell_l == '#' or next_cell_r == '#') return false;

    // Recurse if more boxes behind
    const can_push_l = (next_cell_l == '.') or ((next_cell_l == '[' or next_cell_l == ']') and canPush(warehouse, next_idx, dir));
    const can_push_r = (next_cell_r == '.') or ((next_cell_r == '[' or next_cell_r == ']') and canPush(warehouse, next_idx_r, dir));
    return (can_push_l and can_push_r);
}

fn is_box(cell: u8) bool {
    return cell == 'O' or cell == '[' or cell == ']';
}
