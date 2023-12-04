use advent_2023::day4;
use std::fs;

fn main() {
    let content =
        fs::read_to_string("/home/omar/programming/rust/advent-2023/inputs/day4.txt").unwrap();
    println!("result = {}", day4::part2::process(content));
}
