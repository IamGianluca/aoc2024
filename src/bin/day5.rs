use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./day5_data.txt").unwrap();
    let result = solve_puzzle(input.as_str(), 1);
    println!("Solution part 1: {:?}", result);

    let result = solve_puzzle(input.as_str(), 2);
    println!("Solution part 2: {:?}", result);
}

fn solve_puzzle(input: &str, part: u64) -> u64 {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let rules = process_rules(sections[0]);
    process_lists(sections[1], rules, part)
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

fn process_lists(input: &str, rules: HashMap<u64, Vec<u64>>, part: u64) -> u64 {
    let mut result = 0;
    let func = if part == 1 {
        process_list_part1 as fn(Vec<u64>, &HashMap<u64, Vec<u64>>) -> u64
    } else if part == 2 {
        process_list_part2 as fn(Vec<u64>, &HashMap<u64, Vec<u64>>) -> u64
    } else {
        panic!("Part {:?}", part);
    };
    for line in input.lines() {
        let line = line
            .split(",")
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect();
        result += func(line, &rules);
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

fn process_list_part2(line: Vec<u64>, rules: &HashMap<u64, Vec<u64>>) -> u64 {
    if is_correct_order(&line, rules) {
        0
    } else {
        let new_line = sort_sequence(&line, rules);
        new_line[new_line.len() / 2]
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

fn sort_sequence(sequence: &Vec<u64>, map: &HashMap<u64, Vec<u64>>) -> Vec<u64> {
    let mut result = sequence.clone();
    for i in 1..result.len() {
        let curr = result[i];
        let prev = result[i - 1];
        match map.get(&curr) {
            Some(v) => {
                if v.contains(&prev) {
                    continue;
                } else {
                    let element = result.remove(i);
                    for e in (0..i).rev() {
                        if v.contains(&result[e]) {
                            result.insert(e + 1, element);
                            break;
                        }
                    }
                    if result.len() != sequence.len() {
                        // if loop not escaped already, it means that the elem should be moved at the
                        // beginning of the sequence
                        result.insert(0, element);
                    };
                }
            }
            None => return vec![9999],
        }
    }
    if result.len() != sequence.len() {
        panic!(
            "Result and Sequence do not have the same length ({} vs. {})",
            result.len(),
            sequence.len()
        );
    };
    result
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
        assert_eq!(solve_puzzle(input, 1), 13);
    }

    #[test]
    fn test_incorrect_list() {
        let input = "13|75
        97|13

        97,75,13";
        assert_eq!(solve_puzzle(input, 1), 0);
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
        assert_eq!(solve_puzzle(input, 1), 143);
    }

    #[test]
    fn test_part2_simple() {
        // given
        let rules_str = "61|13
        29|13
        61|29 ";
        let rules = process_rules(rules_str);
        let sequence = vec![61, 13, 29];

        // when
        let result = sort_sequence(&sequence, &rules);

        // then
        let expected = vec![61, 29, 13];
        assert_eq!(result, expected);
    }
}
