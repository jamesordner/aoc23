use std::{array::from_fn, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let result_one = part_one(&input);
    let result_two = part_two(&input);

    println!("{result_one}");
    println!("{result_two}");
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(game_info)
        .map(|(winning_numbers, numbers)| {
            let winning_count = numbers.filter(|n| winning_numbers.contains(n)).count();
            (1 << winning_count) >> 1
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let mut num_cards: [u32; 215] = from_fn(|_| 1);

    for (i, (winning_numbers, numbers)) in input.lines().map(game_info).enumerate() {
        let card_copies = num_cards[i];
        let winning_count = numbers.filter(|n| winning_numbers.contains(n)).count();

        for num_cards in &mut num_cards[i + 1..=i + winning_count] {
            *num_cards += card_copies;
        }
    }

    num_cards.into_iter().sum()
}

/// Returns `(winning numbers, game numbers)`
fn game_info(line: &str) -> (Vec<u32>, impl Iterator<Item = u32> + '_) {
    let mut iter = line[9..].split('|');

    let winning_numbers = iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|str| str.parse::<u32>().unwrap())
        .collect();

    let game_numbers = iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|str| str.parse::<u32>().unwrap());

    (winning_numbers, game_numbers)
}
