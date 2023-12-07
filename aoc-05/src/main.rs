use std::{
    array::from_fn,
    fs,
    num::NonZeroUsize,
    str::Lines,
    sync::atomic::{AtomicU64, Ordering},
    thread::{available_parallelism, scope},
};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let input = parse_input(&input);

    let result_one = part_one(&input);
    let result_two = part_two(&input);

    println!("{result_one}");
    println!("{result_two}");
}

fn part_one(input: &Input) -> u64 {
    input
        .seeds
        .iter()
        .map(|seed| find_location(input, *seed))
        .min()
        .unwrap()
}

fn part_two(input: &Input) -> u64 {
    let parallelism = available_parallelism().map(NonZeroUsize::get).unwrap_or(1) as u64;
    let min_location = AtomicU64::new(u64::MAX);

    for chunk in input.seeds.chunks_exact(2) {
        let seed_start = chunk[0];
        let seed_end = chunk[0] + chunk[1];
        let min_location = &min_location;

        scope(|s| {
            for i in 0..parallelism {
                s.spawn(move || {
                    let chunk_start = seed_start + (seed_end - seed_start) * i / parallelism;
                    let chunk_end = seed_start + (seed_end - seed_start) * (i + 1) / parallelism;

                    let location = (chunk_start..chunk_end)
                        .map(|seed| find_location(input, seed))
                        .min()
                        .unwrap();

                    min_location.fetch_min(location, Ordering::Relaxed);
                });
            }
        });
    }

    min_location.into_inner()
}

fn find_location(input: &Input, mut seed: u64) -> u64 {
    for mapping in &input.range_mappings {
        if let Some(m) = mapping
            .iter()
            .find(|m| (m.src..m.src + m.range).contains(&seed))
        {
            seed = m.dst + (seed - m.src);
        }
    }

    seed
}

struct Input {
    seeds: Vec<u64>,
    range_mappings: [Vec<RangeMap>; 7],
}

#[derive(Debug)]
struct RangeMap {
    dst: u64,
    src: u64,
    range: u64,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|str| str.parse().unwrap())
        .collect();

    let range_mappings = from_fn(|_| parse_mapping(&mut lines));

    Input {
        seeds,
        range_mappings,
    }
}

fn parse_mapping(lines: &mut Lines) -> Vec<RangeMap> {
    let mut mapping = Vec::new();

    lines.next();
    lines.next();

    loop {
        let Some(line) = lines.next() else {
            break;
        };

        if line.is_empty() {
            break;
        }

        let mut vals = line.split_whitespace();

        let dst = vals.next().unwrap().parse().unwrap();
        let src = vals.next().unwrap().parse().unwrap();
        let range = vals.next().unwrap().parse().unwrap();

        mapping.push(RangeMap { dst, src, range });
    }

    mapping
}
