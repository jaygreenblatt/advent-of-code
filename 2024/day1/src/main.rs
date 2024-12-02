use std::fs::File;
use std::io::{self, BufRead, BufReader};

const FILENAME: &str = "src/input.txt";

fn main() -> io::Result<()> {
    puzzle_one()?;

    Ok(())
}

fn puzzle_one() -> io::Result<()> {
    let (x_coords, y_coords) = parse_location_ids_from_file(FILENAME)?;
    let distance = calculate_total_distance(x_coords, y_coords);

    println!("Puzzle One Solution");
    println!("The total distance is: {}", distance);

    Ok(())
}

fn calculate_total_distance(mut x_values: Vec<i32>, mut y_values: Vec<i32>) -> i32 {
    // Sort both arrays
    x_values.sort();
    y_values.sort();

    // Calculate the sum of absolute differences
    let sum = x_values.iter()
        .zip(y_values.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();

    sum
}


fn parse_location_ids_from_file(filename: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut x_values = Vec::new();
    let mut y_values = Vec::new();

    for line in reader.lines() {
        let line = line?;
        // Split the line by whitespace and collect parts
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            // Parse each part as an integer
            match (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                (Ok(x), Ok(y)) => {
                    x_values.push(x);
                    y_values.push(y);
                }
                _ => {
                    eprintln!("Warning: Skipping invalid line: {}", line);
                }
            }
        } else {
            eprintln!("Warning: Skipping malformed line: {}", line);
        }
    }

    Ok((x_values, y_values))
}