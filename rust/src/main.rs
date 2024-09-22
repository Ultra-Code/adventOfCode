#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![feature(num_midpoint)]
#![feature(iter_next_chunk)]
#![feature(iter_array_chunks)]
#![feature(iter_map_windows)]
#![feature(array_windows)]
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // day01::part1()?;
    // day01::part1_improved();
    // day01::part2()?;
    // day01::part2_improved();
    // day02::part1()?;
    // day02::part2()?;
    // day03::part1()?;
    // day03::part2();
    // day04::part1();
    // day04::part2();
    day05::part1();
    day05::part1_take2();
    day05::part2();
    // day06::part1();
    // day06::part2();

    Ok(())
}
