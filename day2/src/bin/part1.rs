#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
lazy_static! {
    static ref MAP: HashMap<&'static str, u64> = [("red", 12), ("green", 13), ("blue", 14),]
        .iter()
        .copied()
        .collect();
}

fn main() {
    let input = include_str!("part1.txt");
    let out = part1(input);
    dbg!(out);
}

fn part1(input: &str) -> u64 {
    input.lines().map(process_line).sum()
}

fn is_invalid(input: &str) -> bool {
    input.split(',').any(|cube| {
        let mut desc = cube.trim().split(' ');
        desc.next()
            .expect("Invalid cube")
            .parse::<u64>()
            .expect("Invalid cube number")
            > *MAP
                .get(desc.next().expect("Incomplete cube"))
                .expect("Invalid cube color")
    })
}

fn process_line(input: &str) -> u64 {
    let mut top_level = input.trim().split(':');
    let game_id = top_level
        .next()
        .expect("Unable to split at top level")
        .trim()
        .split(' ')
        .nth(1)
        .expect("Unable to split in game id")
        .parse::<u64>()
        .expect("Invalid game id");
    if top_level
        .next()
        .expect("Unable to split at game level")
        .trim()
        .split(';')
        .any(is_invalid)
    {
        return 0;
    }
    game_id
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        let expected_result = 8;
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, expected_result);
    }
}
