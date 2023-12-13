const std = @import("std");
const day02 = @import("day02.zig");

pub fn main() !void {
    day02.part1();
    day02.part2();
}

test {
    _ = std.testing.refAllDeclsRecursive(@This());
}
