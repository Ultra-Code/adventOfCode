const std = @import("std");
const builtin = @import("builtin");

pub fn main() !void {
    var GPA = std.heap.GeneralPurposeAllocator(.{}){};

    const gpa = switch (builtin.mode) {
        .Debug => blk: {
            break :blk GPA.allocator();
        },
        else => blk: {
            var arena = std.heap.ArenaAllocator.init(GPA.allocator());
            break :blk arena.allocator();
        },
    };

    const cwd = std.fs.cwd();
    const content = try cwd.readFileAlloc(gpa, "../data/day01.txt", std.math.maxInt(usize));
    var lines = std.mem.splitScalar(u8, content, '\n');
    while (lines.next()) |line| {
        if (line.len > 0) {
            const elve_calaries = try std.fmt.parseInt(usize, line, 10);
        } else {}
    }
    std.debug.print("content\n{s}", .{content});
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
