mod imperative;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
struct Move(usize, usize, usize);
enum CrateMover {
    M9000,
    M9001,
}
struct Stacks<'a, const SHIP_COUNT: usize, const STACK_LEN: usize, const MOVES_LEN: usize> {
    stack_of_crates: [VecDeque<char>; SHIP_COUNT],
    moves: [Move; MOVES_LEN],
    instructions: &'a str,
}

impl<'a, const SHIP_COUNT: usize, const STACK_LEN: usize, const MOVES_LEN: usize>
    Stacks<'a, SHIP_COUNT, STACK_LEN, MOVES_LEN>
{
    fn init(instructions: &'a str) -> Self {
        let stack_of_crates = Self::crate_stacks(instructions);
        let moves = Self::moves(instructions);
        Self {
            stack_of_crates,
            moves,
            instructions,
        }
    }

    pub fn top_crates(&mut self, crate_mover: CrateMover) -> String {
        let crate_stacks = &mut self.stack_of_crates;
        let moves = &self.moves;

        match crate_mover {
            CrateMover::M9000 => {
                for &Move(amount, from, to) in moves {
                    (0..amount).for_each(|_| {
                        let value = crate_stacks[from - 1].pop_front().unwrap();
                        crate_stacks[to - 1].push_front(value);
                    });
                }
            }
            CrateMover::M9001 => {
                for &Move(amount, from, to) in moves {
                    let from_crate = &mut crate_stacks[from - 1];
                    let mut crate_to_move = from_crate.drain(..amount).collect::<VecDeque<_>>();
                    crate_to_move.extend(crate_stacks[to - 1].iter());
                    crate_stacks[to - 1] = crate_to_move;
                }
            }
        }

        let mut top = || {
            let mut top_of_stack_crates = String::new();
            crate_stacks
                .iter_mut()
                .for_each(|stack| top_of_stack_crates.push(stack.pop_front().unwrap()));

            top_of_stack_crates
        };

        top()
    }

    fn moves(instructions: &str) -> [Move; MOVES_LEN] {
        let move_instructions_offset = 2;

        let moves: [Move; MOVES_LEN] = instructions
            .lines()
            .skip(STACK_LEN + move_instructions_offset)
            .map(|line| {
                let &[amount, from, to] = line
                    .split_whitespace()
                    .filter_map(|word| word.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
                    .as_slice()
                else {
                    unreachable!()
                };
                Move(amount, from, to)
            })
            .collect::<Vec<Move>>()
            .try_into()
            .unwrap_or_else(|moves| panic!("Unable to convert {moves:#?} to [Moves;{MOVES_LEN}]"));

        moves
    }

    fn crate_stacks(instructions: &str) -> [std::collections::VecDeque<char>; SHIP_COUNT] {
        // the array lenght is the same as the ship count
        let mut crate_stack = std::array::from_fn(|_| VecDeque::with_capacity(STACK_LEN));

        instructions
            .lines()
            .take(STACK_LEN)
            .flat_map(|line| {
                let distance_between_crates = 4;
                line.chars()
                    .skip(1)
                    .step_by(distance_between_crates)
                    .take(SHIP_COUNT)
                    .enumerate()
                    .filter(|&(_column, char)| char.ne(&' '))
            })
            .for_each(|(column, char)| {
                crate_stack[column].push_back(char);
            });

        crate_stack
    }

    fn cargo_stack_len(&self) -> usize {
        let max_cargo_stack_len = self
            .instructions
            .lines()
            .enumerate()
            .find_map(|(index, line)| {
                let is_empty = line.is_empty();

                if is_empty {
                    Some(index - 1)
                } else {
                    None
                }
            })
            .unwrap();

        max_cargo_stack_len
    }

    fn ship_counts(&self) -> usize {
        let ships_count = self
            .instructions
            .lines()
            .nth(STACK_LEN)
            .unwrap()
            .chars()
            .rev()
            .find(|char| char.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;

        ships_count
    }
}

fn part1() -> String {
    let input = include_str!("data/day05.txt");
    let mut stack = Stacks::<9, 8, 501>::init(input);
    stack.top_crates(CrateMover::M9000)
}

fn part2() -> String {
    let input = include_str!("data/day05.txt");
    let mut stack = Stacks::<9, 8, 501>::init(input);
    stack.top_crates(CrateMover::M9001)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starter() {
        let data: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let mut stack = Stacks::<3, 3, 4>::init(data);
        assert_eq!(stack.cargo_stack_len(), 3);
        assert_eq!(stack.ship_counts(), 3);
        assert_eq!(
            Stacks::<3, 3, 4>::crate_stacks(data),
            [
                VecDeque::from(['N', 'Z']),
                VecDeque::from(['D', 'C', 'M']),
                VecDeque::from(['P']),
            ]
        );
        assert_eq!(
            Stacks::<3, 3, 4>::moves(data),
            [Move(1, 2, 1), Move(3, 1, 3), Move(2, 2, 1), Move(1, 1, 2),]
        );

        assert_eq!(stack.top_crates(CrateMover::M9000), "CMZ");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "PTWLTDSJV");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "WZMFVGGZP");
    }
}
