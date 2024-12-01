use std::{collections::HashMap, fs};

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

    // print result for part 1
    let result = get_total_diff(&mut first, &mut second);
    println!("The result of the first part is: {:?}", result);

    // print result for part 2
    let result = get_similarity(&mut first, &mut second);
    println!("The result of the second part is: {:?}", result);
    Ok(())
}

fn get_total_diff(a: &mut [i64], b: &mut [i64]) -> i64 {
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

fn get_similarity(a: &mut [i64], b: &mut [i64]) -> i64 {
    // keep track of how often each value appears in vector b
    let mut counter = HashMap::new();
    for key in b.iter() {
        match counter.get(&key) {
            Some(value) => counter.insert(key, value + 1),
            None => counter.insert(key, 1),
        };
    }

    // compute "similarity" for each element of vector a
    let mut acc: i64 = 0;
    for key in a.iter() {
        let multiplier = counter.get(key).unwrap_or(&0_i64);
        acc += key * multiplier;
    }
    acc
}

#[cfg(test)]
mod test {
    use crate::{get_similarity, get_total_diff};

    #[test]
    fn test_part1() {
        let mut a = vec![10, 1, 2];
        let mut b = vec![10, 2, 0];
        let result = get_total_diff(&mut a, &mut b);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part2() {
        let mut a = vec![10, 1, 2];
        let mut b = vec![10, 2, 10];
        let result = get_similarity(&mut a, &mut b);
        assert_eq!(result, 22);
    }
}
