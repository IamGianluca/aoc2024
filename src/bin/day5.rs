use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./day5_data.txt").unwrap();
    let result = solve_p1(input.as_str());
    println!("{:?}", result)
}

fn solve_p1(input: &str) -> u64 {
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
        result += process_list(line, &rules);
    }
    result
}

fn process_list(line: Vec<u64>, rules: &HashMap<u64, Vec<u64>>) -> u64 {
    let mut prev = 0;
    for (i, elem) in line.iter().enumerate() {
        if i == 0 {
            prev = *elem
        } else {
            let valid = match rules.get(elem) {
                Some(value) => value,
                None => return 0,
            };
            if valid.contains(&prev) {
                prev = *elem;
                continue;
            } else {
                return 0;
            }
        }
    }
    line[line.len() / 2]
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
        assert_eq!(solve_p1(input), 13);
    }

    #[test]
    fn test_incorrect_list() {
        let input = "13|75
        97|13

        97,75,13";
        assert_eq!(solve_p1(input), 0);
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
        assert_eq!(solve_p1(input), 143);
    }
}
