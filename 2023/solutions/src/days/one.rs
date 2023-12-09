/**
 * Advent of Code 2023 Day 1 solution.
 */
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::reader;

pub fn solution() {
    let filename: &str = "./inputs/one.txt";
    let mut sum: u32 = 0;

    // Read the file into lines.
    if let Ok(lines) = reader::read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            // Parse the digits into a vector.
            let mut vec = Vec::new();
            for c in line.expect("Invalid line.").chars() {
                let digit: u32 = match c.to_digit(10) {
                    Some(num) => num,
                    None => continue,
                };

                vec.push(digit);
            }

            // We now should add the appropriate amount to the total sum.
            sum += vec[0] * 10 + vec[vec.len() - 1];
        }
    }

    println!("[1] The sum is {}.", sum);
}
