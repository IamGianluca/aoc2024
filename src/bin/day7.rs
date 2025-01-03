fn main() {
    todo!();
}

fn solve_part1(input: &str) -> u64 {
    let mut result = 0;
    for equation in input.lines() {
        result += is_calibrated(equation);
    }
    result
}

fn is_calibrated(line: &str) -> u64 {
    const OPS: [fn(u64, u64) -> u64; 2] = [|x: u64, y: u64| x + y, |x: u64, y: u64| x * y];
    let parts: Vec<&str> = line.split(": ").collect();
    let number = parts[0].parse::<u64>().unwrap();
    let rest: Vec<u64> = parts[1]
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    for op in OPS {
        if number == op(rest[0], rest[1]) {
            return number;
        };
    }
    0
}

#[cfg(test)]
mod test {
    use crate::{is_calibrated, solve_part1};

    #[test]
    fn test_add_equation_two_operands() {
        let input = "190: 10 19";

        // When
        let result = is_calibrated(input);

        // Then
        assert_eq!(result, 190);
    }

    #[test]
    fn test_multiply_equation_two_operands() {
        let input = "4: 2 2";

        // When
        let result = is_calibrated(input);

        // Then
        assert_eq!(result, 4);
    }

    #[test]
    fn test_multiply_equation_three_operands() {
        let input = "191: 19 10 1";

        // When
        let result = is_calibrated(input);

        // Then
        assert_eq!(result, 191);
    }

    #[test]
    fn test_wrong_equation() {
        let input = "200: 10 19";

        // When
        let result = is_calibrated(input);

        // Then
        assert_eq!(result, 0);
    }

    #[test]
    fn test_simple_part1() {
        // Given
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        // When
        let result = solve_part1(input);

        // Then
        assert_eq!(result, 3749);
    }
}
