use std::{array::from_fn, fs};

type History = [i64; 21];

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut input = parse_input(&input);

    println!("{}", part_one(&input));
    println!("{}", part_two(&mut input));
}

fn part_one(input: &[History]) -> i64 {
    input.iter().map(|a| a[20] + extrapolate(a)).sum()
}

fn part_two(input: &mut [History]) -> i64 {
    input.iter_mut().for_each(|row| row.reverse());
    input.iter().map(|a| a[20] + extrapolate(a)).sum()
}

fn extrapolate(row: &[i64]) -> i64 {
    if row.iter().all(|a| a == &0) {
        return 0;
    }

    let row = row
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();

    row.last().unwrap() + extrapolate(&row)
}

fn parse_input(input: &str) -> Vec<History> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            from_fn(|_| iter.next().unwrap().parse().unwrap())
        })
        .collect()
}
