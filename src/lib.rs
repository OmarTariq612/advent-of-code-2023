pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

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

    mod day3 {
        use super::super::*;

        #[test]
        fn part1() {
            let content = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

            assert_eq!(4361, day3::part1::process(content));
        }


        #[test]
        fn part2() {
            let content = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

            assert_eq!(467835, day3::part2::process(content));
        }
    }


    mod day4 {
        use super::super::*;


        #[test]
        fn part1() {
            let content = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

            assert_eq!(13, day4::part1::process(content));
        }

        #[test]
        fn part2() {
            let content = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

            assert_eq!(30, day4::part2::process(content));
        }
    }
}
