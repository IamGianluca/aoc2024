use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./day3_data.txt").unwrap();

    let mut result = 0;
    for instruction in input.lines() {
        result += compute(instruction);
    }
    println!("Part 1 result: {:?}", result);
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
    use crate::compute;

    #[test]
    fn test_something() {
        let instruction = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(compute(instruction), 161);
    }
}
