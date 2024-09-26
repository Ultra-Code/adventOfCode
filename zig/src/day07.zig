const std = @import("std");
const testing = std.testing;
const mem = std.mem;
const fmt = std.fmt;

const Fs = struct {
    const File = struct {
        name: []const u8,
        size: usize,
    };
    const Directory = struct {
        name: []const u8,
        files: std.ArrayList(File),
        sub_directories: ?*Directory,
        parent: ?*Directory,
    };
    directory: Directory,

    fn init(arena: mem.Allocator) Fs {
        return .{ .directory = .{
            .name = "",
            .files = .init(arena),
            .sub_directories = null,
            .parent = null,
        } };
    }

    fn browseFs(fs: *Fs, actions: []const u8) *const Fs {
        var command_output = mem.tokenizeAny(u8, actions, " $");
        while (command_output.next()) |command| {
            if (mem.eql(u8, command, "cd")) {
                const path = command_output.next().?;
                fs.directory.name = path;
            } else {
                const ls_output = command_output.next().?;
                var directory = mem.tokenizeSequence(u8, ls_output, "dir");
                var sub_directory = fs.directory.files.allocator.create(Directory) catch unreachable;
                fs.directory.sub_directories = sub_directory;
                sub_directory.parent = &fs.directory;
                while (directory.next()) |dir_output| {
                    var dir_info = mem.tokenizeScalar(u8, dir_output, ' ');

                    const dir_name = dir_info.next().?;
                    sub_directory.name = dir_name;
                    while (dir_info.next()) |size| {
                        const file_size = fmt.parseUnsigned(usize, size, 10) catch unreachable;
                        const file_name = dir_info.next().?;
                        fs.directory.files.append(.{ .name = file_name, .size = file_size }) catch unreachable;
                    }
                } else {
                    var file_info = mem.tokenizeScalar(u8, ls_output, ' ');
                    std.debug.print("ls_output {s}", .{ls_output});

                    while (file_info.next()) |size| {
                        const file_size = fmt.parseUnsigned(usize, size, 10) catch unreachable;
                        const file_name = file_info.next().?;
                        fs.directory.files.append(.{ .name = file_name, .size = file_size }) catch unreachable;
                    }
                }
            }
        }
        return fs;
    }
};

test "sample" {
    const fs_operations =
        \\\$ cd /
        \\\$ ls
        \\\dir a
        \\\14848514 b.txt
        \\\8504156 c.dat
        \\\dir d
        \\\$ cd a
        \\\$ ls
        \\\dir e
        \\\29116 f
        \\\2557 g
        \\\62596 h.lst
        \\\$ cd e
        \\\$ ls
        \\\584 i
        \\\$ cd ..
        \\\$ cd ..
        \\\$ cd d
        \\\$ ls
        \\\4060174 j
        \\\8033020 d.log
        \\\5626152 d.ext
        \\\7214296 k
    ;

    var device = Fs.init(testing.allocator);
    const fs = device.browseFs(fs_operations);
    try testing.expectEqual(&fs.directory, fs.directory.parent);
    // testing.expectEqual(
    //     device.fs.atmost(.dir, 100_000),
    //     &.{ .{ "a", 94_853 }, .{ "e", 584 } },
    // );
}
