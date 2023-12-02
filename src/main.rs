use advent_2023::day2;
use std::fs;

fn main() {
    let content =
        fs::read_to_string("/home/omar/programming/rust/advent-2023/inputs/day2.txt").unwrap();
    println!("result = {}", day2::part2::process(content));
}
