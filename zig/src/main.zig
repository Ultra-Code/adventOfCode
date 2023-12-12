const std = @import("std");
const builtin = @import("builtin");
const utils = @import("utils.zig");

pub fn main() !void {
    @import("day01.zig").day01();
}
