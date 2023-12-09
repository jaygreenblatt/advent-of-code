/**
 * Advent of Code 2023 Day 2 solution.
 */
use std::collections::HashMap;
use std::cmp::max;

use regex::Regex;

use crate::reader;

const colors: [&str; 3] = ["red", "blue", "green"];
const filename: &str = "./inputs/two.txt";


// Helper function to get a HashMap of maximum values for each.
fn get_max_hashmap() -> HashMap<&'static str, u32> {
    let mut maxes: HashMap<&str, u32> = HashMap::new(); 
    maxes.insert("red", 12);
    maxes.insert("blue", 14);
    maxes.insert("green", 13);
    maxes
}

// Helper function to get a HashMap of regexes for each color.
fn get_regex_hashmap() -> HashMap<&'static str, Regex> {
    let mut regexes: HashMap<&str, Regex> = HashMap::new();
    regexes.insert("red", Regex::new(r"\d+ red").unwrap());
    regexes.insert("blue", Regex::new(r"\d+ blue").unwrap());
    regexes.insert("green", Regex::new(r"\d+ green").unwrap());
    regexes
}


// Solution to part one.
pub fn part_one() {
    let mut sum: u32 = 0;
    let maxes = get_max_hashmap();
    let regexes = get_regex_hashmap();


    // Read the file into lines.
    if let Ok(lines) = reader::read_lines(filename) {
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
                    let num: u32 = m.split(" ").collect::<Vec<&str>>()[0]
                        .parse::<u32>()
                        .unwrap();
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

    println!("[2a] The sum is {}.", sum);
}


pub fn part_two() {
    let mut sum: u32 = 0;
    let regexes = get_regex_hashmap();

    if let Ok(lines) = reader::read_lines(filename) {
        for line in lines {
            let l: String = line.expect("Not a string!");
            let mut power: u32 = 1;
            for color in colors {
                let reg: Regex = regexes.get(color).expect("No value found").clone();
                let mut min_needed: u32 = 0;
                let matches: Vec<_> = reg.find_iter(l.as_str()).map(|m| m.as_str()).collect();
                for m in matches {
                    let num: u32 = m.split(" ").collect::<Vec<&str>>()[0]
                        .parse::<u32>()
                        .unwrap();
                    min_needed = max(min_needed, num);
                }
                power *= min_needed;
            }
            sum += power;
        }
    }

    println!("[2b] The sum is {}.", sum); 
}