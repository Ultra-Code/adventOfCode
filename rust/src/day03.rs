use std::fs::read_to_string;

pub fn part1() -> Result<u32, Box<dyn std::error::Error>> {
    let content = read_to_string("src/data/day03.txt")?;
    let lines = content.lines();

    let common_item_in_various_rucksack_compartment =
        lines.into_iter().map(|line| {
            let midpoint = line.len().midpoint(0);
            let (first_compartment, second_compartment) =
                line.split_at(midpoint);
            let matching_item = first_compartment
                .chars()
                .filter(|&item| second_compartment.contains(item))
                .take(1)
                .next();

            matching_item.unwrap()
        });

    let priority_sum =
        items_to_priority(common_item_in_various_rucksack_compartment);
    println!("The priority sum of items in the rucksuck is {priority_sum}");

    Ok(priority_sum)
}

fn items_to_priority<T>(common_item_in_various_rucksack_compartment: T) -> u32
where
    T: std::iter::Iterator<Item = char>,
{
    let item_priority =
        common_item_in_various_rucksack_compartment.map(|item| {
            let lowercase_map = '`' as u32;
            let uppercase_map = '@' as u32;
            let lowercase_end = 26;
            if item.is_lowercase() {
                //convert lowercase char in item to a range of 1=a..z=26
                item as u32 - lowercase_map //'`' in ascii is 96
            } else {
                //convert uppercase char in item to a range of 27=A..Z=52
                (item as u32 - uppercase_map) + lowercase_end //'@' in ascii is 64
            }
        });

    item_priority.sum::<u32>()
}

#[test]
fn test_part1() {
    assert_eq!(8139, part1().unwrap());
}

pub fn part2() -> u32 {
    let content = read_to_string("src/data/day03.txt").unwrap();
    let lines = content.lines();

    // while let Ok(elve_group) = lines.next_chunk::<3>() {
    //     let [first, second, thrid] = elve_group;
    //     println!("{first}, {second},{thrid}");
    //     println!("\n");
    // }

    let group_badges =
        lines.array_chunks::<3>().map(|[first, second, thrid]| {
            let mut common_badge_item =
                first.chars().filter(|&items| second.contains(items));
            let next_common_badge_item = second
                .chars()
                .filter(|&items| thrid.contains(items))
                .collect::<Vec<char>>();

            let badge = common_badge_item
                .find(|badge| next_common_badge_item.contains(badge));
            badge.unwrap()
        });

    let priority_sum = items_to_priority(group_badges);

    println!("The sum of the priorities of the badge items is {priority_sum}");

    priority_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(8139, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(2668, part2());
    }
}
