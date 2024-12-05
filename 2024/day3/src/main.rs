use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const FILENAME: &str = "src/input.txt";
const MUL_PATTERN: &str = r"mul\(\d{1,3},\d{1,3}\)";

fn main() -> io::Result<()> {
    puzzle_one()?;
    puzzle_two()?;

    Ok(())
}

fn puzzle_one() -> io::Result<()> {
    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);

    // Create regex pattern for mul(X,Y) where X and Y are 1-3 digits
    let regex = Regex::new(MUL_PATTERN).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut result = 0;

    for line in reader.lines() {
        let line = line?;
        let matches: Vec<&str> = regex.find_iter(&line)
            .map(|m| m.as_str())
            .collect();

        for (i, match_str) in matches.iter().enumerate() {
            result += parse_product(match_str)
        }
    }

    println!("Puzzle One Solution");
    println!("The result is {}", result);

    Ok(())
}

fn parse_product(mul: &str) -> i32 {
    let spliced = &mul[4..&mul.len() - 1];
    let comma_split = spliced.split_once(",").unwrap();
    let num1 = comma_split.0.parse::<i32>().unwrap();
    let num2 = comma_split.1.parse::<i32>().unwrap();
    return num1 * num2;
}


fn puzzle_two() -> io::Result<()> {
    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);
    let mut total = 0;

    let mut is_enabled = true;
    
    for line in reader.lines() {
        let line = line?;
        let mut i = 0 as usize;
        while i < line.chars().count() {
            // Handle do()
            let do_result = match_substring(i, &line, "do()");
            if do_result.matched {
                is_enabled = true;
                i = do_result.new_index;
                continue;
            }
            if !is_enabled {
                i += 1;
                continue;
            }
            // Handle don't()
            let dont_result = match_substring(i, &line, "don't()");
            if dont_result.matched {
                is_enabled = false;
                i = do_result.new_index;
                continue;
            }
            if !is_enabled {
                i += 1;
                continue;
            }
            // Handle mul
            let mul_result = match_mul(i, &line);
            if mul_result.matched {
                let mul_string = &line[i..mul_result.new_index];
                total += parse_product(&mul_string);
                i = mul_result.new_index;
                continue
            }

            i += 1;
        }

    }

    println!("Puzzle Two Solution");
    println!("The result is {}", total);

    Ok(())
}


struct MatchResult {
    matched: bool,
    new_index: usize
}

fn match_substring(i: usize, s: &str, substring: &str) -> MatchResult {
    let j = i + substring.chars().count();
    if j > s.chars().count() {
        return MatchResult {
            matched: false,
            new_index: i
        }
    }

    if s[i..j] == *substring {
        return MatchResult {
            matched: true,
            new_index: j
        }
    }

    return MatchResult {
        matched: false,
        new_index: i
    }
} 

// mul(0,0)
fn match_mul(i: usize, s: &str) -> MatchResult {
    if i + 4 > s.chars().count() {
        return MatchResult {
            matched: false,
            new_index: i,
        };
    }

    if &s[i..i + 4] != "mul(" {
        return MatchResult {
            matched: false,
            new_index: i,
        };
    }

    let mut j = i + 8; // min size that mul can be
    let regex = Regex::new(MUL_PATTERN).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)).expect("Expected regex");

    while j <= i + 12 {
       let substring = &s[i..j];
       let matches: Vec<&str> = regex.find_iter(&substring)
            .map(|m| m.as_str())
            .collect();

        if matches.len() > 0 {
            return MatchResult {
                matched: true,
                new_index: j                
            }
        }

        j += 1;
    }

    return MatchResult {
        matched: false,
        new_index: i
    }
}