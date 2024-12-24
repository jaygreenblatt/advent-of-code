use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn parse_grid(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut result: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        result.push(line.chars().collect());
    }

    result
}


#[cfg(test)]
mod tests {
    use crate::constants::TEST_INPUT_FILENAME;
    use super::*;

    #[test]
    fn test_parse_grid() {
        let grid = parse_grid(TEST_INPUT_FILENAME);
        let expected_result = vec![
            vec!['R','R','R','R','I','I','C','C','F','F'],
            vec!['R','R','R','R','I','I','C','C','C','F'],
            vec!['V','V','R','R','R','C','C','F','F','F'],
            vec!['V','V','R','C','C','C','J','F','F','F'],
            vec!['V','V','V','V','C','J','J','C','F','E'],
            vec!['V','V','I','V','C','C','J','J','E','E'],
            vec!['V','V','I','I','I','C','J','J','E','E'],
            vec!['M','I','I','I','I','I','J','J','E','E'],
            vec!['M','I','I','I','S','I','J','E','E','E'],
            vec!['M','M','M','I','S','S','J','E','E','E'],
        ];

        assert_eq!(grid, expected_result);
    }
}