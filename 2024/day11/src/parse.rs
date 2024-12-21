use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn parse_stones(filename: &str) -> Vec<i128> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    let line = reader.lines().next();
    let line = line.unwrap().expect("Error parsing line");

    let result: Vec<i128> = line.split(' ')
        .filter_map(|s| s.parse::<i128>().ok())
        .collect();

    result
}


#[cfg(test)]
mod tests {
    use crate::constants::{FILENAME, TEST_INPUT_FILENAME};

    use super::*;

    #[test]
    fn test_parse_stones_simple() {
        let stones = parse_stones(TEST_INPUT_FILENAME);
        assert_eq!(
            stones,
            vec![125, 17]
        );
    }

    #[test]
    fn test_parse_stones() {
        let stones = parse_stones(FILENAME);
        assert_eq!(
            stones,
            vec![1950139, 0, 3, 837, 6116, 18472, 228700, 45]
        )
    }
}