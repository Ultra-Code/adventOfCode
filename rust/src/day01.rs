use std::fs::read_to_string;
fn elve_list() -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let content = read_to_string("src/data/day01.txt")?;

    let mut list_of_elves_calories = vec![];

    let elves_calories = content.split("\n\n");

    for elve_calories in elves_calories {
        let each_calory = elve_calories.split('\n');
        {
            let mut total_calory = 0;
            for calory in each_calory {
                if let Ok(calory) = calory.parse::<usize>() {
                    total_calory += calory;
                } else {
                    break;
                }
            }
            list_of_elves_calories.push(total_calory);
        }
    }

    Ok(list_of_elves_calories)
}

pub fn part1() -> Result<usize, Box<dyn std::error::Error>> {
    let list_of_elves_calories = elve_list()?;
    let max = list_of_elves_calories
        .iter()
        .reduce(|x, y| std::cmp::max(x, y))
        .unwrap();

    println!("The maximum number of calories with an elve is {max}");
    Ok(*max)
}

pub fn part1_improved() -> usize {
    let max_elves_calories = include_str!("./data/day01.txt")
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|number| number.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap();
    println!("The maximum number of calories with an elve is {max_elves_calories}");
    max_elves_calories
}

pub fn part2() -> Result<usize, Box<dyn std::error::Error>> {
    let mut list_of_elves_calories = elve_list()?;

    list_of_elves_calories.sort_unstable_by(|a, b| b.cmp(a));
    let total_calories_of_top_3_elves = list_of_elves_calories.iter().take(3).sum::<usize>();

    println!("The top 3 elves have total calories of {total_calories_of_top_3_elves}");
    Ok(total_calories_of_top_3_elves)
}

pub fn part2_improved() -> usize {
    let mut list_of_elves_calories = include_str!("data/day01.txt")
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|number| number.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();
    list_of_elves_calories.sort_unstable_by(|a, b| b.cmp(a));
    let total_calories_of_top_3_elves = list_of_elves_calories.iter().take(3).sum();

    println!("The top 3 elves have total calories of {total_calories_of_top_3_elves}");
    total_calories_of_top_3_elves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1().ok().unwrap(), part1_improved());
        assert_eq!(part1_improved(), 74_711);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2().ok().unwrap(), part2_improved());
        assert_eq!(part2_improved(), 209_481);
    }
}
