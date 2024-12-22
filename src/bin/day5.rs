use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./day5_data.txt").unwrap();
    let result = solve_puzzle(input.as_str());
    println!("Solution part 1: {:?}", result);
    println!("Solution part 2: {:?}", 0);
}

fn solve_puzzle(input: &str) -> u64 {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let rules = process_rules(sections[0]);
    process_lists(sections[1], rules)
}

fn process_rules(input: &str) -> HashMap<u64, Vec<u64>> {
    let mut m = HashMap::<u64, Vec<u64>>::new();
    for line in input.lines() {
        process_rule(&mut m, line);
    }
    m
}

fn process_rule(m: &mut HashMap<u64, Vec<u64>>, input: &str) {
    let r: Vec<u64> = input
        .split("|")
        .map(|x| x.trim().parse::<u64>().unwrap())
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

fn process_lists(input: &str, rules: HashMap<u64, Vec<u64>>) -> u64 {
    let mut result = 0;
    for line in input.lines() {
        let line = line
            .split(",")
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect();
        result += process_list_part1(line, &rules);
    }
    result
}

fn process_list_part1(line: Vec<u64>, rules: &HashMap<u64, Vec<u64>>) -> u64 {
    if is_correct_order(&line, rules) {
        line[line.len() / 2]
    } else {
        0
    }
}

fn is_correct_order(line: &[u64], rules: &HashMap<u64, Vec<u64>>) -> bool {
    let mut prev = 0;
    for (i, elem) in line.iter().enumerate() {
        if i == 0 {
            prev = *elem
        } else {
            let valid = match rules.get(elem) {
                Some(value) => value,
                None => return false,
            };
            if valid.contains(&prev) {
                prev = *elem;
                continue;
            } else {
                return false;
            }
        }
    }
    true
}

fn sort_sequence(sequence: &str, map: &HashMap<u64, Vec<u64>>) -> Vec<u64> {
    let mut sequence: Vec<u64> = sequence
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    for i in 1..sequence.len() {
        let curr = sequence[i];
        let prev = sequence[i - 1];
        match map.get(&curr) {
            Some(v) => {
                if v.contains(&prev) {
                    println!("Found");
                    continue;
                } else {
                    println!("Not Found");
                    let element = sequence.remove(i);
                    for e in (0..i).rev() {
                        println!("{:?}", e);
                        if v.contains(&sequence[e]) {
                            sequence.insert(e + 1, element);
                            continue;
                        }
                    }
                }
            }
            None => return vec![9999],
        }
    }
    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_one_rule() {
        let input = "47|53";
        let rules = process_rules(input);
        let result = rules.get(&53).unwrap();
        assert_eq!(*result, vec![47]);
    }

    #[test]
    fn test_multiple_rules() {
        let input = "47|53
97|53
75|29";
        let result = process_rules(input);
        let result = result.get(&53).unwrap();
        assert_eq!(*result, vec![47, 97]);
    }

    #[test]
    fn test_simple_complete_case() {
        let input = "13|75
        97|13

        97,13,75";
        assert_eq!(solve_puzzle(input), 13);
    }

    #[test]
    fn test_incorrect_list() {
        let input = "13|75
        97|13

        97,75,13";
        assert_eq!(solve_puzzle(input), 0);
    }

    #[test]
    fn test_basics() {
        let input = "47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47";
        assert_eq!(solve_puzzle(input), 143);
    }

    #[test]
    fn test_part2_simple() {
        // given
        let rules_str = "61|13
        29|13
        61|29 ";
        let rules = process_rules(rules_str);
        let sequence = "61,13,29";

        // when
        let result = sort_sequence(sequence, &rules);

        // then
        let expected = vec![61, 29, 13];
        assert_eq!(result, expected);
    }
}
