fn parse_input() -> Vec<[usize; 4]> {
    let content = std::fs::read_to_string("src/data/day04.txt").unwrap();
    let pair_of_section_ids = content.lines();

    let range = pair_of_section_ids.map(|elves_pair| {
        let (first_section_range, second_section_range) = elves_pair.split_once(',').unwrap();

        let (first_section_begin, first_section_end) = first_section_range.split_once('-').unwrap();
        let (first_section_begin, first_section_end) = (
            first_section_begin.parse::<usize>().unwrap(),
            first_section_end.parse::<usize>().unwrap(),
        );

        let (second_section_begin, second_section_end) =
            second_section_range.split_once('-').unwrap();
        let (second_section_begin, second_section_end) = (
            second_section_begin.parse::<usize>().unwrap(),
            second_section_end.parse::<usize>().unwrap(),
        );

        [
            first_section_begin,
            first_section_end,
            second_section_begin,
            second_section_end,
        ]
    });

    range.collect()
}

pub fn part1() -> usize {
    let pair_of_section_ids = parse_input();
    let fully_contained_range = pair_of_section_ids.iter().filter(
        |[first_section_begin, first_section_end, second_section_begin, second_section_end]| {
            (first_section_begin <= second_section_begin && first_section_end >= second_section_end)
                || (second_section_begin <= first_section_begin
                    && second_section_end >= first_section_end)
        },
    );

    let number_of_fully_contained_range = fully_contained_range.count();

    println!(
        "In {number_of_fully_contained_range} assignment pairs, one range fully contains the other"
    );

    number_of_fully_contained_range
}

pub fn part2() -> usize {
    let pair_of_section_ids = parse_input();
    let overlap_range = pair_of_section_ids.iter().filter(
        |[first_section_begin, first_section_end, second_section_begin, second_section_end]| {
            (first_section_begin <= second_section_begin
                && first_section_end >= second_section_begin)
                || (second_section_begin <= first_section_begin
                    && second_section_end >= first_section_begin)
        },
    );

    let number_of_overlap = overlap_range.count();

    println!("The ranges overlap in {number_of_overlap} assignment pairs");

    number_of_overlap
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(573, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(867, part2());
    }
}
