use std::{array::from_fn, collections::HashSet, fs};

use rayon::prelude::*;

type Vec3 = nalgebra_glm::TVec3<u16>;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut bricks = parse_input(&input);

    let part_one = part_one(&mut bricks);
    println!("{part_one}");

    let part_two = part_two(&bricks);
    println!("{part_two}");
}

#[derive(Clone, Copy, Debug)]
struct Brick {
    pos: Vec3,
    dim: Vec3,
}

impl Brick {
    /// Returns `None` if already on the ground.
    fn lowered(mut self) -> Option<Self> {
        if self.pos.z == 1 {
            return None;
        }

        self.pos.z -= 1;

        Some(self)
    }

    fn collides(&self, other: &Self) -> bool {
        self.pos.x < other.pos.x + other.dim.x
            && self.pos.x + self.dim.x > other.pos.x
            && self.pos.y < other.pos.y + other.dim.y
            && self.pos.y + self.dim.y > other.pos.y
            && self.pos.z < other.pos.z + other.dim.z
            && self.pos.z + self.dim.z > other.pos.z
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();

            let mut iter = start.split(',');
            let pos = Vec3::from(from_fn(|_| iter.next().unwrap().parse::<u16>().unwrap()));

            let mut iter = end.split(',');
            let end = Vec3::from(from_fn(|_| iter.next().unwrap().parse::<u16>().unwrap()));

            Brick {
                pos,
                dim: end - pos + Vec3::new(1, 1, 1),
            }
        })
        .collect()
}

fn part_one(bricks: &mut [Brick]) -> u32 {
    settle(bricks);

    (0..bricks.len())
        .into_par_iter()
        .filter(|removed_index| {
            // iterate through all other bricks and see if any are able to move
            for (checked_index, brick) in bricks
                .iter()
                .enumerate()
                .filter(|(i, _)| removed_index != i)
            {
                let Some(brick) = brick.lowered() else {
                    // brick didn't move, check the next one
                    continue;
                };

                if bricks
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| removed_index != i && &checked_index != i)
                    .all(|(_, other)| !brick.collides(other))
                {
                    // a brick fell!
                    return false;
                }
            }

            // no bricks moved
            true
        })
        .count() as u32
}

fn part_two(bricks: &[Brick]) -> u32 {
    (0..bricks.len())
        .into_par_iter()
        .map(|removed_index| {
            let mut bricks = bricks.to_owned();
            bricks.remove(removed_index);

            settle(&mut bricks)
        })
        .sum()
}

/// Returns the number of bricks which moved.
fn settle(bricks: &mut [Brick]) -> u32 {
    let mut moved = false;
    let mut moved_ids = HashSet::new();

    loop {
        for i in 0..bricks.len() {
            let Some(brick) = bricks[i].lowered() else {
                continue;
            };

            if bricks
                .iter()
                .enumerate()
                .filter(|(j, _)| &i != j)
                .all(|(_, other)| !brick.collides(other))
            {
                moved_ids.insert(i);
                bricks[i] = brick;
                moved = true;
            }
        }

        if !moved {
            break;
        }

        moved = false;
    }

    moved_ids.len() as u32
}
