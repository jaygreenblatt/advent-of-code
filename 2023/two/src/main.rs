/**
 * Advent of Code 2023 Day 2 solution.
 */

 use std::fs::File;
 use std::io::{self, BufRead};
 use std::path::Path;
 use std::collections::HashMap;

 use regex::Regex;

fn main() {
    let filename: &str = "./input.txt";
    let mut sum: u32 = 0;

    let colors = ["red", "blue", "green"];    
    let mut maxes: HashMap<&str, u32> = HashMap::new();
    let mut regexes: HashMap<&str, Regex> = HashMap::new();

    // Insert maximums into the map.
    maxes.insert("red", 12);
    maxes.insert("blue", 14);
    maxes.insert("green", 13);

    // Insert regexes into the map.
    regexes.insert("red", Regex::new(r"\d+ red").unwrap());
    regexes.insert("blue", Regex::new(r"\d+ blue").unwrap());
    regexes.insert("green", Regex::new(r"\d+ green").unwrap());

    // Read the file into lines.
    if let Ok(lines) = read_lines(filename) {
        let mut i = 1;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let l: String = line.expect("Not a string!");
            let mut is_possible_game = true;
            for color in colors {
                let reg: Regex = regexes.get(color).expect("No value found").clone();
                let max_allowed: u32 = maxes.get(color).expect("No value found").clone();

                let matches: Vec<_> = reg.find_iter(l.as_str()).map(|m| m.as_str()).collect();
                for m in matches {
                    let num: u32 = m.split(" ").collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
                    if num > max_allowed {
                        is_possible_game = false;
                    }
                }
            }

            if is_possible_game {
                sum += i;
            }
            i += 1;
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