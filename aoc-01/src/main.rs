use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let result_one = part_one(&input);
    let result_two = part_two(&input);

    println!("{result_one}");
    println!("{result_two}");
}

fn part_one(input: &str) -> u32 {
    sum_lines(
        input,
        |line| {
            line.chars()
                .find(|a| a.is_numeric())
                .unwrap()
                .to_digit(10)
                .unwrap()
        },
        |line| {
            line.chars()
                .rev()
                .find(|a| a.is_numeric())
                .unwrap()
                .to_digit(10)
                .unwrap()
        },
    )
}

fn part_two(input: &str) -> u32 {
    sum_lines(
        input,
        |mut line| loop {
            if let Some(digit) = parse_str(|i| line.get(..i)) {
                break digit;
            }

            line = &line[1..];
        },
        |mut line| loop {
            if let Some(digit) = parse_str(|i| line.len().checked_sub(i).map(|i| &line[i..])) {
                break digit;
            }

            line = &line[..line.len() - 1];
        },
    )
}

fn sum_lines<F, Q>(input: &str, tens: F, ones: Q) -> u32
where
    F: Fn(&str) -> u32,
    Q: Fn(&str) -> u32,
{
    input.lines().map(|line| tens(line) * 10 + ones(line)).sum()
}

fn parse_str<'a, F>(get_slice: F) -> Option<u32>
where
    F: Fn(usize) -> Option<&'a str>,
{
    get_slice(1)
        .and_then(|s| s.parse::<u32>().ok())
        .or_else(|| {
            [
                "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ]
            .iter()
            .enumerate()
            .find(|(_, number_str)| get_slice(number_str.len()) == Some(**number_str))
            .map(|(i, _)| i as u32)
        })
}
