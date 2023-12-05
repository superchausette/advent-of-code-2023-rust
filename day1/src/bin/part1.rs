use core::panic;

fn main() {
    let input = include_str!("part1.txt");
    let out = part1(input);
    dbg!(out);
}

fn part1(input: &str) -> u64 {
    input.lines().map(process_line).fold(0, |acc, val| acc + val)
}

fn process_line(line: &str) -> u64 {
    let mut first_and_last: (Option<u8>, Option<u8>) = (None, None);
    for char in line.chars() {
        if !char.is_ascii_digit() {
            continue;
        }
        let digit = char as u8 - b'0';
        if first_and_last.0.is_none() {
            first_and_last.0 = Some(digit);
        }
        first_and_last.1 = Some(char as u8 - b'0');
    }
    match first_and_last {
        (Some(first), None) => first as u64,
        (Some(first), Some(second)) => first as u64 * 10 + second as u64,
        (None, Some(_)) => panic!("Something went wrong 😟"),
        (None, None) =>  0,
    }

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        let expected_result = 142;
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, expected_result);
    }
}
