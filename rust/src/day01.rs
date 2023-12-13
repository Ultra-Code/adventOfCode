use std::fs::read_to_string;
fn elve_list() -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let content = read_to_string("src/data/day01.txt")?;

    let mut list_of_elves_calories: Vec<usize> = vec![];

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
pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let list_of_elves_calories = elve_list()?;
    let max = list_of_elves_calories
        .iter()
        .reduce(|x, y| std::cmp::max(x, y))
        .unwrap();

    println!("The maximum number of calories with an elve is {max}");
    Ok(())
}

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let mut list_of_elves_calories = elve_list()?;

    list_of_elves_calories.sort_unstable_by(|a, b| b.cmp(a));
    let total_calories_of_top_3_elves = list_of_elves_calories.iter().take(3).sum::<usize>();

    println!("The top 3 elves have total calories of {total_calories_of_top_3_elves}");
    Ok(())
}
