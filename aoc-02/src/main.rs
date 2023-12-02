use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let result_one = part_one(&input);
    let result_two = part_two(&input);

    println!("{result_one}");
    println!("{result_two}");
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(game_info)
        .filter(|(_, game)| game_possible(game))
        .map(|(game_number, _)| game_number)
        .sum()
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(game_info)
        .map(|(_, game)| game_power(game))
        .sum()
}

fn game_possible(game: &str) -> bool {
    dice_pairs(game).all(|(quantity, color_index)| quantity <= [12, 13, 14][color_index])
}

fn game_power(game: &str) -> u32 {
    dice_pairs(game)
        .fold([0; 3], |mut acc, (quantity, color_index)| {
            acc[color_index] = acc[color_index].max(quantity);
            acc
        })
        .into_iter()
        .reduce(|acc, a| acc * a)
        .unwrap()
}

/// Returns `(game number, game str)`
fn game_info(line: &str) -> (u32, &str) {
    let i = line.find(':').unwrap();
    let game_number = line[5..i].parse::<u32>().unwrap();
    (game_number, &line[i + 1..])
}

/// Returns an iterator over all pairs `(quantity, color index)` for all sets in the game
fn dice_pairs(game: &str) -> impl Iterator<Item = (u32, usize)> + '_ {
    game.split(';').flat_map(|set| set.split(',')).map(|pair| {
        let mut iter = pair.split_whitespace();
        let quantity = iter.next().unwrap().parse::<u32>().unwrap();
        let color = iter.next().unwrap();

        let index = ["red", "green", "blue"]
            .iter()
            .position(|label| color == *label)
            .unwrap();

        (quantity, index)
    })
}
