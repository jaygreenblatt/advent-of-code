/**
 * Advent of Code 2023 Day 1 solution.
 */

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename: &str = "./input.txt";
    let mut sum: u32 = 0;

    // Read the file into lines.
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            // Parse the digits into a vector.
            let mut vec = Vec::new();
            for c in line.expect("Invalid line.").chars() {
                let digit: u32 = match c.to_digit(10) {
                    Some(num) => num,
                    None => continue
                };

                vec.push(digit);
            }

            // We now should add the appropriate amount to the total sum.
            sum += vec[0] * 10 + vec[vec.len() - 1];
        }
    }  

    println!("The sum is {}.", sum);
}


// Taken from the documentation.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}