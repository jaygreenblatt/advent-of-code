use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input.txt";


pub fn parse_file() -> Vec<Vec<char>> {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        map.push(chars);
    }

    map
}