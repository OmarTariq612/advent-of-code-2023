pub fn process(content: impl AsRef<str>) -> u64 {
    let patterns = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];

    let values = [0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    content
        .as_ref()
        .lines()
        .map(|line| {
            let mut result = 0u64;

            let first_digit = line.char_indices().find(|(_, ch)| ch.is_digit(10));

            let first_number = patterns
                .iter()
                .enumerate()
                .map(|(i, pattern)| (i, line.find(pattern)))
                .filter(|(_, op)| op.is_some())
                .map(|(i, op)| (i, op.unwrap()))
                .min_by(|(_, x), (_, y)| x.cmp(y));

            let value = match (first_digit, first_number) {
                (_, None) => u64::from(first_digit.unwrap().1.to_digit(10).unwrap()),
                (None, _) => values[first_number.unwrap().0],
                (Some(first_digit), Some(first_number)) => {
                    if first_digit.0 < first_number.1 {
                        u64::from(first_digit.1.to_digit(10).unwrap())
                    } else {
                        values[first_number.0]
                    }
                }
            };

            result += 10 * value;

            let last_digit = line.char_indices().rfind(|(_, ch)| ch.is_digit(10));

            let last_number = patterns
                .iter()
                .enumerate()
                .map(|(i, pattern)| (i, line.rfind(pattern)))
                .filter(|(_, op)| op.is_some())
                .map(|(i, op)| (i, op.unwrap()))
                .max_by(|(_, x), (_, y)| x.cmp(y));

            let value = match (last_digit, last_number) {
                (_, None) => u64::from(last_digit.unwrap().1.to_digit(10).unwrap()),
                (None, _) => values[last_number.unwrap().0],
                (Some(last_digit), Some(last_number)) => {
                    if last_digit.0 > last_number.1 {
                        u64::from(last_digit.1.to_digit(10).unwrap())
                    } else {
                        values[last_number.0]
                    }
                }
            };

            result += value;

            result
        })
        .sum()
}
