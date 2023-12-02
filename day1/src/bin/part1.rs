use core::panic;

fn main() {
    let input = include_str!("part1.txt");
    let out = part1(input);
    dbg!(out);
}

fn part1(input: &str) -> u64 {
    let mut calibration_value: u64 = 0;
    for line in input.lines() {
        let mut first_and_last: (Option<u8>, Option<u8>) = (None, None);
        for char in line.chars() {
            if !char.is_digit(10) {
                continue;
            }
            let digit = char as u8 - '0' as u8;
            if first_and_last.0.is_none() {
                first_and_last.0 = Some(digit);
            }
            first_and_last.1 = Some(char as u8 - '0' as u8);
        }
        match first_and_last {
            (Some(first), None) => calibration_value += first as u64,
            (Some(first), Some(second)) => calibration_value += first as u64 * 10 + second as u64,
            (None, Some(_)) => panic!("Something went wrong 😟"),
            (None, None) => (),
        }
    }
    return calibration_value;
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
