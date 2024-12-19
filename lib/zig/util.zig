const std = @import("std");

pub fn startsWith(comptime T: type, string: []const T, prefix: []const T) bool {
    return string.len >= prefix.len and std.mem.eql(T, string[0..prefix.len], prefix);
}

pub fn contains(comptime T: type, list: []T, value: T) bool {
    for (list) |item| {
        if (item == value) {
            return true;
        }
    }
    return false;
}

pub fn freeNested(comptime T: type, nested: [][]T, allocator: std.mem.Allocator) void {
    for (nested) |inner|
        allocator.free(inner);
    allocator.free(nested);
}
