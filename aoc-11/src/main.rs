use std::fs;

type Coord = [usize; 2];

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let input = parse_input(&input);

    let part_one = distances(&input, 2);
    println!("{part_one}");

    let part_two = distances(&input, 1000000);
    println!("{part_two}");
}

fn distances(input: &Input, multiplier: usize) -> usize {
    input
        .galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, coord)| {
            input.galaxies[i..].iter().map(|other| {
                let extra_rows = (other[0]..coord[0])
                    .chain(coord[0]..other[0])
                    .filter(|i| input.empty_rows.contains(i))
                    .count();

                let extra_cols = (other[1]..coord[1])
                    .chain(coord[1]..other[1])
                    .filter(|i| input.empty_cols.contains(i))
                    .count();

                distance(coord, other) + (extra_rows + extra_cols) * (multiplier - 1)
            })
        })
        .sum()
}

struct Input {
    galaxies: Vec<Coord>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

fn parse_input(input: &str) -> Input {
    let galaxies: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter(|(_, &b)| b == b'#')
                .map(move |(j, _)| [i, j])
        })
        .collect();

    let empty_rows = (0..140)
        .filter(|i| galaxies.iter().all(|[a, _]| i != a))
        .collect();

    let empty_cols = (0..140)
        .filter(|i| galaxies.iter().all(|[_, b]| i != b))
        .collect();

    Input {
        galaxies,
        empty_rows,
        empty_cols,
    }
}

fn distance(a: &Coord, b: &Coord) -> usize {
    a[0].abs_diff(b[0]) + a[1].abs_diff(b[1])
}
