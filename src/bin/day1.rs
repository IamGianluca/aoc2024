use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read input
    let input = fs::read_to_string("./day1_data.txt")?;

    let mut first: Vec<i64> = Vec::new();
    let mut second: Vec<i64> = Vec::new();

    for line in input.lines() {
        let r: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let (a, b) = (r[0], r[1]);
        first.push(a);
        second.push(b);
    }
    let result = total_diff(&mut first, &mut second);
    println!("The result of the first part is: {:?}", result);
    Ok(())
}

fn total_diff(a: &mut [i64], b: &mut [i64]) -> i64 {
    // sort vectors
    a.sort();
    b.sort();

    // for each pair, compute difference
    let mut diffs = Vec::new();
    for (a, b) in a.iter().zip(b.iter()) {
        let d = (b - a).abs();
        diffs.push(d);
    }

    // sum differences
    let result: i64 = diffs.iter().sum();
    result
}

#[cfg(test)]
mod test {
    use crate::total_diff;

    #[test]
    fn test_simple() {
        let mut a = vec![10, 1, 2];
        let mut b = vec![10, 2, 0];
        let result = total_diff(&mut a, &mut b);
        assert_eq!(result, 1);
    }
}
