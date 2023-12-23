#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![feature(num_midpoint)]
#![feature(iter_next_chunk)]
#![feature(iter_array_chunks)]
mod day04;

fn main() {
    day04::part1();
    day04::part2();
}
