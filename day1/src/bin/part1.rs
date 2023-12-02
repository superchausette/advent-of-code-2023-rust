fn main() {
    let input = include_str!("part1.txt");
    let out = part1(input);
    dbg!(out);
}

fn part1(input: &str) -> String {
    return "TODO".to_string();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        let expected_result = "OrNotTODO";
        let result = part1("");
        assert_eq!(result, expected_result);
    }
}