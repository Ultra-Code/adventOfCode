#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
mod day01;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day01::part1()?;
    day01::part2()?;
    Ok(())
}
