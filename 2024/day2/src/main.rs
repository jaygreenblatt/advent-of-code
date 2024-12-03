use std::fs::File;
use std::io::{self, BufRead, BufReader};

const FILENAME: &str = "src/input.txt";

fn main() -> io::Result<()> {
    puzzle_one()?;

    Ok(())
}

fn puzzle_one() -> io::Result<()> {
    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);
    let mut num_safe_reports = 0;

    for line in reader.lines() {
        let line = line?;
        let report = parse_line(&line).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        if is_safe(report) {
            num_safe_reports += 1;
        }
    }

    println!("Puzzle One Solution:");
    println!("The number of safe reports is {}", num_safe_reports);

    Ok(())
}


fn parse_line(line: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    line.split_whitespace()
        .map(|num| num.parse::<i32>())
        .collect()
}

fn is_safe(report: Vec<i32>) -> bool {
    if report[0] == report[1] {
        return false;
    }
    let is_increasing = report[0] < report[1];

    for i in 0..report.len() - 1 {
        if report[i] == report[i + 1] {
            return false;
        }
        if is_increasing && (report[i] >= report[i + 1] || report[i + 1] - report[i] > 3) {
            return false;
        }
        else if !is_increasing && (report[i] <= report[i + 1] || report[i] - report[i + 1] > 3) {
            return false;
        }
    }
    return true;
}