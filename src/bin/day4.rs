use regex::Regex;

fn main() {
    unimplemented!()
}

fn get_xes_location(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new("X").unwrap();
    let mut result: Vec<(usize, usize)> = Vec::new();

    for (r, line) in input.lines().enumerate() {
        println!("{}", line);
        for m in re.find_iter(line) {
            let c = m.start();
            result.push((r, c));
        }
    }
    result
}

// fn get_possible_sequences(input: &str, xes_loc: Vec<(usize, usize)>) -> Vec<&str> {
//     for x_loc in xes_loc {
//         input
//     }
//     vec!["broken"]
// }

#[cfg(test)]
mod tests {
    use crate::get_xes_location;

    #[test]
    fn test_simple() {
        let input = "..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X....";
        assert_eq!(
            get_xes_location(input),
            vec![(0, 2), (1, 4), (3, 0), (4, 1)]
        );
    }

    // #[test]
    // fn test_get_possible_sequence() {
    //     let input = "..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X....";
    //     let xes_loc = vec![(0, 0)];
    //     assert_eq!(get_possible_sequences(input, xes_loc), vec!["test"]);
    // }
}
