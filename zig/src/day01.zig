const std = @import("std");
const utils = @import("utils.zig");

const gpa = utils.gpa;
const Calories = utils.Calories;
const EMPTY = utils.EMPTY;

pub fn day01() void {
    const cwd = std.fs.cwd();
    const content = cwd.readFileAlloc(gpa, "../data/day01.txt", std.math.maxInt(usize)) catch unreachable;

    var lines = std.mem.splitScalar(u8, content, '\n');

    var elves_calories = std.ArrayList(Calories).init(gpa);
    defer elves_calories.deinit();

    var current_total_calories: usize = 0;
    while (lines.next()) |line| {
        if (line.len != EMPTY) {
            const elve_calories = std.fmt.parseInt(usize, line, 10) catch unreachable;
            current_total_calories += elve_calories;
        } else {
            //add the number of calories for the current elve
            elves_calories.append(current_total_calories) catch unreachable;
            //reset number of calories for next elve
            current_total_calories = 0;
        }
    }
    const location_of_elve_with_max_calories = std.mem.indexOfMax(Calories, elves_calories.items);
    std.debug.print(
        "The {[position]}th elve has the maximum number of calories which is {[max]}\n",
        .{ .position = location_of_elve_with_max_calories, .max = elves_calories.items[location_of_elve_with_max_calories] },
    );
}
