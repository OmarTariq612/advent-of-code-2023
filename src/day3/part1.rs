use std::collections::HashSet;

pub fn process(content: impl AsRef<str>) -> u64 {
    let mut matrix: Vec<Vec<char>> = content
        .as_ref()
        .lines()
        .map(|line| format!(".{}.", line).chars().collect())
        .collect();

    let (rows, cols) = (matrix.len() + 2, matrix[0].len());

    matrix.insert(0, ".".repeat(cols).chars().collect());
    matrix.push(".".repeat(cols).chars().collect());

    let mut set = HashSet::new();
    let mut result = 0u64;

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if !matrix[i][j].is_digit(10) && matrix[i][j] != '.' {
                for x in -1isize..=1 {
                    for y in -1isize..=1 {
                        let (i, j) = ((i as isize + x) as usize, (j as isize + y) as usize);
                        if matrix[i][j].is_digit(10) {
                            let mut start = j;
                            while matrix[i][start].is_digit(10) {
                                start -= 1;
                            }
                            start += 1;

                            let mut end = start;
                            while matrix[i][end].is_digit(10) {
                                end += 1;
                            }

                            let number =
                                &matrix[i][start..end].into_iter().fold(0u64, |acc, ch| {
                                    println!("acc = {}, ch = {}", acc, ch);
                                    (acc + ch.to_digit(10).unwrap() as u64) * 10
                                }) / 10;

                            if !set.contains(&(i, start, end)) {
                                result += number;
                                set.insert((i, start, end));
                            }
                        }
                    }
                }
            }
        }
    }

    result
}
