const std = @import("std");
const parseUnsigned = std.fmt.parseUnsigned;
const tokenizeScalar = std.mem.tokenizeScalar;
const splitScalar = std.mem.splitScalar;
const debug = std.log.debug;
const expectEqual = std.testing.expectEqual;

pub fn part1() usize {
    const content = @embedFile("data/day04.txt");

    var elf_pair = tokenizeScalar(u8, content, '\n');

    var number_of_fully_contained_pairs: usize = 0;

    while (elf_pair.next()) |elves| {
        var elves_section_range = tokenizeScalar(u8, elves, ',');
        const first_range = elves_section_range.next().?;
        const second_range = elves_section_range.next().?;

        var first_range_numbers = splitScalar(u8, first_range, '-');
        const first_range_start = parseUnsigned(usize, first_range_numbers.next().?, 10) catch unreachable;
        const first_range_end = parseUnsigned(usize, first_range_numbers.next().?, 10) catch unreachable;

        var second_range_numbers = splitScalar(u8, second_range, '-');
        const second_range_start = parseUnsigned(usize, second_range_numbers.next().?, 10) catch unreachable;
        const second_range_end = parseUnsigned(usize, second_range_numbers.next().?, 10) catch unreachable;

        //check if range overlap fully
        if (first_range_start <= second_range_start and first_range_end >= second_range_end) {
            number_of_fully_contained_pairs += 1;
        } else if (second_range_start <= first_range_start and second_range_end >= first_range_end) {
            number_of_fully_contained_pairs += 1;
        }
    }

    debug("{} assignment pairs are fully contain by other section assignment ranges\n", .{number_of_fully_contained_pairs});

    return number_of_fully_contained_pairs;
}

test part1 {
    try expectEqual(@as(usize, 573), part1());
}

pub fn part2() usize {
    const content = @embedFile("data/day04.txt");

    var elf_pair = tokenizeScalar(u8, content, '\n');

    var number_of_overlapping_pairs: usize = 0;

    while (elf_pair.next()) |elves| {
        var elves_section_range = tokenizeScalar(u8, elves, ',');
        const first_range = elves_section_range.next().?;
        const second_range = elves_section_range.next().?;

        var first_range_numbers = splitScalar(u8, first_range, '-');
        const first_range_start = parseUnsigned(usize, first_range_numbers.next().?, 10) catch unreachable;
        const first_range_end = parseUnsigned(usize, first_range_numbers.next().?, 10) catch unreachable;

        var second_range_numbers = splitScalar(u8, second_range, '-');
        const second_range_start = parseUnsigned(usize, second_range_numbers.next().?, 10) catch unreachable;
        const second_range_end = parseUnsigned(usize, second_range_numbers.next().?, 10) catch unreachable;

        //check if range overlap at all
        if (first_range_start <= second_range_start and first_range_end >= second_range_start) {
            number_of_overlapping_pairs += 1;
        } else if (second_range_start <= first_range_start and second_range_end >= first_range_start) {
            number_of_overlapping_pairs += 1;
        }
    }

    debug("{} assignment pairs overlap with their consecutive section assignments\n", .{number_of_overlapping_pairs});

    return number_of_overlapping_pairs;
}

test part2 {
    try expectEqual(@as(usize, 867), part2());
}
