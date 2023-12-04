use super::part1::to_numbers;

pub fn process(content: impl AsRef<str>) -> u64 {
    let line_count = content.as_ref().lines().count();
    let mut v = vec![1u64; line_count];

    content.as_ref().lines().enumerate().for_each(|(i, line)| {
        let (_, line) = line.split_once(": ").unwrap();
        let (winning_cards, numbers_i_have) = line.split_once(" | ").unwrap();
        let (winning_cards, numbers_i_have) =
            (to_numbers(winning_cards), to_numbers(numbers_i_have));
        let count = numbers_i_have
            .iter()
            .filter(|number| winning_cards.contains(number))
            .count();

        for index in (i + 1)..(i + 1 + count) {
            v[index] += v[i];
        }
    });

    v.iter().sum()
}
