use core::panic;

fn main() {
    let input = include_str!("part1.txt");
    let out = part2(input);
    dbg!(out);
}

struct Lexer<'a> {
    line: &'a str,
}

impl<'a> Lexer<'a> {
    fn read_next_digit(&mut self) -> Option<u8> {
        if self.line.is_empty() {
            return None;
        }
        let char = self.line.chars().nth(0).expect("Where is my char ?");
        if char > '0' && char <= '9' {
            self.line = &self.line[1..];
            return Some(char as u8 - b'0');
        }
        if self.line.starts_with("one") {
            self.line = &self.line[1..];
            return Some(1);
        }
        if self.line.starts_with("two") {
            self.line = &self.line[1..];
            return Some(2);
        }
        if self.line.starts_with("three") {
            self.line = &self.line[1..];
            return Some(3);
        }
        if self.line.starts_with("four") {
            self.line = &self.line[1..];
            return Some(4);
        }
        if self.line.starts_with("five") {
            self.line = &self.line[1..];
            return Some(5);
        }
        if self.line.starts_with("six") {
            self.line = &self.line[1..];
            return Some(6);
        }
        if self.line.starts_with("seven") {
            self.line = &self.line[1..];
            return Some(7);
        }
        if self.line.starts_with("eight") {
            self.line = &self.line[1..];
            return Some(8);
        }
        if self.line.starts_with("nine") {
            self.line = &self.line[1..];
            return Some(9);
        }
        self.line = &self.line[1..];
        self.read_next_digit()
    }
}

fn process_line(line: &str) -> u64 {
    let mut first_and_last: (Option<u8>, Option<u8>) = (None, None);

    let mut lexer = Lexer { line };
    while let Some(digit) = lexer.read_next_digit() {
        if first_and_last.0.is_none() {
            first_and_last.0 = Some(digit);
        }
        first_and_last.1 = Some(digit);
    }
    match first_and_last {
        (Some(first), None) => first as u64,
        (Some(first), Some(second)) => first as u64 * 10 + second as u64,
        (None, Some(_)) => panic!("Something went wrong 😟"),
        (None, None) => 0,
    }
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(process_line)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        let expected_result = 281;
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, expected_result);
    }
    #[test]
    fn example_lines() {
        assert_eq!(29, process_line("two1nine"));
        assert_eq!(83, process_line("eightwothree"));
        assert_eq!(13, process_line("abcone2threexyz"));
        assert_eq!(24, process_line("xtwone3four"));
        assert_eq!(42, process_line("4nineeightseven2"));
        assert_eq!(14, process_line("zoneight234"));
        assert_eq!(76, process_line("7pqrstsixteen"));
    }

    #[test]
    fn example_edge_cases() {
        assert_eq!(18, process_line("oneight"));
        assert_eq!(11, process_line("one7one"));
        assert_eq!(78, process_line("sevenineight"));
        assert_eq!(82, process_line("eightwo"));
        assert_eq!(51, process_line("fivezg8jmf6hrxnhgxxttwoneg"));
        assert_eq!(79, process_line("fplrjjznseventwocrv9"));
        assert_eq!(95, process_line("977ckpkmx5"));
    }
}
