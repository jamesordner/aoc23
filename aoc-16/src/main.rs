use std::{array::from_fn, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let grid = parse_input(&input);

    let part_one = part_one(grid.clone());
    println!("{part_one}");

    let part_two = part_two(grid);
    println!("{part_two}");
}

fn parse_input(input: &str) -> Grid {
    let mut lines = input.lines();

    from_fn(|_| {
        let line = lines.next().unwrap();
        let mut bytes = line.as_bytes().iter();

        from_fn(|_| {
            let ty = match bytes.next().unwrap() {
                b'/' => TileType::MirrorFS,
                b'\\' => TileType::MirrorBS,
                b'-' => TileType::SplitH,
                b'|' => TileType::SplitV,
                _ => TileType::Empty,
            };

            Tile {
                ty,
                visited_bitmask: 0,
            }
        })
    })
    .into()
}

fn part_one(mut grid: Grid) -> u32 {
    let position = Position {
        coords: [0, 0],
        dir: Dir::Right,
    };

    propogate(&mut grid, &position);

    grid.iter()
        .flatten()
        .filter(|tile| tile.visited_bitmask != 0)
        .count() as u32
}

fn part_two(grid: Grid) -> u32 {
    let left = (0..110).rev().map(|i| Position {
        coords: [i, 109],
        dir: Dir::Left,
    });

    let right = (0..110).map(|i| Position {
        coords: [i, 0],
        dir: Dir::Right,
    });

    let up = (0..110).rev().map(|i| Position {
        coords: [109, i],
        dir: Dir::Up,
    });

    let down = (0..110).map(|i| Position {
        coords: [0, i],
        dir: Dir::Down,
    });

    left.chain(right)
        .chain(up)
        .chain(down)
        .map(|position| {
            let mut grid = grid.clone();

            propogate(&mut grid, &position);

            grid.iter()
                .flatten()
                .filter(|tile| tile.visited_bitmask != 0)
                .count() as u32
        })
        .max()
        .unwrap()
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Right = 0b1000,
}

type Grid = Box<[[Tile; 110]; 110]>;

#[derive(Clone, Copy)]
struct Tile {
    ty: TileType,
    visited_bitmask: u8,
}

#[derive(Clone, Copy)]
enum TileType {
    Empty,
    MirrorFS,
    MirrorBS,
    SplitH,
    SplitV,
}

#[derive(Clone, Copy)]
struct Position {
    coords: [usize; 2],
    dir: Dir,
}

impl Position {
    fn next(&self) -> Option<Position> {
        match self.dir {
            Dir::Up => {
                if self.coords[0] == 0 {
                    None
                } else {
                    let mut position = *self;
                    position.coords[0] -= 1;
                    Some(position)
                }
            }
            Dir::Down => {
                if self.coords[0] == 109 {
                    None
                } else {
                    let mut position = *self;
                    position.coords[0] += 1;
                    Some(position)
                }
            }
            Dir::Left => {
                if self.coords[1] == 0 {
                    None
                } else {
                    let mut position = *self;
                    position.coords[1] -= 1;
                    Some(position)
                }
            }
            Dir::Right => {
                if self.coords[1] == 109 {
                    None
                } else {
                    let mut position = *self;
                    position.coords[1] += 1;
                    Some(position)
                }
            }
        }
    }

    fn reflect(&self, tile_type: TileType) -> Self {
        let dir = match tile_type {
            TileType::MirrorFS => match self.dir {
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Down,
                Dir::Right => Dir::Up,
            },
            TileType::MirrorBS => match self.dir {
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
            },
            _ => unreachable!(),
        };

        Self {
            coords: self.coords,
            dir,
        }
    }
}

fn propogate(grid: &mut Grid, position: &Position) {
    let tile = &mut grid[position.coords[0]][position.coords[1]];

    if tile.visited_bitmask & position.dir as u8 > 0 {
        // already visited
        return;
    }

    tile.visited_bitmask |= position.dir as u8;

    match tile.ty {
        TileType::MirrorFS | TileType::MirrorBS => {
            if let Some(position) = position.reflect(tile.ty).next() {
                propogate(grid, &position);
            }
        }
        TileType::SplitH if position.dir == Dir::Up || position.dir == Dir::Down => {
            for dir in [Dir::Left, Dir::Right] {
                let position = Position {
                    coords: position.coords,
                    dir,
                };

                if let Some(position) = position.next() {
                    propogate(grid, &position);
                }
            }
        }
        TileType::SplitV if position.dir == Dir::Right || position.dir == Dir::Left => {
            for dir in [Dir::Up, Dir::Down] {
                let position = Position {
                    coords: position.coords,
                    dir,
                };

                if let Some(position) = position.next() {
                    propogate(grid, &position);
                }
            }
        }
        _ => {
            if let Some(position) = position.next() {
                propogate(grid, &position);
            }
        }
    }
}
