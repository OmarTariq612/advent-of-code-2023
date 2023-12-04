pub fn process(content: impl AsRef<str>) -> u64 {
    content
        .as_ref()
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(": ").unwrap();
            let (winning_cards, numbers_i_have) = line.split_once(" | ").unwrap();
            let (winning_cards, numbers_i_have) =
                (to_numbers(winning_cards), to_numbers(numbers_i_have));
            let count = numbers_i_have
                .iter()
                .filter(|number| winning_cards.contains(number))
                .count();
            if count == 0 {
                0
            } else {
                1 << (count - 1)
            }
        })
        .sum()
}

pub fn to_numbers(line: &str) -> Vec<u64> {
    let mut v = Vec::new();

    let mut multiple = 1u64;
    let mut in_number = false;
    let mut number = 0u64;

    for ch in line.chars() {
        if in_number {
            if ch.is_digit(10) {
                number *= multiple;
                number += ch.to_digit(10).unwrap() as u64;
                multiple *= 10;
            } else {
                in_number = false;
                multiple = 1;
                v.push(number);
            }
        } else {
            if ch.is_digit(10) {
                in_number = true;
                number = ch.to_digit(10).unwrap() as u64;
                multiple *= 10;
            } else {
                // don't do anything actually.
            }
        }
    }

    if in_number {
        v.push(number);
    }

    v
}
