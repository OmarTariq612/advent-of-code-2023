use super::part1::Bag;

pub fn process(content: impl AsRef<str>) -> u64 {
    content
        .as_ref()
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once(": ").unwrap();
            let bag = rest
                .split("; ")
                .map(|round| round.parse::<Bag>().unwrap())
                .fold(Bag::default(), |acc, bag| acc.get_greater(bag));

            bag.red * bag.green * bag.blue
        })
        .sum()
}
