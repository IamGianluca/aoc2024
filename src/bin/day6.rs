use std::fs;

fn main() {
    let input = fs::read_to_string("./day6_data.txt").unwrap();

    let result_p1 = solve_part1(input.as_str());
    println!("Result part 1: {:?}", result_p1);

    // TODO: Result for part 2 is incorrect and the program is extremely slow. Create an unit test
    // to capture the exact behavior we are currently not supporting correctly and address it.
    // let result_p2 = solve_part2(input.as_str());
    // println!("Result part 2: {:?}", result_p2);
}

fn solve_part1(input: &str) -> u64 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut direction_index = 0;
    let mut current_position = get_starting_position(&grid);

    // Move until we reach the end of the board. If there is something directly in front of
    // you, turn right 90 degrees. Otherwise, take a step forward.
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
        let candidate = match grid.get(candidate_position.0) {
            Some(v) => v,
            None => return count_visited(&grid),
        };
        let candidate = match candidate.get(candidate_position.1) {
            Some(v) => v,
            None => return count_visited(&grid),
        };
        match candidate {
            '#' => {
                // Skip and turn right next time
                direction_index += 1;
            }
            '.' | 'x' | '^' => {
                // Valid landing position
                current_position = candidate_position;
                grid[current_position.0][current_position.1] = 'x';
                continue;
            }
            _ => continue_game = false,
        };
    }
    0
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

fn count_visited(grid: &Vec<Vec<char>>) -> u64 {
    let mut result = 0;
    for r in grid.iter() {
        for c in r.iter() {
            if *c == 'x' || *c == '^' {
                result += 1;
            }
        }
    }
    result
}

fn solve_part2(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_pos = get_starting_position(&grid);
    let mut result = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '#' || (r, c) == start_pos {
                continue;
            }
            let mut tmp_grid = grid.clone();
            tmp_grid[r][c] = '#';
            if creates_loop(&mut tmp_grid, start_pos) {
                result += 1;
            }
        }
    }
    result
}

fn creates_loop(grid: &mut Vec<Vec<char>>, start_pos: (usize, usize)) -> bool {
    const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut visited_pos = vec![];
    let mut direction_index = 0;
    let mut current_position = start_pos;

    // TODO: passing by the same position multiple times is not necessarily a sign we are in an
    // infinite loop. Maybe is making the same turn?

    // Move until we reach the end of the board. If there is something directly in front of
    // you, turn right 90 degrees. Otherwise, take a step forward.
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
        let candidate = match grid.get(candidate_position.0) {
            Some(v) => v,
            None => return false,
        };
        let candidate = match candidate.get(candidate_position.1) {
            Some(v) => v,
            None => return false,
        };
        match candidate {
            '#' => {
                // Skip and turn right next time
                direction_index += 1;
            }
            '.' | 'x' | '^' => {
                // Valid landing position
                current_position = candidate_position;
                let count = visited_pos
                    .iter()
                    .filter(|&&x| x == current_position)
                    .count();
                if count >= 3 {
                    println!("Found another match");
                    for line in grid.iter() {
                        println!("{:?}", line);
                    }
                    return true;
                } else {
                    visited_pos.push(current_position);
                    grid[current_position.0][current_position.1] = 'x';
                    continue;
                };
            }
            _ => continue_game = false,
        };
    }
    false
}

#[cfg(test)]
mod test {
    use crate::{count_visited, get_starting_position, solve_part1, solve_part2};

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

        // When
        let result = solve_part1(input);

        // Then
        assert_eq!(result, 41);
    }

    #[test]
    fn test_count_visited() {
        // Given
        let grid = "....x.....
....x....#
....x.....
..#.x.....
....x..#..
....x.....
.#..^.....
........#.
#.........
......#...";
        let grid = grid.lines().map(|line| line.chars().collect()).collect();

        // When
        let result = count_visited(&grid);

        // Then
        assert_eq!(result, 7)
    }

    #[test]
    fn test_simple_part2() {
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

        // When
        let result = solve_part2(input);

        // Then
        assert_eq!(result, 6);
    }

    #[test]
    fn test_add_one_obstacle_make_infinite_loop() {
        // Given
        let input = ".#..
...#
.^..
..#.";

        // When
        let result = solve_part2(input);

        // Then
        assert_eq!(result, 1);
    }

    #[test]
    fn test_bouncing_on_obstacle_not_previously_visited_creates_infinite_loop() {
        // Given
        let input = ".#....
..#...
.....#
......
..^...
.#..#.";

        // When
        let result = solve_part2(input);

        // Then
        assert_eq!(result, 2);
    }
}
