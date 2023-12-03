use advent_2023::day3;
use std::fs;

fn main() {
    let content =
        fs::read_to_string("/home/omar/programming/rust/advent-2023/inputs/day3.txt").unwrap();
    println!("result = {}", day3::part1::process(content));
}
