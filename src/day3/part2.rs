use std::collections::{HashMap, HashSet};

pub fn process(content: impl AsRef<str>) -> u64 {
    let mut matrix: Vec<Vec<char>> = content
        .as_ref()
        .lines()
        .map(|line| format!(".{}.", line).chars().collect())
        .collect();

    let (rows, cols) = (matrix.len() + 2, matrix[0].len());

    matrix.insert(0, ".".repeat(cols).chars().collect());
    matrix.push(".".repeat(cols).chars().collect());

    let mut visited_numbers = HashSet::new();
    let mut from_geer_to_numbers = HashMap::new();

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if matrix[i][j] == '*' {
                for x in -1isize..=1 {
                    for y in -1isize..=1 {
                        let (n, m) = ((i as isize + x) as usize, (j as isize + y) as usize);
                        if matrix[n][m].is_digit(10) {
                            let mut start = m;
                            while matrix[n][start].is_digit(10) {
                                start -= 1;
                            }
                            start += 1;

                            let mut end = start;
                            while matrix[n][end].is_digit(10) {
                                end += 1;
                            }

                            let number =
                                &matrix[n][start..end].into_iter().fold(0u64, |acc, ch| {
                                    (acc + ch.to_digit(10).unwrap() as u64) * 10
                                }) / 10;

                            if !visited_numbers.contains(&(n, start, end)) {
                                visited_numbers.insert((n, start, end));
                                from_geer_to_numbers
                                    .entry((i, j))
                                    .or_insert(Vec::new())
                                    .push(number);
                            }
                        }
                    }
                }
            }
        }
    }

    from_geer_to_numbers
        .iter()
        .filter(|&(_, value)| value.len() == 2)
        .map(|(_, value)| value.iter().product::<u64>())
        .sum()
}
