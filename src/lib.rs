pub mod day1;
pub mod day2;

#[cfg(test)]
mod tests {

    mod day1 {
        use super::super::*;

        #[test]
        fn part1() {
            let content = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

            assert_eq!(142, day1::part1::process(content));
        }

        #[test]
        fn part2() {
            let content = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

            assert_eq!(281, day1::part2::process(content));
        }
    }

    mod day2 {
        use super::super::*;

        #[test]
        fn part1() {
            let content = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

            assert_eq!(8, day2::part1::process(content));
        }

        #[test]
        fn part2() {
            let content = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

            assert_eq!(2286, day2::part2::process(content));
        }
    }
}
