use std::{array::from_fn, fs};

type Grid = [[bool; 100]; 100];
type Coord = [usize; 2];

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let (mut grid, mut rocks) = parse_input(&input);

    let part_one = part_one(&mut grid, &mut rocks);
    println!("{part_one}");

    // reload input to start fresh
    let (mut grid, mut rocks) = parse_input(&input);

    let part_two = part_two(&mut grid, &mut rocks);
    println!("{part_two}");
}

fn part_one(grid: &mut Grid, rocks: &mut [Coord]) -> u32 {
    roll_north(grid, rocks);
    total_load(rocks)
}

fn part_two(grid_tortoise: &mut Grid, rocks_tortoise: &mut [Coord]) -> u32 {
    const TOTAL_CYCLES: u32 = 1000000000;

    let mut grid_hare = *grid_tortoise;
    let grid_hare = &mut grid_hare;

    let mut rocks_hare: Vec<_> = rocks_tortoise.into();
    let rocks_hare = rocks_hare.as_mut_slice();

    cycle(grid_tortoise, rocks_tortoise);

    cycle(grid_hare, rocks_hare);
    cycle(grid_hare, rocks_hare);

    let mut i = 1;

    while grid_tortoise != grid_hare {
        cycle(grid_tortoise, rocks_tortoise);

        cycle(grid_hare, rocks_hare);
        cycle(grid_hare, rocks_hare);

        i += 1;
    }

    let remaining_cycles = TOTAL_CYCLES - TOTAL_CYCLES / i * i;

    for _ in 0..remaining_cycles {
        cycle(grid_tortoise, rocks_tortoise);
    }

    total_load(rocks_tortoise)
}

fn roll_north(grid: &mut Grid, rocks: &mut [Coord]) {
    rocks.sort_unstable();

    for rock in rocks.iter_mut() {
        while rock[0] > 0 && !grid[rock[0] - 1][rock[1]] {
            grid[rock[0]][rock[1]] = false;
            rock[0] -= 1;
            grid[rock[0]][rock[1]] = true;
        }
    }
}

fn roll_east(grid: &mut Grid, rocks: &mut [Coord]) {
    rocks.sort_unstable_by(|a, b| (100 - a[1]).cmp(&(100 - b[1])));

    for rock in rocks.iter_mut() {
        while rock[1] < 99 && !grid[rock[0]][rock[1] + 1] {
            grid[rock[0]][rock[1]] = false;
            rock[1] += 1;
            grid[rock[0]][rock[1]] = true;
        }
    }
}

fn roll_south(grid: &mut Grid, rocks: &mut [Coord]) {
    rocks.sort_unstable_by(|a, b| (100 - a[0]).cmp(&(100 - b[0])));

    for rock in rocks.iter_mut() {
        while rock[0] < 99 && !grid[rock[0] + 1][rock[1]] {
            grid[rock[0]][rock[1]] = false;
            rock[0] += 1;
            grid[rock[0]][rock[1]] = true;
        }
    }
}

fn roll_west(grid: &mut Grid, rocks: &mut [Coord]) {
    rocks.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

    for rock in rocks.iter_mut() {
        while rock[1] > 0 && !grid[rock[0]][rock[1] - 1] {
            grid[rock[0]][rock[1]] = false;
            rock[1] -= 1;
            grid[rock[0]][rock[1]] = true;
        }
    }
}

fn cycle(grid: &mut Grid, rocks: &mut [Coord]) {
    roll_north(grid, rocks);
    roll_west(grid, rocks);
    roll_south(grid, rocks);
    roll_east(grid, rocks);
}

fn total_load(rocks: &[Coord]) -> u32 {
    rocks.iter().map(|[a, _]| 100 - *a as u32).sum()
}

fn parse_input(input: &str) -> (Grid, Vec<Coord>) {
    let mut lines = input.lines();

    let grid = from_fn(|_| {
        let mut bytes = lines.next().unwrap().as_bytes().iter();
        from_fn(|_| *bytes.next().unwrap() != b'.')
    });

    let rocks = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter(|(_, &b)| b == b'O')
                .map(move |(j, _)| [i, j])
        })
        .collect();

    (grid, rocks)
}
