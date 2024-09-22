use std::collections;

fn startpacket<const WINDOW_SIZE: usize>(datastream: &str) -> usize {
    let start_index = datastream
        .as_bytes()
        .array_windows::<WINDOW_SIZE>()
        .enumerate()
        .find_map(|(index, chunk)| {
            let mut set = collections::HashSet::new();
            let all_distinct = chunk.iter().all(|&char| set.insert(char));
            if all_distinct {
                Some(index)
            } else {
                None
            }
        })
        .unwrap();

    start_index + WINDOW_SIZE
}

pub fn part1() -> usize {
    const WINDOW_SIZE: usize = 4;
    let input = include_str!("data/day06.txt");
    startpacket::<WINDOW_SIZE>(input)
}

pub fn part2() -> usize {
    const WINDOW_SIZE: usize = 14;
    let input = include_str!("data/day06.txt");
    startpacket::<WINDOW_SIZE>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startpacket() {
        const WINDOW_SIZE: usize = 4;
        let data = [
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        for (index, &packet) in data.iter().enumerate() {
            match index {
                0 => assert_eq!(startpacket::<WINDOW_SIZE>(packet), 7),
                1 => assert_eq!(startpacket::<WINDOW_SIZE>(packet), 5),
                2 => assert_eq!(startpacket::<WINDOW_SIZE>(packet), 6),
                3 => assert_eq!(startpacket::<WINDOW_SIZE>(packet), 10),
                4 => assert_eq!(startpacket::<WINDOW_SIZE>(packet), 11),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1987);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 3059);
    }
}
