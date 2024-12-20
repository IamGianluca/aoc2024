use std::collections::HashMap;

fn main() {
    unimplemented!()
}

fn solve_p1(input: &str) -> u64 {
    // page ordering rules
    for line in input.lines() {
        if line.is_empty() {
            break;
        } else {
            println!("{}", line);
        }
    }
    // process update sequence
    process_all_rules(input);
    // return
    16
}

fn process_all_rules(input: &str) -> HashMap<u64, Vec<u64>> {
    let mut m = HashMap::<u64, Vec<u64>>::new();
    for line in input.lines() {
        process_rule(&mut m, line);
    }
    m
}

fn process_rule(m: &mut HashMap<u64, Vec<u64>>, input: &str) {
    let r: Vec<u64> = input
        .split("|")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let first = r.first().unwrap();
    let last = r.get(1).unwrap();

    match m.get(last) {
        Some(v) => {
            let mut new_value = v.clone();
            new_value.push(*first);
            m.insert(*last, new_value)
        }
        None => m.insert(*last, vec![*first]),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_one_rule() {
        let input = "47|53";
        let mut map = HashMap::<u64, Vec<u64>>::new();
        process_rule(&mut map, input);
        let result = map.get(&53).unwrap();
        assert_eq!(*result, vec![47]);
    }

    #[test]
    fn test_multiple_rules() {
        let input = "47|53
97|53
75|29";
        let result = process_all_rules(input);
        println!("Result: {:?}", result);
        let result = result.get(&53).unwrap();
        assert_eq!(*result, vec![47, 97]);
    }

    //     #[test]
    //     fn test_basics() {
    //         let input = "47|53
    //     97|13
    //     97|61
    //     97|47
    //     75|29
    //     61|13
    //     75|53
    //     29|13
    //     97|29
    //     53|29
    //     61|53
    //     97|53
    //     61|29
    //     47|13
    //     75|47
    //     97|75
    //     47|61
    //     75|61
    //     47|29
    //     75|13
    //     53|13
    //
    //     75,47,61,53,29
    //     97,61,53,29,13
    //     75,29,13
    //     75,97,47,61,53
    //     61,13,29
    //     97,13,75,29,47";
    //         assert_eq!(solve_p1(input), 1);
    //     }
}
