use std::{
    array::from_fn,
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let (workflows, parts) = parse_input(&input);

    let part_one = part_one(&parts, &workflows);
    println!("{part_one}");

    let part_two = part_two(&workflows);
    println!("{part_two}");
}

type Workflows<'a> = HashMap<&'a str, Workflow<'a>>;

#[derive(Debug)]
struct Workflow<'a> {
    instructions: Vec<Instruction<'a>>,
}

#[derive(Debug)]
struct Instruction<'a> {
    op: Option<Operation>,
    dst: Dst<'a>,
}

#[derive(Debug)]
struct Operation {
    ordering: Ordering,
    part_index: usize,
    val: u32,
}

#[derive(Debug)]
enum Dst<'a> {
    Accept,
    Reject,
    Workflow(&'a str),
}

type Part = [u32; 4];

type PartRange = [(u32, u32); 4];

fn parse_input(input: &str) -> (Workflows, Vec<Part>) {
    let mut lines = input.lines();

    let mut workflows = HashMap::new();

    loop {
        let line = lines.next().unwrap();

        if line.is_empty() {
            break;
        }

        let mut iter = line.split(|c| c == '{' || c == ',' || c == '}');
        let label = iter.next().unwrap();

        let instructions = iter
            .filter(|str| !str.is_empty())
            .map(|str| {
                if let Some((op, dst)) = str.split_once(':') {
                    let ordering = if op.contains('<') {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    };

                    let (part_type, val) = op.split_once(|c| c == '<' || c == '>').unwrap();
                    let val = val.parse().unwrap();

                    let part_index = match part_type {
                        "x" => 0,
                        "m" => 1,
                        "a" => 2,
                        "s" => 3,
                        _ => unreachable!(),
                    };

                    Instruction {
                        op: Some(Operation {
                            ordering,
                            part_index,
                            val,
                        }),
                        dst: map_dst(dst),
                    }
                } else {
                    Instruction {
                        op: None,
                        dst: map_dst(str),
                    }
                }
            })
            .collect();

        workflows.insert(label, Workflow { instructions });
    }

    let parts = lines
        .map(|line| {
            let mut iter = line.split(|c| c == '=' || c == ',' || c == '}');

            from_fn(|_| {
                iter.next();
                iter.next().unwrap().parse().unwrap()
            })
        })
        .collect();

    (workflows, parts)
}

fn map_dst(dst: &str) -> Dst {
    match dst {
        "A" => Dst::Accept,
        "R" => Dst::Reject,
        label => Dst::Workflow(label),
    }
}

fn part_one(parts: &[Part], workflows: &Workflows) -> u32 {
    parts
        .iter()
        .filter(|part| accepted(part, workflows))
        .flatten()
        .sum()
}

fn accepted(part: &Part, workflows: &Workflows) -> bool {
    accepted_recursive(part, &workflows["in"], workflows)
}

fn accepted_recursive(part: &Part, workflow: &Workflow, workflows: &Workflows) -> bool {
    for instruction in &workflow.instructions {
        if let Some(op) = &instruction.op {
            if part[op.part_index].cmp(&op.val) == op.ordering {
                return goto_dst(part, &instruction.dst, workflows);
            }
        } else {
            return goto_dst(part, &instruction.dst, workflows);
        }
    }

    unreachable!()
}

fn goto_dst(part: &Part, dst: &Dst, workflows: &Workflows) -> bool {
    match dst {
        Dst::Accept => true,
        Dst::Reject => false,
        Dst::Workflow(dst) => accepted_recursive(part, &workflows[dst], workflows),
    }
}

fn part_two(workflows: &Workflows) -> u64 {
    let mut accepted_ranges = HashSet::new();

    calc_accepted_ranges(
        from_fn(|_| (1, 4000)),
        &workflows["in"],
        workflows,
        &mut accepted_ranges,
    );

    accepted_ranges
        .into_iter()
        .map(|range| {
            range
                .map(|a| (a.1 + 1 - a.0) as u64)
                .into_iter()
                .reduce(|acc, a| acc * a)
                .unwrap()
        })
        .sum()
}

fn calc_accepted_ranges(
    mut part_range: PartRange,
    workflow: &Workflow,
    workflows: &Workflows,
    accepted_ranges: &mut HashSet<PartRange>,
) {
    for instruction in &workflow.instructions {
        if let Some(op) = &instruction.op {
            let mut part_range_branch = part_range;

            if op.ordering == Ordering::Less {
                part_range_branch[op.part_index].1 = op.val - 1;
                part_range[op.part_index].0 = op.val;
            } else {
                part_range[op.part_index].1 = op.val;
                part_range_branch[op.part_index].0 = op.val + 1;
            }

            branch_range(
                part_range_branch,
                &instruction.dst,
                workflows,
                accepted_ranges,
            );
        } else {
            branch_range(part_range, &instruction.dst, workflows, accepted_ranges);
        }
    }
}

fn branch_range(
    part_range: PartRange,
    dst: &Dst,
    workflows: &Workflows,
    accepted_ranges: &mut HashSet<PartRange>,
) {
    match dst {
        Dst::Accept => {
            accepted_ranges.insert(part_range);
        }
        Dst::Workflow(dst) => {
            calc_accepted_ranges(part_range, &workflows[dst], workflows, accepted_ranges);
        }
        Dst::Reject => {}
    }
}
