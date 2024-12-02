use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("./day2_data.txt")?;

    let mut acc = 0;
    for line in input.lines() {
        if is_safe(line.to_string(), 0) {
            acc += 1;
        }
    }
    println!("Safe count: {}", acc);

    let mut acc = 0;
    for line in input.lines() {
        if is_safe(line.to_string(), 1) {
            acc += 1;
        }
    }
    println!("Safe count (with tolerance): {}", acc);
    Ok(())
}

fn is_safe(s: String, tol: u8) -> bool {
    let s: Vec<u8> = s
        .split_whitespace()
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    let op = if *s.first().unwrap() > *s.last().unwrap() {
        |x: u8, y: u8| x < y
    } else {
        |x: u8, y: u8| x > y
    };

    let mut previous = 0;
    let mut errors = 0;
    for (i, level) in s.iter().enumerate() {
        if i == 0 {
            previous = *level;
            continue;
        }
        if op(*level, previous) {
            if ((*level as i8) - (previous as i8)).abs() > 3 {
                errors += 1;
            } else {
                previous = *level;
            };
        } else {
            errors += 1;
        };
    }
    if errors > tol {
        return false;
    };
    true
}

#[cfg(test)]
mod test {
    use crate::is_safe;

    #[test]
    fn test_valid_decreasing() {
        assert!(is_safe("7 6 4 2 1".to_string(), 0));
        assert!(is_safe("7 6 4 2 1".to_string(), 1));
    }

    #[test]
    fn test_increasing_too_large_step() {
        assert!(!is_safe("1 2 7 8 9".to_string(), 0));
        assert!(!is_safe("1 2 7 8 9".to_string(), 1));
    }

    #[test]
    fn test_decreasing_too_large_step() {
        assert!(!is_safe("9 7 6 2 1".to_string(), 0));
        assert!(!is_safe("9 7 6 2 1".to_string(), 1));
    }

    #[test]
    fn test_not_strictly_increasing() {
        assert!(!is_safe("1 3 2 4 5".to_string(), 0));
        assert!(is_safe("1 3 2 4 5".to_string(), 1));
    }

    #[test]
    fn test_decreasing_too_small_step() {
        assert!(!is_safe("8 6 4 4 1".to_string(), 0));
        assert!(is_safe("8 6 4 4 1".to_string(), 1));
    }

    #[test]
    fn test_valid_increasing() {
        assert!(is_safe("1 3 6 7 9".to_string(), 0));
        assert!(is_safe("1 3 6 7 9".to_string(), 1));
    }

    #[test]
    fn test_remove_first() {
        assert!(!is_safe("7 3 4 5 6".to_string(), 0));
        assert!(is_safe("7 3 4 5 6".to_string(), 1));
    }

    #[test]
    fn test_remove_last() {
        assert!(!is_safe("2 3 4 5 1".to_string(), 0));
        assert!(is_safe("2 3 4 5 1".to_string(), 1));
    }
}
