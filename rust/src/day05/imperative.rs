type Operations<'a> = std::iter::Skip<std::str::Lines<'a>>;

fn parse_input(content: &str) -> ([[Option<char>; 9]; 8], Operations<'_>) {
    let mut stacks_of_crates = [[None; 9]; 8];

    let crates_and_operations = content.lines();
    let mut crates = crates_and_operations.clone().take(8).collect::<Vec<&str>>();
    //reverse the order of the elements so that when they been placed in the stack the original
    //order is obtained
    crates.reverse();

    let initial_label_offset = 1; //regular count is 2 but since we are 0 indexing it is 1
    let next_label_offset = 4;
    for (row_index, &marked_crates) in crates.iter().enumerate() {
        let mut label_offset = initial_label_offset;

        for column_index in 0..9 {
            if column_index == 0 {
                let current_crate = marked_crates.chars().nth(initial_label_offset);
                stacks_of_crates[row_index][0] = match current_crate {
                    Some(' ') => None,
                    _ => current_crate,
                };
                label_offset += next_label_offset;
                continue;
            }
            let current_crate = marked_crates.chars().nth(label_offset);

            stacks_of_crates[row_index][column_index] = match current_crate {
                Some(' ') => None,
                _ => current_crate,
            };
            label_offset += next_label_offset;
        }
    }

    let stacks_of_crates_len = 10;

    (
        stacks_of_crates,
        crates_and_operations.skip(stacks_of_crates_len),
    )
}

pub fn part1() -> String {
    let content = std::fs::read_to_string("src/data/day05.txt").unwrap();
    let (stacks_of_crates, operations) = parse_input(&content);
    let mut stack_1 = vec![];
    let mut stack_2 = vec![];
    let mut stack_3 = vec![];
    let mut stack_4 = vec![];
    let mut stack_5 = vec![];
    let mut stack_6 = vec![];
    let mut stack_7 = vec![];
    let mut stack_8 = vec![];
    let mut stack_9 = vec![];

    for marked_crates in stacks_of_crates {
        for (column_index, &current_crate) in marked_crates.iter().enumerate() {
            if let Some(crate_value) = current_crate {
                match column_index {
                    0 => stack_1.push(crate_value),
                    1 => stack_2.push(crate_value),
                    2 => stack_3.push(crate_value),
                    3 => stack_4.push(crate_value),
                    4 => stack_5.push(crate_value),
                    5 => stack_6.push(crate_value),
                    6 => stack_7.push(crate_value),
                    7 => stack_8.push(crate_value),
                    8 => stack_9.push(crate_value),
                    _ => unreachable!(),
                }
            }
        }
    }

    let stacks = [
        &mut stack_1,
        &mut stack_2,
        &mut stack_3,
        &mut stack_4,
        &mut stack_5,
        &mut stack_6,
        &mut stack_7,
        &mut stack_8,
        &mut stack_9,
    ];

    for operation in operations {
        let mut micro_operations = operation.split(' ');
        let quantity_to_move = micro_operations.nth(1).unwrap().parse::<usize>().unwrap();
        let from_stack = micro_operations.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to_stack = micro_operations.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        for _ in 0..quantity_to_move {
            let move_value = stacks[from_stack].pop().unwrap();
            stacks[to_stack].push(move_value);
        }
    }

    let mut top_stacks = String::new();
    for stack in stacks {
        top_stacks.push(*stack.last().unwrap());
    }
    println!("(take:1) Top stacks are : {top_stacks:?}");

    top_stacks
}

pub fn part1_take2() -> String {
    let content = std::fs::read_to_string("src/data/day05.txt").unwrap();
    let crates_and_operations = content.lines();
    let mut crates = crates_and_operations.clone().take(8).collect::<Vec<&str>>();
    //reverse the order of the elements so that when they been placed in the stack the original
    //order is obtained
    crates.reverse();

    let mut stack_1 = vec![];
    let mut stack_2 = vec![];
    let mut stack_3 = vec![];
    let mut stack_4 = vec![];
    let mut stack_5 = vec![];
    let mut stack_6 = vec![];
    let mut stack_7 = vec![];
    let mut stack_8 = vec![];
    let mut stack_9 = vec![];
    //a map of indexes to stacks
    let mut stacks_of_crates = [
        &mut stack_1,
        &mut stack_2,
        &mut stack_3,
        &mut stack_4,
        &mut stack_5,
        &mut stack_6,
        &mut stack_7,
        &mut stack_8,
        &mut stack_9,
    ];

    //offset of 1st label in the stack of crates diagram
    let initial_label_offset = 1; //regular count is 2 but since we are 0 indexing it is 1
    let next_label_offset = 4; // offset of next label in the stack of crates diagram
    for marked_crates in crates {
        let mut label_offset = initial_label_offset;

        for stack in &mut stacks_of_crates {
            let current_crate = marked_crates.chars().nth(label_offset);
            let current_crate = match current_crate {
                Some(' ') => {
                    label_offset += next_label_offset;
                    continue;
                }
                _ => current_crate,
            };

            stack.push(current_crate);
            label_offset += next_label_offset;
        }
    }

    for operation in content.lines().skip(10) {
        let mut micro_operations = operation.split(' ');
        let quantity_to_move = micro_operations.nth(1).unwrap().parse::<usize>().unwrap();
        let from_stack = micro_operations.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to_stack = micro_operations.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        for _ in 0..quantity_to_move {
            let move_value = stacks_of_crates[from_stack].pop().unwrap();
            stacks_of_crates[to_stack].push(move_value);
        }
    }

    let mut top_stacks = String::new();
    for stack in &stacks_of_crates {
        top_stacks.push(stack.last().unwrap().unwrap());
    }
    println!("(part 1 take: 2) Top stacks are : {top_stacks:?}");

    top_stacks
}

pub fn part2() -> String {
    let content = std::fs::read_to_string("src/data/day05.txt").unwrap();
    let crates_and_operations = content.lines();
    let mut crates = crates_and_operations.clone().take(8).collect::<Vec<&str>>();
    //reverse the order of the elements so that when they been placed in the stack the original
    //order is obtained
    crates.reverse();

    let mut stack_1 = vec![];
    let mut stack_2 = vec![];
    let mut stack_3 = vec![];
    let mut stack_4 = vec![];
    let mut stack_5 = vec![];
    let mut stack_6 = vec![];
    let mut stack_7 = vec![];
    let mut stack_8 = vec![];
    let mut stack_9 = vec![];
    //a map of indexes to stacks
    let mut stacks_of_crates = [
        &mut stack_1,
        &mut stack_2,
        &mut stack_3,
        &mut stack_4,
        &mut stack_5,
        &mut stack_6,
        &mut stack_7,
        &mut stack_8,
        &mut stack_9,
    ];

    //offset of 1st label in the stack of crates diagram
    let initial_label_offset = 1; //regular count is 2 but since we are 0 indexing it is 1
    let next_label_offset = 4; // offset of next label in the stack of crates diagram
    for marked_crates in crates {
        let mut label_offset = initial_label_offset;

        for stack in &mut stacks_of_crates {
            let current_crate = marked_crates.chars().nth(label_offset);
            let current_crate = match current_crate {
                Some(' ') => {
                    label_offset += next_label_offset;
                    continue;
                }
                _ => current_crate,
            };

            stack.push(current_crate);
            label_offset += next_label_offset;
        }
    }

    for operation in content.lines().skip(10) {
        let mut micro_operations = operation.split(' ');
        let quantity_to_move = micro_operations.nth(1).unwrap().parse::<usize>().unwrap();
        let from_stack = micro_operations.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to_stack = micro_operations.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        let mut move_values = vec![];
        for _ in 0..quantity_to_move {
            let value = stacks_of_crates[from_stack].pop().unwrap();
            move_values.push(value);
        }
        move_values.reverse(); //to maintain original order

        for value in move_values {
            stacks_of_crates[to_stack].push(value);
        }
    }

    let mut top_stacks = String::new();
    for stack in &stacks_of_crates {
        top_stacks.push(stack.last().unwrap().unwrap());
    }
    println!("(take: 2) Top stacks are : {top_stacks}");

    top_stacks
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), "PTWLTDSJV");
        assert_eq!(part1_take2(), "PTWLTDSJV");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "WZMFVGGZP");
    }
}
