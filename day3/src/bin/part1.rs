use std::collections::HashSet;

fn main() {
    let input = include_str!("part1.txt");
    let out = part1(input);
    dbg!(out);
}

fn part1(input: &str) -> u64 {
    let schematic = parse_schematic(input);
    schematic
        .part_numbers
        .into_iter()
        .map(|part_number| {
            let position_diffs: [(i64, i64); 8] = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            for (line_diff, column_diff) in position_diffs {
                if part_number.line == 0 && line_diff == -1 {
                    continue;
                }
                for column in part_number.start_column..=part_number.end_column {
                    if column == 0 && column_diff == -1 {
                        continue;
                    }
                    if schematic.symbols.contains(&Position {
                        line: (part_number.line as i64 + line_diff) as usize,
                        column: (column as i64 + column_diff) as usize,
                    }) {
                        return part_number.value;
                    }
                }
            }
            0
        })
        .sum()
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Position {
    line: usize,
    column: usize,
}

#[derive(Eq, PartialEq, Debug)]
struct PartNumber {
    value: u64,
    line: usize,
    start_column: usize,
    end_column: usize,
}

#[derive(Debug)]
struct Schematic {
    symbols: HashSet<Position>,
    part_numbers: Vec<PartNumber>,
}

fn parse_schematic(input: &str) -> Schematic {
    let mut schematic = Schematic {
        symbols: HashSet::new(),
        part_numbers: vec![],
    };
    for (line_no, line) in input.lines().enumerate() {
        let mut current_part_number: Option<(String, usize, usize, usize)> = None;
        let mut part_number_close =
            |current_part_number: &mut Option<(String, usize, usize, usize)>| {
                if let Some((part_number_value, line_no, start_column, end_column)) =
                    current_part_number
                {
                    schematic.part_numbers.push(PartNumber {
                        value: part_number_value.parse::<u64>().unwrap(),
                        line: *line_no,
                        start_column: *start_column,
                        end_column: *end_column,
                    });
                    *current_part_number = None;
                }
            };
        for (column_no, char) in line.chars().enumerate() {
            match char {
                '0'..='9' => match current_part_number {
                    None => {
                        current_part_number =
                            Some((String::from(char), line_no, column_no, column_no));
                    }
                    Some((ref mut part_number_value, _, _, ref mut end_column)) => {
                        *end_column = column_no;
                        part_number_value.push(char);
                    }
                },
                '*' | '#' | '+' | '$' | '-' | '%' | '&' | '/' | '@' | '=' => {
                    part_number_close(&mut current_part_number);
                    schematic.symbols.insert(Position {
                        line: line_no,
                        column: column_no,
                    });
                }
                '.' => {
                    part_number_close(&mut current_part_number);
                }
                _ => panic!("Unexpected character found {}", char),
            }
        }
        part_number_close(&mut current_part_number);
    }
    schematic
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        let expected_result = 4361;
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, expected_result);
    }

    #[test]
    fn parser_example() {
        let result = parse_schematic(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(
            result.symbols,
            HashSet::from([
                Position { line: 1, column: 3 },
                Position { line: 3, column: 6 },
                Position { line: 4, column: 3 },
                Position { line: 5, column: 5 },
                Position { line: 8, column: 3 },
                Position { line: 8, column: 5 },
            ])
        );
        assert_eq!(
            result.part_numbers,
            vec![
                PartNumber {
                    value: 467,
                    line: 0,
                    start_column: 0,
                    end_column: 2
                },
                PartNumber {
                    value: 114,
                    line: 0,
                    start_column: 5,
                    end_column: 7
                },
                PartNumber {
                    value: 35,
                    line: 2,
                    start_column: 2,
                    end_column: 3
                },
                PartNumber {
                    value: 633,
                    line: 2,
                    start_column: 6,
                    end_column: 8
                },
                PartNumber {
                    value: 617,
                    line: 4,
                    start_column: 0,
                    end_column: 2
                },
                PartNumber {
                    value: 58,
                    line: 5,
                    start_column: 7,
                    end_column: 8
                },
                PartNumber {
                    value: 592,
                    line: 6,
                    start_column: 2,
                    end_column: 4
                },
                PartNumber {
                    value: 755,
                    line: 7,
                    start_column: 6,
                    end_column: 8
                },
                PartNumber {
                    value: 664,
                    line: 9,
                    start_column: 1,
                    end_column: 3
                },
                PartNumber {
                    value: 598,
                    line: 9,
                    start_column: 5,
                    end_column: 7
                }
            ]
        );
    }
}
