use std::u64;

fn main() {
    todo!();
}

fn solve_part1(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let grid_len = grid.len();
    let grid_hei = grid[0].len();

    for (r, row) in grid.iter().enumerate() {
        // find starting point or move to next row
        let c = match row.iter().position(|x| *x == '^') {
            Some(c) => c,
            None => continue,
        };
        println!("Found starting point at row {r} and column {c}");

        // move until we find an obstacle or reach the end of the board
        let trajectory = &grid[r..0][c];
        if trajectory.contains(&'#') {
            // obstacle was reached
            let c = match trajectory.iter().position(|x| *x == '#') {
                Some(c) => c,
                None => continue,
            };
        } else {
            // end of board reached
            return 200;
        };
    }
    0
}

fn find_starting_point(grid: Vec<Vec<char>>) -> (usize, usize) {
    for (r, row) in grid.iter().enumerate() {
        // find starting point or move to next row
        let c = match row.iter().position(|x| *x == '^') {
            Some(c) => c,
            None => continue,
        };
        println!("Found starting point at row {r} and column {c}");
        return (r, c);
    }
    (99999, 99999)
}

#[cfg(test)]
mod test {
    use crate::{find_starting_point, solve_part1};

    #[test]
    fn test_find_starting_point() {
        // given
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

        // when
        let result = find_starting_point(grid);

        // then
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
