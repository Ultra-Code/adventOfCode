const std = @import("std");
const day03 = @import("day03.zig");

pub fn main() !void {
    day03.part1();
    day03.part2();
}

test {
    _ = std.testing.refAllDeclsRecursive(@This());
}
