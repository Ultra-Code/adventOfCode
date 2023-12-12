#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
mod day01;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day01::day01()?;
    Ok(())
}
