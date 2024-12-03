const std = @import("std");

pub fn startswith(comptime T: type, string: []const T, prefix: []const T) bool {
    return string.len > prefix.len and std.mem.eql(T, string[0..prefix.len], prefix);
}
