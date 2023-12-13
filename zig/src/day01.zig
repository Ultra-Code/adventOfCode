const std = @import("std");
const utils = @import("utils.zig");

const gpa = utils.gpa;
const Calories = utils.Calories;
const EMPTY = utils.EMPTY;

fn elveList() std.ArrayList(Calories) {
    const cwd = std.fs.cwd();
    const content = cwd.readFileAlloc(gpa, "../data/day01.txt", std.math.maxInt(usize)) catch unreachable;

    var lines = std.mem.splitScalar(u8, content, '\n');

    var elves_calories = std.ArrayList(Calories).init(gpa);

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
    return elves_calories;
}

pub fn part1() void {
    const elves_calories = elveList();
    const location_of_elve_with_max_calories = std.mem.indexOfMax(Calories, elves_calories.items);
    std.debug.print(
        "The {[position]}th elve has the maximum number of calories which is {[max]}\n",
        .{ .position = location_of_elve_with_max_calories, .max = elves_calories.items[location_of_elve_with_max_calories] },
    );
}

pub fn part2() void {
    const elves_calories = elveList().items;
    std.mem.sortUnstable(usize, elves_calories, {}, std.sort.desc(usize));
    const total_calories_of_top_3_elves = sum: {
        var total: usize = 0;
        for (elves_calories[0..3]) |calories| {
            total += calories;
        }
        break :sum total;
    };
    std.debug.print(
        "The total calories of the top 3 elves is {[total]}\n",
        .{ .total = total_calories_of_top_3_elves },
    );
}
