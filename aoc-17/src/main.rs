use std::{
    array::from_fn,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs,
    ops::RangeInclusive,
};

use num::{Complex, Zero};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let grid = parse_input(&input);

    let part_one = part_one(&grid);
    println!("{part_one}");

    let part_two = part_two(&grid);
    println!("{part_two}");
}

type Grid = [[isize; LEN as usize]; LEN as usize];
type Coord = Complex<isize>;

const LEN: isize = 141;

const GOAL: Coord = Coord::new(LEN - 1, LEN - 1);

const DIRS: [Coord; 4] = [
    Complex::new(1, 0),
    Complex::new(0, 1),
    Complex::new(-1, 0),
    Complex::new(0, -1),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct CoordEntry {
    coord: Coord,
    dir: Coord,
    consecutive: usize,
}

#[derive(Clone, Copy, Debug)]
struct Node {
    coord: CoordEntry,
    dist: isize,
}

impl Node {
    fn start() -> [Self; 4] {
        DIRS.map(|dir| Self {
            coord: CoordEntry {
                coord: Coord::new(0, 0),
                dir,
                consecutive: 1,
            },
            dist: 0,
        })
    }

    fn next(&self, dir: Coord, turn_range: RangeInclusive<usize>, grid: &Grid) -> Option<Self> {
        if self.coord.dir + dir == Coord::zero() {
            return None;
        }

        if self.coord.dir != dir && !turn_range.contains(&self.coord.consecutive) {
            return None;
        }

        let consecutive = if self.coord.dir == dir {
            self.coord.consecutive + 1
        } else {
            1
        };

        if consecutive > *turn_range.end() {
            return None;
        }

        let coord = CoordEntry {
            coord: self.coord.coord + dir,
            dir,
            consecutive,
        };

        if (0..LEN).contains(&coord.coord.re) && (0..LEN).contains(&coord.coord.im) {
            Some(Self {
                coord,
                dist: self.dist + grid[coord.coord.re as usize][coord.coord.im as usize],
            })
        } else {
            None
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

fn parse_input(input: &str) -> Box<Grid> {
    let mut lines = input.lines();

    from_fn(|_| {
        let mut line = lines.next().unwrap().as_bytes().iter();
        from_fn(|_| (line.next().unwrap() - b'0').into())
    })
    .into()
}

fn part_one(grid: &Grid) -> isize {
    find_path(grid, 1..=3)
}

fn part_two(grid: &Grid) -> isize {
    find_path(grid, 4..=10)
}

fn find_path(grid: &Grid, turn_range: RangeInclusive<usize>) -> isize {
    let start_nodes = Node::start();

    let mut open_set = BinaryHeap::from(start_nodes);
    let mut came_from = HashMap::new();
    let mut abs_score = HashMap::from(start_nodes.map(|node| (node.coord, node.dist)));

    while let Some(current) = open_set.pop() {
        if current.coord.coord == GOAL {
            continue;
        }

        for next in DIRS
            .iter()
            .filter_map(|&dir| current.next(dir, turn_range.clone(), grid))
        {
            let current_score = abs_score.entry(next.coord).or_insert(isize::MAX);

            if next.dist < *current_score {
                *current_score = next.dist;
                came_from.insert(next.coord, current.coord);
                open_set.push(next);
            }
        }
    }

    let abs_score = &abs_score;

    DIRS.iter()
        .flat_map(|&dir| {
            (1..=*turn_range.end()).flat_map(move |consecutive| {
                abs_score.get(&CoordEntry {
                    coord: GOAL,
                    dir,
                    consecutive,
                })
            })
        })
        .copied()
        .min()
        .unwrap()
}
