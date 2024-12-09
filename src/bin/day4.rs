use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./day4_data.txt").unwrap();
    let result_p1 = solve_p1(input.as_str());
    println!("Result p1: {}", result_p1);
}

fn solve_p1(input: &str) -> u64 {
    let xes_loc = get_xes_location(input);
    get_possible_sequences(input, xes_loc)
}

fn get_xes_location(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new("X").unwrap();
    let mut result: Vec<(usize, usize)> = Vec::new();

    for (r, line) in input.lines().enumerate() {
        for m in re.find_iter(line) {
            let c = m.start();
            result.push((r, c));
        }
    }
    result
}

fn get_possible_sequences(input: &str, xes_loc: Vec<(usize, usize)>) -> u64 {
    let mut counter = 0;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let x_max = grid.len() as i64;
    let y_max = grid[0].len() as i64;
    let directions: [(i64, i64); 8] = [
        (0, -1),  // left
        (0, 1),   // right
        (1, 0),   // up
        (-1, 0),  // down
        (1, -1),  // upper left diagonal
        (1, 1),   // upper right diagonal
        (-1, -1), // lower left diagonal
        (-1, 1),  // lower right diagonal
    ];
    for loc in xes_loc {
        for direction in directions {
            let mut seq = vec![];
            for offset in 0..4 {
                let new_x: i64 = (loc.0 as i64) + (direction.0 * offset);
                let new_y: i64 = (loc.1 as i64) + (direction.1 * offset);
                if new_x >= 0 && new_y >= 0 && new_x < x_max && new_y < y_max {
                    let new_char = grid[new_x as usize][new_y as usize];
                    seq.push(new_char);
                }
            }
            if String::from_iter(seq) == "XMAS" {
                counter += 1;
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use crate::{get_possible_sequences, get_xes_location};

    #[test]
    fn test_simple() {
        let input = "..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X....";
        assert_eq!(
            get_xes_location(input),
            vec![(0, 2), (1, 4), (3, 0), (4, 1)]
        );
    }

    #[test]
    fn test_get_possible_sequence() {
        let input = "..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X....";
        let xes_loc = get_xes_location(input);
        assert_eq!(get_possible_sequences(input, xes_loc), 4);
    }
}
