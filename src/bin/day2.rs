use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("./day2_data.txt")?;

    let mut acc = 0;
    for line in input.lines() {
        if is_safe(line.to_string()) {
            acc += 1;
        }
    }
    println!("Safe count: {}", acc);
    Ok(())
}

fn is_safe(s: String) -> bool {
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
    for level in s.iter() {
        if previous != 0 {
            if op(*level, previous) {
                if ((*level as i8) - (previous as i8)).abs() > 3 {
                    return false;
                }
                previous = *level;
                continue;
            } else {
                return false;
            }
        } else {
            previous = *level;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::is_safe;

    #[test]
    fn test_valid_decreasing() {
        assert!(is_safe("7 6 4 2 1".to_string()));
    }

    #[test]
    fn test_increasing_too_large_step() {
        assert!(!is_safe("1 2 7 8 9".to_string()));
    }

    #[test]
    fn test_decreasing_too_large_step() {
        assert!(!is_safe("9 7 6 2 1".to_string()));
    }

    #[test]
    fn test_not_strictly_increasing() {
        assert!(!is_safe("1 3 2 4 5".to_string()));
    }

    #[test]
    fn test_decreasing_too_small_step() {
        assert!(!is_safe("8 6 4 4 1".to_string()));
    }

    #[test]
    fn test_valid_increasing() {
        assert!(is_safe("1 3 6 7 9".to_string()));
    }
}
