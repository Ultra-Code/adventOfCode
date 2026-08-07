const std = @import("std");

const column_no = 2;
const column_upper = 2;

const Array = struct {
    const List = [column_no]std.ArrayList(u8);
    items: List,

    fn init(buf: *[column_no * column_upper]u8) Array {
        var arrays: List = @splat(.empty);
        for (&arrays, 0..) |*array, index| {
            array.* = .initBuffer(buf[index * column_no ..][0..column_upper]);
        }
        return .{ .items = arrays };
    }
};

pub fn main() !void {
    var buf: [column_no * column_upper]u8 = undefined;
    var stacks: Array = .init(&buf);
    try stacks.items[0].appendBounded(1);
    try stacks.items[0].appendBounded(2);

    try stacks.items[1].appendBounded(3);
    try stacks.items[1].appendBounded(4);
    for (stacks.items) |items| {
        std.debug.print("{any}\n", .{items.items});
    }
    // try stacks.items[2].appendBounded(0);
}
