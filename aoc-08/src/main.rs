use std::{collections::HashMap, fs};

use num::integer::lcm;

type Loc = [u8; 3];

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let (directions, map) = parse_input(&input);

    let result_one = part_one(&directions, &map);
    println!("{result_one}");

    let result_two = part_two(&directions, &map);
    println!("{result_two}");
}

fn part_one(directions: &[usize], map: &HashMap<Loc, [Loc; 2]>) -> u64 {
    step_count(*b"AAA", directions, map, |loc| loc == *b"ZZZ")
}

fn part_two(directions: &[usize], map: &HashMap<Loc, [Loc; 2]>) -> u64 {
    map.keys()
        .filter(|key| key[2] == b'A')
        .copied()
        .map(|loc| step_count(loc, directions, map, |loc| loc[2] == b'Z'))
        .reduce(lcm)
        .unwrap()
}

fn step_count<F>(
    mut location: Loc,
    directions: &[usize],
    map: &HashMap<Loc, [Loc; 2]>,
    condition: F,
) -> u64
where
    F: Fn(Loc) -> bool,
{
    let mut count = 0;

    for dir in directions.iter().cycle() {
        if condition(location) {
            break;
        }

        location = map[&location][*dir];
        count += 1;
    }

    count
}

fn parse_input(input: &str) -> (Vec<usize>, HashMap<Loc, [Loc; 2]>) {
    let mut lines = input.lines();

    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|char| if char == 'L' { 0 } else { 1 })
        .collect();

    lines.next();

    let map = lines
        .map(|line| {
            (
                line[0..3].as_bytes().try_into().unwrap(),
                [
                    line[7..10].as_bytes().try_into().unwrap(),
                    line[12..15].as_bytes().try_into().unwrap(),
                ],
            )
        })
        .collect();

    (directions, map)
}
