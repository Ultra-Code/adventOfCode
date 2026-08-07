const std = @import("std");

pub fn main() !void {
    const day01_txt = std.fs.cwd().openFile(
        "src/data/day01.txt",
        .{ .mode = .read_only },
    ) catch unreachable;
    defer day01_txt.close();
    var day01_data: std.fs.File.Reader = .init(day01_txt, &.{});

    const size = try day01_data.getSize();
    std.debug.print("File size is {Bi}\n", .{size});

    var aw: std.Io.Writer.Allocating = try .initCapacity(std.heap.smp_allocator, size);
    const bytes = aw.writer.sendFileAll(&day01_data, .limited(size)) catch unreachable;
    const content = aw.written();
    std.debug.print("Has bytes {}\nAnd last slice is {s}\n", .{ bytes, content[content.len - 5 .. content.len - 1] });
}
