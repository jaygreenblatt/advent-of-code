use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::error::Error;

const FILENAME: &str = "src/input.txt";

fn main() -> io::Result<()> {
    puzzle_one()?;

    Ok(())
}

fn puzzle_one() -> io::Result<()> {
    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);

    // Create regex pattern for mul(X,Y) where X and Y are 1-3 digits
    let pattern = r"mul\(\d{1,3},\d{1,3}\)";
    let regex = Regex::new(pattern).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut result = 0;

    for line in reader.lines() {
        let line = line?;
        let matches: Vec<&str> = regex.find_iter(&line)
            .map(|m| m.as_str())
            .collect();

        for (i, match_str) in matches.iter().enumerate() {
            let spliced = &match_str[4..&match_str.len() - 1];
            let comma_split = spliced.split_once(",").unwrap();
            let num1 = comma_split.0.parse::<i32>().unwrap();
            let num2 = comma_split.1.parse::<i32>().unwrap();
            result += num1 * num2;
        }
    }

    println!("Puzzle One Solution");
    println!("The result is {}", result);

    Ok(())
}