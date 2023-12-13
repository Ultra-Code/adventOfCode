#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
mod day02;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day02::part1()?;
    day02::part2()?;
    Ok(())
}
