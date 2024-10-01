const std = @import("std");
const testing = std.testing;
const mem = std.mem;
const fmt = std.fmt;
const debug = std.debug;
const alloc = @import("alloc.zig");

const Fs = struct {
    root: Directory,
    arena: mem.Allocator,

    const File = struct {
        name: []const u8,
        size: usize,
    };

    const Directory = struct {
        name: []const u8,
        files: std.ArrayListUnmanaged(File),
        sub_dirs: std.ArrayListUnmanaged(Directory),
        parent: ?*Directory,
        size: usize,

        fn init(name: []const u8, parent: ?*Directory) Directory {
            return .{
                .name = name,
                .files = .{},
                .sub_dirs = .{},
                .parent = parent,
                .size = 0,
            };
        }

        fn deinit(dir: *Directory, arena: mem.Allocator) void {
            dir.files.deinit(arena);
            dir.sub_dirs.deinit(arena);
            if (dir.parent) |parent| {
                parent.deinit(arena);
            }
        }
    };

    fn init(arena: mem.Allocator) Fs {
        return .{
            .root = .init("/", null),
            .arena = arena,
        };
    }

    fn deinit(fs: *Fs, dir: *Directory) void {
        dir.deinit(fs.arena);
    }

    fn browseFs(fs: *Fs, actions: []const u8) *Directory {
        const arena = fs.arena;
        const root = &fs.root;
        var current_dir = root;

        var command_output = mem.tokenizeAny(u8, actions, "$");
        while (command_output.next()) |commands| {
            var command = mem.tokenizeAny(u8, commands, " \n");
            while (command.next()) |next_command| {
                if (mem.eql(u8, next_command, "cd")) {
                    const cd_dir_name = command.next().?;
                    if (mem.eql(u8, cd_dir_name, "/")) {
                        continue;
                    } else if (mem.eql(u8, cd_dir_name, "..")) {
                        current_dir = current_dir.parent.?;
                    } else {
                        for (current_dir.sub_dirs.items) |*dir| {
                            if (mem.eql(u8, dir.name, cd_dir_name)) {
                                current_dir = dir;
                                break;
                            }
                        } else {
                            unreachable;
                        }
                    }
                } else {
                    debug.assert(mem.eql(u8, next_command, "ls"));

                    while (command.next()) |dir_or_filesize| {
                        if (mem.eql(u8, dir_or_filesize, "dir")) {
                            const sub_dir_name = command.next().?;
                            const sub_dir: Directory = .init(
                                sub_dir_name,
                                current_dir,
                            );
                            current_dir.sub_dirs.append(
                                arena,
                                sub_dir,
                            ) catch unreachable;
                        } else {
                            const file_name = command.next().?;
                            const file_size = fmt.parseUnsigned(
                                usize,
                                dir_or_filesize,
                                10,
                            ) catch unreachable;

                            current_dir.size += file_size;
                            if (current_dir.parent) |parent| {
                                parent.size += current_dir.size;
                            }

                            current_dir.files.append(arena, File{
                                .name = file_name,
                                .size = file_size,
                            }) catch unreachable;
                        }
                    }
                }
            }
        }
        return root;
    }

    const fs_operations =
        \\$ cd /
        \\$ ls
        \\dir a
        \\14848514 b.txt
        \\8504156 c.dat
        \\dir d
        \\$ cd a
        \\$ ls
        \\dir e
        \\29116 f
        \\2557 g
        \\62596 h.lst
        \\$ cd e
        \\$ ls
        \\584 i
        \\$ cd ..
        \\$ cd ..
        \\$ cd d
        \\$ ls
        \\4060174 j
        \\8033020 d.log
        \\5626152 d.ext
        \\7214296 k
    ;

    test browseFs {
        var arena = std.heap.ArenaAllocator.init(testing.allocator);
        defer arena.deinit();

        var device_fs = Fs.init(arena.allocator());
        const dir = device_fs.browseFs(fs_operations);
        defer device_fs.deinit(dir);

        const files = dir.files.items;
        const sub_dirs = dir.sub_dirs.items;
        for (0..2) |max_depth| {
            switch (max_depth) {
                0 => {
                    const file_b = Fs.File{
                        .name = "b.txt",
                        .size = 14848514,
                    };
                    try testing.expectEqualDeep(file_b, files[max_depth]);
                    const sub_dir = sub_dirs[max_depth];
                    try testing.expectEqualSlices(u8, sub_dir.name, "a");
                    try testing.expectEqual(94853, sub_dir.size);
                    try testing.expectEqual(sub_dir.parent.?.name, "/");
                    const i_file_index = 0;
                    const file_i_actual = sub_dirs[max_depth].sub_dirs.items[max_depth].files.items[i_file_index];
                    const file_i = Fs.File{
                        .name = "i",
                        .size = 584,
                    };

                    try testing.expectEqualDeep(file_i, file_i_actual);
                },
                1 => {
                    const file_c = Fs.File{
                        .name = "c.dat",
                        .size = 8504156,
                    };
                    try testing.expectEqualDeep(file_c, files[max_depth]);
                    const sub_dir = sub_dirs[max_depth];
                    try testing.expectEqualSlices(u8, sub_dir.name, "d");
                    try testing.expectEqual(24933642, sub_dir.size);

                    try testing.expectEqual(sub_dir.parent.?.name, "/");
                    const k_file_index = 3;
                    const file_k_actual = sub_dirs[max_depth].files.items[k_file_index];
                    const file_k = Fs.File{
                        .name = "k",
                        .size = 7214296,
                    };

                    try testing.expectEqualDeep(file_k, file_k_actual);
                },
                else => unreachable,
            }
        }
    }

    // preorder traversal of the directory tree
    fn sumSizeOfDirLessThan(fs: *Fs, max_size: usize) usize {
        var running_sum: usize = if (fs.root.size <= max_size) fs.root.size else 0;
        var queue: std.ArrayList([]Directory) = .init(fs.arena);
        queue.append(fs.root.sub_dirs.items) catch unreachable;
        var loop_count: usize = 0;
        while (queue.popOrNull()) |dirs| : (loop_count += 1) {
            std.debug.print("loop count {}\n", .{loop_count});
            for (dirs) |dir| {
                if (dir.size <= max_size) running_sum += dir.size;
                if (dir.sub_dirs.items.len != 0) {
                    queue.append(dir.sub_dirs.items) catch unreachable;
                }
            }
        }
        return running_sum;
    }

    test sumSizeOfDirLessThan {
        var arena = std.heap.ArenaAllocator.init(testing.allocator);
        defer arena.deinit();

        var device_fs = Fs.init(arena.allocator());
        const dir = device_fs.browseFs(fs_operations);
        defer device_fs.deinit(dir);

        try testing.expectEqual(95437, device_fs.sumSizeOfDirLessThan(100_000));
    }
};

fn part1() usize {
    const data = @embedFile("data/day07.txt");

    var device_fs = Fs.init(alloc.arena);
    const dir = device_fs.browseFs(data);
    defer device_fs.deinit(dir);

    return device_fs.sumSizeOfDirLessThan(100_000);
}

test part1 {
    try testing.expectEqual(1, part1());
}

test {
    _ = Fs;
}
