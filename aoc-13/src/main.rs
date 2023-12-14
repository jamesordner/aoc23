use std::{fs, iter::zip};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let patterns = parse_input(&input);

    let part_one = part_one(&patterns);
    println!("{part_one}");

    let part_two = part_two(&patterns);
    println!("{part_two}");
}

fn part_one(patterns: &[Pattern]) -> u32 {
    patterns
        .iter()
        .map(|pattern| {
            cols_left_of_reflection(pattern)
                .next()
                .unwrap_or_else(|| rows_above_reflection(pattern).next().unwrap() * 100)
        })
        .sum()
}

fn part_two(patterns: &[Pattern]) -> u32 {
    patterns
        .iter()
        .map(|pattern| {
            alt_cols_left_of_reflection(pattern)
                .unwrap_or_else(|| alt_rows_above_reflection(pattern).unwrap() * 100)
        })
        .sum()
}

fn cols_left_of_reflection(pattern: &Pattern) -> impl Iterator<Item = u32> + '_ {
    (1..pattern.column_len)
        .filter(|&i| {
            let mut col1 = i;
            let mut col0 = col1 - 1;

            while zip(pattern.col(col0), pattern.col(col1)).all(|(a, b)| a == b) {
                if col0 == 0 || col1 == pattern.column_len - 1 {
                    return true;
                }

                col0 -= 1;
                col1 += 1;
            }

            false
        })
        .map(|i| i as u32)
}

fn alt_cols_left_of_reflection(pattern: &Pattern) -> Option<u32> {
    let original_val = cols_left_of_reflection(pattern).next();

    (0..pattern.data.len()).find_map(|i| {
        let mut pattern = pattern.clone();
        pattern.data[i] = !pattern.data[i];

        let mut iter = cols_left_of_reflection(&pattern);

        iter.find(|&val| Some(val) != original_val)
    })
}

fn rows_above_reflection(pattern: &Pattern) -> impl Iterator<Item = u32> + '_ {
    let row_len = pattern.data.len() / pattern.column_len;

    (1..row_len)
        .filter(move |&i| {
            let mut row1 = i;
            let mut row0 = row1 - 1;

            while pattern.row(row0) == pattern.row(row1) {
                if row0 == 0 || row1 == row_len - 1 {
                    return true;
                }

                row0 -= 1;
                row1 += 1;
            }

            false
        })
        .map(|i| i as u32)
}

fn alt_rows_above_reflection(pattern: &Pattern) -> Option<u32> {
    let original_val = rows_above_reflection(pattern).next();

    (0..pattern.data.len()).find_map(|i| {
        let mut pattern = pattern.clone();
        pattern.data[i] = !pattern.data[i];

        let mut iter = rows_above_reflection(&pattern);

        iter.find(|&val| Some(val) != original_val)
    })
}

#[derive(Clone)]
struct Pattern {
    data: Vec<bool>,
    column_len: usize,
}

impl Pattern {
    fn row(&self, index: usize) -> &[bool] {
        &self.data[index * self.column_len..(index + 1) * self.column_len]
    }

    fn col(&self, index: usize) -> impl Iterator<Item = &bool> {
        let row_len = self.data.len() / self.column_len;
        (0..row_len).map(move |i| &self.data[i * self.column_len + index])
    }
}

fn parse_input(input: &str) -> Vec<Pattern> {
    let mut lines = input.lines();

    let mut patterns = Vec::new();

    while let Some(line) = lines.next() {
        let mut data: Vec<_> = line.as_bytes().iter().map(|&b| b == b'#').collect();
        let column_len = data.len();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            data.extend(line.as_bytes().iter().map(|&b| b == b'#'));
        }

        patterns.push(Pattern { data, column_len })
    }

    patterns
}
