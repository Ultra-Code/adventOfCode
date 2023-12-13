use std::collections::HashMap;
use std::fs::read_to_string;

enum Game {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum GameOutcome {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let content = read_to_string("src/data/day02.txt")?;
    let initial_game_map = HashMap::<&str, Game>::from([
        ("A", Game::Rock),
        ("B", Game::Paper),
        ("C", Game::Scissors),
        ("X", Game::Rock),
        ("Y", Game::Paper),
        ("Z", Game::Scissors),
    ]);

    let wins = content.split('\n').map_while(|lines| {
        let mut line = if lines.is_empty() {
            return None;
        } else {
            lines.split(' ')
        };
        let opponent_str = line.next().unwrap();
        let me_str = line.next().unwrap();
        let opponent = initial_game_map.get(opponent_str).unwrap();
        let me = initial_game_map.get(me_str).unwrap();
        match opponent {
            Game::Rock => match me {
                Game::Rock => Some((Game::Rock, GameOutcome::Draw)),
                Game::Paper => Some((Game::Paper, GameOutcome::Win)),
                Game::Scissors => Some((Game::Scissors, GameOutcome::Lost)),
            },
            Game::Paper => match me {
                Game::Rock => Some((Game::Rock, GameOutcome::Lost)),
                Game::Paper => Some((Game::Paper, GameOutcome::Draw)),
                Game::Scissors => Some((Game::Scissors, GameOutcome::Win)),
            },
            Game::Scissors => match me {
                Game::Rock => Some((Game::Rock, GameOutcome::Win)),
                Game::Paper => Some((Game::Paper, GameOutcome::Lost)),
                Game::Scissors => Some((Game::Scissors, GameOutcome::Draw)),
            },
        }
    });
    let mut total_score = 0;
    for (game_value, outcome_value) in wins {
        total_score += game_value as usize + outcome_value as usize;
    }

    println!("According to the initial strategy the total score is {total_score}");

    Ok(())
}

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let content = read_to_string("src/data/day02.txt")?;

    #[rustfmt::skip]
    let opponent_game_map = HashMap::<&str, Game>::from([
        ("A", Game::Rock),
        ("B", Game::Paper),
        ("C", Game::Scissors),
    ]);

    let my_game_map = HashMap::<&str, GameOutcome>::from([
        ("X", GameOutcome::Lost),
        ("Y", GameOutcome::Draw),
        ("Z", GameOutcome::Win),
    ]);

    let wins = content.split('\n').map_while(|lines| {
        let mut line = if lines.is_empty() {
            return None;
        } else {
            lines.split(' ')
        };
        let opponent_str = line.next().unwrap();
        let me_str = line.next().unwrap();
        let opponent = opponent_game_map.get(opponent_str).unwrap();
        let me = my_game_map.get(me_str).unwrap();
        match opponent {
            Game::Rock => match me {
                GameOutcome::Lost => Some((Game::Scissors, GameOutcome::Lost)),
                GameOutcome::Draw => Some((Game::Rock, GameOutcome::Draw)),
                GameOutcome::Win => Some((Game::Paper, GameOutcome::Win)),
            },
            Game::Paper => match me {
                GameOutcome::Lost => Some((Game::Rock, GameOutcome::Lost)),
                GameOutcome::Draw => Some((Game::Paper, GameOutcome::Draw)),
                GameOutcome::Win => Some((Game::Scissors, GameOutcome::Win)),
            },
            Game::Scissors => match me {
                GameOutcome::Lost => Some((Game::Paper, GameOutcome::Lost)),
                GameOutcome::Draw => Some((Game::Scissors, GameOutcome::Draw)),
                GameOutcome::Win => Some((Game::Rock, GameOutcome::Win)),
            },
        }
    });
    let mut total_score = 0;
    for (game_value, outcome_value) in wins {
        total_score += game_value as usize + outcome_value as usize;
    }

    println!("According to the new strategy guide the total score would be {total_score}");

    Ok(())
}
