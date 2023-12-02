#[derive(Debug, Clone, Copy)]
pub(super) struct Bag {
    pub(super) red: u64,
    pub(super) green: u64,
    pub(super) blue: u64,
}

impl Bag {
    pub(super) fn new(red: u64, green: u64, blue: u64) -> Self {
        Self { red, green, blue }
    }

    pub(super) fn get_greater(self, other: Self) -> Self {
        Self {
            red: std::cmp::max(self.red, other.red),
            green: std::cmp::max(self.green, other.green),
            blue: std::cmp::max(self.blue, other.blue),
        }
    }
}

impl Default for Bag {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

use std::ops::{Add, AddAssign};

impl Add for Bag {
    type Output = Bag;
    fn add(self, rhs: Self) -> Self::Output {
        Bag::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl AddAssign for Bag {
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        (self.red == other.red) && (self.green == other.green) && (self.blue == other.blue)
    }
}

impl PartialOrd for Bag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            _ if self.red < other.red && self.green < other.green && self.blue < other.blue => {
                Some(std::cmp::Ordering::Less)
            }
            _ if self.red > other.red && self.green > other.green && self.blue > other.blue => {
                Some(std::cmp::Ordering::Greater)
            }
            _ if self.eq(other) => Some(std::cmp::Ordering::Equal),
            _ => None,
        }
    }

    fn le(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn ge(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }
}

use std::str::FromStr;

impl FromStr for Bag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 3 blue, 4 red
        Ok(s.split(", ")
            .map(|item| {
                let (number, kind) = item.split_once(' ').unwrap();
                let number: u64 = number.parse().unwrap();

                match kind {
                    "red" => Bag::new(number, 0, 0),
                    "green" => Bag::new(0, number, 0),
                    "blue" => Bag::new(0, 0, number),
                    _ => unreachable!(),
                }
            })
            .fold(Bag::default(), |acc, bag| acc + bag))
    }
}

pub fn process(content: impl AsRef<str>) -> u64 {
    let current_bag = Bag::new(12, 13, 14);

    content
        .as_ref()
        .lines()
        .map(|line| {
            let (game, rest) = line.split_once(": ").unwrap();
            let game_id: u64 = game.strip_prefix("Game ").unwrap().parse().unwrap();

            (
                rest.split("; ")
                    .map(|item| item.parse::<Bag>().unwrap())
                    .all(|bag| bag <= current_bag),
                game_id,
            )
        })
        .filter(|&(boolean, _)| boolean)
        .map(|(_, game_id)| game_id)
        .fold(0, |acc, id| acc + id)
}
