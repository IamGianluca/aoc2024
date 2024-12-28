fn main() {
    todo!();
}

#[allow(dead_code)]
fn solve_part1(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut direction_index = 0;
    for line in grid.iter() {
        println!("{:?}", line);
    }
    let mut current_position = get_starting_position(&grid);
    println!(
        "Starting position: ({:?}, {:?})",
        current_position.0, current_position.1
    );

    // Move until we reach the end of the board. If there is something directly in front of
    // you, turn right 90 degrees. Otherwise, take a step forward.
    let mut steps_count = 0;

    let mut continue_game = true;
    while continue_game {
        // Propose next step
        let direction = DIRECTIONS[direction_index % 4];
        let candidate_position = (
            (current_position.0 as i64 + direction.0) as usize,
            (current_position.1 as i64 + direction.1) as usize,
        );

        // If next step contains an obstacle, turn right. If next step is out of the grid,
        // the game is over.
        // Looks a good candidate to use pattern matching!
        // Remember to increase steps_count if we moved.
        // And don't forget to update continue_game to false if the game is over.
        let candidate = match grid.get(candidate_position.0) {
            Some(v) => v,
            None => return steps_count,
        };
        let candidate = match candidate.get(candidate_position.1) {
            Some(v) => v,
            None => return steps_count,
        };
        println!("Step: {:?}, Value: {:?}", steps_count, candidate);
        match candidate {
            '#' => {
                // Skip and turn right next time
                direction_index += 1;
            }
            '.' => {
                // Valid landing spot
                current_position = candidate_position;
                steps_count += 1;
                continue;
            }
            '^' => {
                // Valid landing spot
                current_position = candidate_position;
                steps_count += 1;
                continue;
            }
            _ => continue_game = false,
        };
    }
    steps_count
}

fn get_starting_position(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (r, row) in grid.iter().enumerate() {
        // Find starting point or move to next row
        let c = match row.iter().position(|x| *x == '^') {
            Some(c) => c,
            None => continue,
        };
        return (r, c);
    }
    (99999, 99999)
}

#[cfg(test)]
mod test {
    use crate::{get_starting_position, solve_part1};

    #[test]
    fn test_find_starting_point() {
        // Given
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        // When
        let result = get_starting_position(&grid);

        // Then
        let expected = (6, 4);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_simple_case_part1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let result = solve_part1(input);
        assert_eq!(result, 41);
    }
}
