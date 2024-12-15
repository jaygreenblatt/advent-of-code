use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input.txt";

pub fn parse_file() -> Vec<Vec<char>> {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);
    let mut result: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        result.push(line.chars().collect::<Vec<char>>());
    }

    result
}