use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input.txt";


pub fn parse_file() -> Vec<(i64, Vec<i64>)> {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);
    let mut result: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let split = line.split_once(":").unwrap();
        let calibration = split.0.parse::<i64>().unwrap();
        let numbers: Vec<i64> = split.1
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        result.push((calibration, numbers));
    }

    result
}