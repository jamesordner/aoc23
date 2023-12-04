use std::{array::from_fn, collections::HashMap, fs, ops::Range, str};

const GRID_SIZE: usize = 140;
type Grid = [[u8; GRID_SIZE]; GRID_SIZE];

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut lines = input.lines();

    let grid = from_fn(|_| {
        let line = lines.next().unwrap();
        let mut bytes = line.bytes();
        from_fn(|_| bytes.next().unwrap())
    });

    part_one_and_two(&grid);
}

fn part_one_and_two(grid: &Grid) {
    let mut part_number_sum = 0;
    let mut ratio_sum = 0;

    let mut gears = HashMap::new();

    for (row_index, line) in grid.iter().enumerate() {
        let mut col_index_start = 0;

        while col_index_start < GRID_SIZE {
            if line[col_index_start].is_ascii_digit() {
                // we've found the start of a number, now find the end
                let mut col_index_end = col_index_start + 1;

                while col_index_end < GRID_SIZE && line[col_index_end].is_ascii_digit() {
                    col_index_end += 1;
                }

                let part_number = str::from_utf8(&line[col_index_start..col_index_end])
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();

                let gear_found = |ptr| {
                    if let Some(val) = gears.remove(&ptr) {
                        ratio_sum += part_number * val;
                    } else {
                        gears.insert(ptr, part_number);
                    }
                };

                if is_range_adjacent(grid, row_index, col_index_start..col_index_end, gear_found) {
                    part_number_sum += part_number;
                }

                col_index_start = col_index_end;
            } else {
                col_index_start += 1;
            }
        }
    }

    println!("{part_number_sum}");
    println!("{ratio_sum}");
}

fn is_range_adjacent<F>(
    grid: &Grid,
    row_index: usize,
    col_range: Range<usize>,
    mut gear_found: F,
) -> bool
where
    F: FnMut(*const u8),
{
    let col_index_start = col_range.start.saturating_sub(1);
    let col_index_end = (col_range.end + 1).min(GRID_SIZE);

    let mut adjacent = false;

    if row_index > 0 {
        for char in &grid[row_index - 1][col_index_start..col_index_end] {
            if *char != b'.' {
                adjacent = true;
            }

            if *char == b'*' {
                gear_found(char as *const u8);
            }
        }
    }

    if row_index < GRID_SIZE - 1 {
        for char in &grid[row_index + 1][col_index_start..col_index_end] {
            if *char != b'.' {
                adjacent = true;
            }

            if *char == b'*' {
                gear_found(char as *const u8);
            }
        }
    }

    if col_range.start > 0 {
        let char = &grid[row_index][col_index_start];
        if *char != b'.' {
            adjacent = true;
        }

        if *char == b'*' {
            gear_found(char as *const u8);
        }
    }

    if col_range.end < GRID_SIZE - 1 {
        let char = &grid[row_index][col_range.end];
        if *char != b'.' {
            adjacent = true;
        }

        if *char == b'*' {
            gear_found(char as *const u8);
        }
    }

    adjacent
}
