use std::fs::read_to_string;

pub fn day01() -> Result<(), Box<dyn std::error::Error>> {
    let content = read_to_string("../data/day01.txt")?;

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

    let max = list_of_elves_calories
        .iter()
        .reduce(|x, y| std::cmp::max(x, y))
        .unwrap();

    println!("The maximum number of calories with an elve is {max}");
    Ok(())
}
