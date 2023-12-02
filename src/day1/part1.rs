pub fn process(content: impl AsRef<str>) -> u64 {
    content.as_ref().lines().map(|line| {
        let mut result = 0u64;
        for ch in line.chars() {
            if ch.is_numeric() {
                result += 10 * u64::from(ch.to_digit(10).unwrap());
                break;
            }
        }
        for ch in line.chars().rev() {
            if ch.is_numeric() {
                result += u64::from(ch.to_digit(10).unwrap());
                break;
            }
        }
        result
    }).sum()
}