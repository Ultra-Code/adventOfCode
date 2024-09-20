#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![feature(num_midpoint)]
#![feature(iter_next_chunk)]
#![feature(iter_array_chunks)]
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day02::part1()?;
    day02::part2()?;

    // day05::part1();
    // day05::part1_take2();
    // day05::part2();

    Ok(())
}
