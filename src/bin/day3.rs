use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./day3_data.txt").unwrap();

    let mut result = 0;
    for instruction in input.lines() {
        result += compute(instruction);
    }
    println!("Part 1 result: {:?}", result);

    let mut result = 0;
    for instruction in input.lines() {
        result += compute_does_and_donts(instruction);
    }
    println!("Part 2 result: {:?}", result);
}

fn compute_does_and_donts(instruction: &str) -> u64 {
    println!("{:?}", instruction);
    let re = Regex::new(r"don't\(\).*?(?:do\(\)|$)").unwrap();
    let new_instruction = re.replace_all(instruction, "").to_string();
    println!("{:?}", new_instruction);
    compute(&new_instruction)
}

fn compute(instruction: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result: u64 = 0;
    for (_, [a, b]) in re.captures_iter(instruction).map(|c| c.extract()) {
        result += (a.parse::<u64>().unwrap()) * (b.parse::<u64>().unwrap());
    }
    result
}

#[cfg(test)]
mod test {
    use crate::{compute, compute_does_and_donts};

    #[test]
    fn test_something() {
        let instruction = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(compute(instruction), 161);
    }

    #[test]
    fn test_does_and_donts() {
        let instruction =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(compute_does_and_donts(instruction), 48);
    }

    #[test]
    fn test_does_and_donts_until_end_of_string() {
        let instruction = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)()?mul(8,5))";
        assert_eq!(compute_does_and_donts(instruction), 8);
    }

    #[test]
    fn test_does_and_donts_not_reenabled() {
        let instruction = "xmul(2,4)&mul[3,7]!^dn't()_mul(5,5)+mul(32,64](mul(11,8)und()?mul(8,5))";
        assert_eq!(compute_does_and_donts(instruction), 161);
    }

    #[test]
    fn test_does_and_donts_nothing() {
        let instruction =
            "don't()xmul(2,4)&mul[3,7]!^dn't()_mul(5,5)+mul(32,64](mul(11,8)und()?mul(8,5))";
        assert_eq!(compute_does_and_donts(instruction), 0);
    }

    #[test]
    fn test_does_and_donts_overlapping() {
        let instruction = "don't() mul(1,1) do() mul(2,2) don't() mul(3,3) do() mul(4,4)";
        assert_eq!(compute_does_and_donts(instruction), 20);
    }

    #[test]
    fn test_nested_donts() {
        let instruction = "don't() mul(1,1) don't() mul(2,2) do() mul(3,3)";
        assert_eq!(compute_does_and_donts(instruction), 9);
    }
    #[test]
    fn test_order_counts() {
        let instruction = "don't() mul(1,1) don't() mul(2,2) do() mul(3,3)";
        assert_eq!(compute_does_and_donts(instruction), 9);
    }

    #[test]
    fn test_multiple() {
        let instruction = "mul(1,1) don't() do() don't() mul(2,2)";
        assert_eq!(compute_does_and_donts(instruction), 1);
    }

    #[test]
    fn test_multiple_even_more() {
        let instruction = "mul(1,1) don't() do() don't() do() don't() mul(2,2)";
        assert_eq!(compute_does_and_donts(instruction), 1);
    }

    #[test]
    fn test_ug() {
        let instruction =
            "mul(1,1) don't() mul(2,2) don't() mul(3,3) do() don't() do() don't() mul(2,2)";
        assert_eq!(compute_does_and_donts(instruction), 1);
    }

    #[test]
    fn test_most_complex_case() {
        let instruction =
            "mul(1,1) do() mul(2,2) don't() mul(3,3) do() mul(4,4) don't() don't() mul(5,5) do() mul(6,6) don't() mul(7,7) do() mul(8,8) don't() mul(9,9)";
        assert_eq!(compute_does_and_donts(instruction), 121);
    }
}
