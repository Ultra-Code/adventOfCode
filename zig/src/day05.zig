const std = @import("std");

const StacksOfCrates = struct {
    const Self = @This();

    const stack_offset = 4;
    const column_no = 9;
    const column_len = 8 + 40; //the +40 is to account for the lenght during crates shuffle/movement
    const moves_no = 501;

    const Stack = std.BoundedArray(u8, column_len);

    /// struct { quantity: u9, from: u9, to: u9 };
    const Moves = struct { u9, u9, u9 };

    crates: [column_no]Stack,
    moves: [moves_no]Moves,

    pub fn init() Self {
        const input = @embedFile("data/day05.txt");
        var crates: [column_no]Stack = [_]Stack{Stack.init(0) catch unreachable} ** column_no;

        var lines = std.mem.splitScalar(u8, input, '\n');
        while (lines.next()) |line| {
            //end of the stack
            if (line[0] != '[') {
                _ = lines.next(); //skip the column numbers under the stack
                break;
            }
            var stack_index: usize = 0;
            while (stack_index < column_no) : (stack_index += 1) {
                const position = (stack_offset * stack_index) + 1; // +1 due to 0 counting
                const stack_value = line[position];

                if (stack_value != ' ')
                    crates[stack_index].appendAssumeCapacity(stack_value);
            }
        }

        var moves: [moves_no]Moves = undefined;

        var instruction_index: usize = 0;
        while (lines.next()) |instructions| : (instruction_index += 1) {
            if (instructions.len == 0) break;

            var instruction = std.mem.tokenizeAny(u8, instructions, "move from to");
            moves[instruction_index] = Moves{
                parse(instruction.next().?),
                parse(instruction.next().?),
                parse(instruction.next().?),
            };
        }

        // reverse the stack so that its end match the input sample end
        for (&crates) |*stack| {
            std.mem.reverse(u8, stack.slice());
        }

        return .{ .crates = crates, .moves = moves };
    }

    fn parse(buf: []const u8) u9 {
        return std.fmt.parseUnsigned(u9, buf, 10) catch unreachable;
    }

    fn print(crates: anytype) void {
        for (crates, 0..) |row, index| {
            std.debug.print("{}", .{index});
            for (row.constSlice()) |value| {
                std.debug.print("[{c}]", .{value});
            }
            std.debug.print("\n", .{});
        }
    }
};

pub fn part1() void {
    var stacks = StacksOfCrates.init();

    for (stacks.moves) |instruction| {
        const quantity, const from, const to = instruction;

        var move_count: usize = 0;
        while (move_count < quantity) : (move_count += 1) {
            const move_value = stacks.crates[from - 1].pop();
            stacks.crates[to - 1].appendAssumeCapacity(move_value);
        }
    }

    //get top of stacks
    for (stacks.crates) |crate| {
        if (crate.len > 0) std.debug.print("{c}", .{crate.get(crate.len - 1)});
    }
}
