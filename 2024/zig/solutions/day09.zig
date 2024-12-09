const std = @import("std");
const aoc_lib = @import("aoc_lib");
const Allocator = std.mem.Allocator;
const ProblemInput = aoc_lib.types.ProblemInput;
const Result = aoc_lib.types.Result;

pub fn results() [4]Result {
    return .{
        Result{ .USize = 1928 },
        Result{ .USize = 6262891638328 },
        Result{ .USize = 2858 },
        Result{ .USize = 6287317016845 },
    };
}

pub fn solvePart01(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    const disk_map = try input.string();
    const blocks = try createBlocks(disk_map, allocator);

    // For part one we split all blocks into 1-size blocks of same name to allow piece-wise moves
    var files = try std.ArrayList(Block).initCapacity(allocator, disk_map.len);
    var free_spans = try std.ArrayList(Block).initCapacity(allocator, disk_map.len);

    for (blocks.files) |file| for (0..file.length) |offset|
        try files.append(Block{ .id = file.id.?, .position = file.position + offset, .length = 1 });

    for (blocks.free_spans) |free| for (0..free.length) |offset|
        try free_spans.append(Block{ .id = null, .position = free.position + offset, .length = 1 });

    // File compacting process
    cleanupDisk(files.items, free_spans.items, false);

    const checksum = computeChecksum(files.items);
    return Result{ .USize = checksum };
}

pub fn solvePart02(allocator: Allocator, input: *ProblemInput, is_sample: bool) !Result {
    _ = is_sample;

    const disk_map = try input.string();
    const blocks = try createBlocks(disk_map, allocator);

    cleanupDisk(blocks.files, blocks.free_spans, true);

    const checksum = computeChecksum(blocks.files);
    return Result{ .USize = checksum };
}

/// File or free span block (id only not-null for files)
const Block = struct { id: ?usize, position: usize, length: usize };

fn createBlocks(disk_map: []u8, allocator: Allocator) !struct { files: []Block, free_spans: []Block } {
    var files = std.ArrayList(Block).init(allocator);
    var free_spans = std.ArrayList(Block).init(allocator);

    var index: usize = 0;
    for (disk_map, 0..) |d, i| {
        const len = d - '0';
        const is_file = (i & 1 == 0);
        if (is_file) {
            std.debug.assert(len > 0); // 0-length files would break the algorithm
            try files.append(Block{ .id = i / 2, .position = index, .length = len });
        } else {
            if (len > 0) try free_spans.append(Block{ .id = null, .position = index, .length = len });
        }
        index += len;
    }

    return .{ .files = try files.toOwnedSlice(), .free_spans = try free_spans.toOwnedSlice() };
}

/// Moves files on disk. If always_search_all is true, we reset the "free_span" counter to 0
/// for each file, which is needed in part 2, as smaller files can fit in spans that larger, earlier files couldnt.
fn cleanupDisk(files: []Block, free_spans: []Block, always_search_all: bool) void {
    // Reverse iterate through all files and move to left if possible
    var freespan_idx: usize = 0;
    var file_idx: usize = files.len;
    while (file_idx > 0) {
        file_idx -= 1;
        const file = &files[file_idx];

        // Check all free spans to the left
        for (free_spans[freespan_idx..], freespan_idx..) |*free, next_fs_idx| {
            if (free.position >= file.position) break; // Only move to left

            // Fill free space if big enough
            if (free.length >= file.length) {
                // Could add the now free space back to the list, but it'll be always right of any file we move next, so not relevant
                file.position = free.position;
                free.position += file.length;
                free.length -= file.length;

                freespan_idx = if (always_search_all) 0 else next_fs_idx + 1;
                break;
            }
        }
    }
}

fn computeChecksum(files: []Block) usize {
    var checksum: usize = 0;
    for (files) |file| {
        for (0..file.length) |offset| {
            checksum += (file.position + offset) * file.id.?;
        }
    }
    return checksum;
}
