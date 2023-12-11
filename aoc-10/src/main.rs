use std::{array::from_fn, collections::HashSet, fs, ops::Range};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let grid = parse_input(&input);

    // Contained coords are twice as big, to accomodate space between pipes
    let mut pipe_coords = HashSet::new();

    let part_one = part_one(&grid, &mut pipe_coords);
    println!("{part_one}");

    let part_two = part_two(&pipe_coords);
    println!("{part_two}");
}

fn part_one(grid: &Grid, pipe_coords: &mut HashSet<Coord>) -> u32 {
    let mut positions = Position::start_pair(grid);
    let mut steps = 1;

    // insert starting coord
    pipe_coords.insert(positions[0].prev.map(|c| c * 2));

    while positions[0].curr != positions[1].curr {
        for position in &mut positions {
            insert_pipe_coords(position, pipe_coords);

            // follow the pipe
            position.increment(grid);
        }

        steps += 1;
    }

    for position in &positions {
        insert_pipe_coords(position, pipe_coords);
    }

    steps
}

fn part_two(pipe_coords: &HashSet<Coord>) -> u32 {
    (0..140)
        .flat_map(|y| (0..140).map(move |x| [y * 2, x * 2]))
        .filter(|coord| !pipe_coords.contains(coord))
        .filter(|coord| !is_outside(coord, pipe_coords))
        .count() as u32
}

fn parse_input(input: &str) -> Grid {
    let mut lines = input.lines();
    from_fn(|_| lines.next().unwrap().as_bytes().try_into().unwrap())
}

type Grid = [[u8; 140]; 140];
type Coord = [usize; 2];

fn start_coord(grid: &Grid) -> Coord {
    (0..140)
        .flat_map(|y| (0..140).map(move |x| [y, x]))
        .find(|coord| grid[coord[0]][coord[1]] == b'S')
        .unwrap()
}

#[derive(Clone, Copy, Debug, Default)]
struct Position {
    prev: Coord,
    curr: Coord,
}

impl Position {
    fn start_pair(grid: &Grid) -> [Self; 2] {
        let prev = start_coord(grid);

        adjacent_coords(prev, 0..140)
            .filter(|&coord| {
                adjacent_pipes(grid, coord)
                    .map(|adjacent| adjacent.contains(&prev))
                    .unwrap_or(false)
            })
            .map(|curr| Self { prev, curr })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn increment(&mut self, grid: &Grid) {
        let adjacent = adjacent_pipes(grid, self.curr).unwrap();
        let next = adjacent.iter().find(|&c| c != &self.prev).unwrap();

        self.prev = self.curr;
        self.curr = *next;
    }
}

fn adjacent_coords(coord: Coord, range: Range<isize>) -> impl Iterator<Item = Coord> {
    [[-1, 0], [1, 0], [0, 1], [0, -1]]
        .iter()
        .map(move |offset| [coord[0] as isize + offset[0], coord[1] as isize + offset[1]])
        .filter(move |coord| coord.iter().all(|c| range.contains(c)))
        .map(|coord| coord.map(|c| c as usize))
}

fn adjacent_pipes(grid: &Grid, coord: Coord) -> Option<[Coord; 2]> {
    let coords = match grid[coord[0]][coord[1]] {
        b'|' => [[coord[0] - 1, coord[1]], [coord[0] + 1, coord[1]]],
        b'-' => [[coord[0], coord[1] - 1], [coord[0], coord[1] + 1]],
        b'L' => [[coord[0] - 1, coord[1]], [coord[0], coord[1] + 1]],
        b'J' => [[coord[0] - 1, coord[1]], [coord[0], coord[1] - 1]],
        b'7' => [[coord[0] + 1, coord[1]], [coord[0], coord[1] - 1]],
        b'F' => [[coord[0] + 1, coord[1]], [coord[0], coord[1] + 1]],
        _ => return None,
    };

    Some(coords)
}

fn insert_pipe_coords(position: &Position, pipe_coords: &mut HashSet<Coord>) {
    // insert current coord (doubled) and intermediate previous coord
    let pipe_coord = position.curr.map(|c| c * 2);

    let diff = [
        position.prev[0] as isize - position.curr[0] as isize,
        position.prev[1] as isize - position.curr[1] as isize,
    ];

    let intermediate_coord = [
        (pipe_coord[0] as isize + diff[0]) as usize,
        (pipe_coord[1] as isize + diff[1]) as usize,
    ];

    pipe_coords.insert(intermediate_coord);
    pipe_coords.insert(pipe_coord);
}

fn is_outside(coord: &Coord, pipe_coords: &HashSet<Coord>) -> bool {
    is_outside_recursive(*coord, pipe_coords, &mut HashSet::new())
}

fn is_outside_recursive(
    coord: Coord,
    pipe_coords: &HashSet<Coord>,
    visited: &mut HashSet<Coord>,
) -> bool {
    if visited.contains(&coord) {
        return false;
    }

    if pipe_coords.contains(&coord) {
        return false;
    }

    if on_edge(coord) {
        return true;
    }

    visited.insert(coord);

    adjacent_coords(coord, 0..140 * 2)
        .any(|coord| is_outside_recursive(coord, pipe_coords, visited))
}

fn on_edge(coord: Coord) -> bool {
    coord.iter().any(|c| !(1..140 * 2).contains(c))
}
