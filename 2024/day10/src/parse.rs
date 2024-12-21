use std::fs::File;
use std::io::{BufRead, BufReader};



pub fn parse_file_into_grid(filename: &str) -> Vec<Vec<i32>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();

        let nums: Vec<i32> = chars
            .iter()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        grid.push(nums)

    }

    grid
}


#[cfg(test)]
mod tests {
    use crate::constants::TEST_INPUT_FILENAME;

    use super::*;

    #[test]
    fn test_parse_file_into_grid() {
        let grid = parse_file_into_grid(TEST_INPUT_FILENAME);
        assert_eq!(grid, vec![
            vec![8,9,0,1,0,1,2,3],
            vec![7,8,1,2,1,8,7,4],
            vec![8,7,4,3,0,9,6,5],
            vec![9,6,5,4,9,8,7,4],
            vec![4,5,6,7,8,9,0,3],
            vec![3,2,0,1,9,0,1,2],
            vec![0,1,3,2,9,8,0,1],
            vec![1,0,4,5,6,7,3,2],
        ]);
    }
}