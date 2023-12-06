use itertools::Itertools;
use std::cmp::max;

fn main() {
    let input = include_str!("part1.txt");
    let out = part2(input);
    dbg!(out);
}

fn part2(input: &str) -> u64 {
    input.lines().map(process_line).sum()
}

struct CubeSet {
    red: u64,
    green: u64,
    blue: u64,
}

impl CubeSet {
    fn from(input: &str) -> Self {
        let mut cube_set = CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        };
        let cubes = input.trim().split(',').collect::<Vec<&str>>();
        for cube in cubes {
            let (number, color) = cube
                .split_whitespace()
                .collect_tuple()
                .expect("Invalid Cube");
            match color {
                "red" => cube_set.red = number.parse::<u64>().expect("Invalid cube number"),
                "green" => cube_set.green = number.parse::<u64>().expect("Invalid cube number"),
                "blue" => cube_set.blue = number.parse::<u64>().expect("Invalid cube number"),
                _ => panic!("Invalid color {}", color),
            }
        }

        cube_set
    }

    fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }

    fn max(&self, other: &Self) -> Self {
        CubeSet {
            red: max(self.red, other.red),
            green: max(self.green, other.green),
            blue: max(self.blue, other.blue),
        }
    }
}

fn process_line(input: &str) -> u64 {
    input
        .trim()
        .split(':')
        .nth(1)
        .expect("Unable to split at game level")
        .trim()
        .split(';')
        .map(CubeSet::from)
        .reduce(|a, b| a.max(&b))
        .expect("Expected at least on cube set")
        .power()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        let expected_result = 2286;
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, expected_result);
    }

    #[test]
    fn example_lines() {
        let test_cases = vec![
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                12,
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                1560,
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                630,
            ),
            ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36),
        ];
        for test_case in test_cases {
            assert_eq!(process_line(test_case.0), test_case.1);
        }
    }
}
