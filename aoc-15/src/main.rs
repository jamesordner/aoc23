use std::{array::from_fn, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let part_one = part_one(&input);
    println!("{part_one}");

    let part_two = part_two(&input);
    println!("{part_two}");
}

fn part_one(input: &str) -> u32 {
    input
        .split(',')
        .map(|seq| seq.trim())
        .map(hash)
        .map(u32::from)
        .sum()
}

fn part_two(input: &str) -> u32 {
    let mut boxes: Boxes = from_fn(|_| Vec::new());

    for step in input.split(',').map(|seq| seq.trim()) {
        if step.contains('-') {
            let label = &step[..step.len() - 1];
            let hash = hash(label);
            boxes[hash as usize].retain(|lens| lens.label != label);
        } else {
            let label = &step[..step.len() - 2];
            let hash = hash(label);
            let focal_length = step[step.len() - 1..].parse::<u32>().unwrap();

            if let Some(slot) = boxes[hash as usize]
                .iter_mut()
                .find(|lens| lens.label == label)
            {
                slot.focal_length = focal_length;
            } else {
                boxes[hash as usize].push(Slot {
                    label,
                    focal_length,
                });
            }
        }
    }

    focusing_power(&boxes)
}

type Boxes<'a> = [Vec<Slot<'a>>; 256];

struct Slot<'a> {
    label: &'a str,
    focal_length: u32,
}

fn hash(sequence: &str) -> u8 {
    sequence
        .as_bytes()
        .iter()
        .fold(0_u8, |acc, &byte| acc.wrapping_add(byte).wrapping_mul(17))
}

fn focusing_power(boxes: &Boxes) -> u32 {
    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, lenses)| {
            let i = 1 + i as u32;

            lenses
                .iter()
                .enumerate()
                .map(move |(j, lens)| i * (j as u32 + 1) * lens.focal_length)
        })
        .sum()
}
