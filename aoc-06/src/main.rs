use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let result_one = part_one(&input);
    println!("{result_one}");

    let result_two = part_two(&input);
    println!("{result_two}");
}

fn part_one(input: &str) -> u64 {
    parse_input_part_one(input)
        .iter()
        .map(ways_to_win)
        .reduce(|acc, a| acc * a)
        .unwrap()
}

fn part_two(input: &str) -> u64 {
    let race_info = parse_input_part_two(input);
    ways_to_win(&race_info)
}

fn ways_to_win(race_info: &RaceInfo) -> u64 {
    (1..race_info.time)
        .filter(|hold_time| (race_info.time - hold_time) * hold_time > race_info.record)
        .count() as u64
}

#[derive(Debug)]
struct RaceInfo {
    time: u64,
    record: u64,
}

fn parse_input_part_one(input: &str) -> Vec<RaceInfo> {
    let mut lines = input.lines();

    let mut race_info = lines.next().unwrap()[10..]
        .split_whitespace()
        .map(|str| RaceInfo {
            time: str.parse().unwrap(),
            record: 0,
        })
        .collect::<Vec<_>>();

    lines.next().unwrap()[10..]
        .split_whitespace()
        .map(|str| str.parse().unwrap())
        .zip(&mut race_info)
        .for_each(|(record, info)| info.record = record);

    race_info
}

fn parse_input_part_two(input: &str) -> RaceInfo {
    let mut lines = input.lines();

    let time = lines.next().unwrap()[10..]
        .split_whitespace()
        .fold(String::new(), |acc, str| acc + str)
        .parse()
        .unwrap();

    let record = lines.next().unwrap()[10..]
        .split_whitespace()
        .fold(String::new(), |acc, str| acc + str)
        .parse()
        .unwrap();

    RaceInfo { time, record }
}
