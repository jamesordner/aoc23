use std::{cmp::Ordering, fs};

type Cards = [u8; 5];

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut hand = parse_input(&input);

    let result_one = part_one(&mut hand);
    println!("{result_one}");

    let result_two = part_two(&mut hand);
    println!("{result_two}");
}

fn part_one(hand: &mut [Hand]) -> u32 {
    hand.sort_unstable_by(Hand::cmp_part_one);

    hand.iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) as u32 * hand.bid)
        .sum()
}

fn part_two(hand: &mut [Hand]) -> u32 {
    hand.sort_unstable_by(Hand::cmp_part_two);

    hand.iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) as u32 * hand.bid)
        .sum()
}

#[derive(Debug)]
struct Hand {
    cards: Cards,
    bid: u32,
}

impl Hand {
    fn cmp_part_one(&self, other: &Self) -> Ordering {
        let a_seq_lens = sequence_lengths_part_one(self.cards);
        let b_seq_lens = sequence_lengths_part_one(other.cards);

        cmp_type(a_seq_lens, b_seq_lens).unwrap_or_else(|| self.cards.cmp(&other.cards))
    }

    fn cmp_part_two(&self, other: &Self) -> Ordering {
        let a_seq_lens = sequence_lengths_part_two(self.cards);
        let b_seq_lens = sequence_lengths_part_two(other.cards);

        cmp_type(a_seq_lens, b_seq_lens).unwrap_or_else(|| {
            let a = self.cards.map(|card| if card == 11 { 1 } else { card });
            let b = other.cards.map(|card| if card == 11 { 1 } else { card });
            a.cmp(&b)
        })
    }
}

fn sequence_lengths_part_one(mut cards: Cards) -> [usize; 5] {
    cards.sort_unstable();
    sequence_lengths_slice(&cards)
}

fn sequence_lengths_part_two(cards: Cards) -> [usize; 5] {
    let mut cards = cards.map(|card| if card == 11 { 15 } else { card });

    cards.sort_unstable();

    let (cards, jokers) = cards.split_at(cards.partition_point(|&a| a < 15));

    let mut lengths = sequence_lengths_slice(cards);
    lengths[0] += jokers.len();
    lengths
}

fn sequence_lengths_slice(cards: &[u8]) -> [usize; 5] {
    let mut length_index = 0;
    let mut lengths = [0; 5];

    let mut sequence_start = 0;

    for (i, card) in cards.iter().copied().enumerate() {
        if cards[sequence_start] != card {
            lengths[length_index] = i - sequence_start;
            length_index += 1;
            sequence_start = i;
        }
    }

    lengths[length_index] = cards.len() - sequence_start;

    lengths.sort_unstable();
    lengths.reverse();
    lengths
}

fn cmp_type(a_seq_lens: [usize; 5], b_seq_lens: [usize; 5]) -> Option<Ordering> {
    if a_seq_lens[0] != b_seq_lens[0] {
        Some(a_seq_lens[0].cmp(&b_seq_lens[0]))
    } else if a_seq_lens[0] == 3 && a_seq_lens[1] == 2 && b_seq_lens[1] != 2 {
        Some(Ordering::Greater)
    } else if a_seq_lens[0] == 3 && a_seq_lens[1] != 2 && b_seq_lens[1] == 2 {
        Some(Ordering::Less)
    } else if a_seq_lens[0] == 2 && a_seq_lens[1] == 2 && b_seq_lens[1] != 2 {
        Some(Ordering::Greater)
    } else if a_seq_lens[0] == 2 && a_seq_lens[1] != 2 && b_seq_lens[1] == 2 {
        Some(Ordering::Less)
    } else {
        None
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();

            let cards = Cards::try_from(iter.next().unwrap().as_bytes())
                .unwrap()
                .map(|card| match card {
                    b'2'..=b'9' => card - b'0',
                    b'T' => 10,
                    b'J' => 11,
                    b'Q' => 12,
                    b'K' => 13,
                    b'A' => 14,
                    _ => unreachable!(),
                });

            Hand {
                cards,
                bid: iter.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}
