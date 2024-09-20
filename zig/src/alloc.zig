const std = @import("std");
const builtin = @import("builtin");

var GPA = std.heap.GeneralPurposeAllocator(.{}){};

pub const gpa = switch (builtin.mode) {
    .Debug => blk: {
        break :blk GPA.allocator();
    },
    else => blk: {
        var arena = std.heap.ArenaAllocator.init(GPA.allocator());
        break :blk arena.allocator();
    },
};
