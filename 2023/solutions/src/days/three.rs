/**
 * Advent of Code 2023 Day 3 solution.
 */

use crate::reader;

const filename: &str = "./inputs/three.txt";

pub fn solution() {
    let mut sum: u32 = 0;
    let mut line_vector: Vec<String> = Vec::new();

    // Read the file into lines and construct a line vector.
    if let Ok(lines) = reader::read_lines(filename) {
        for line in lines {
            let l: String = line.expect("Not a string!");
            line_vector.push(l);
        }
    }

    // Loop through the line vector and find numbers.
    let mut current_line: usize = 0;
    // We need to borrow here.
    for line in &line_vector {

        let mut i = 0;
        let char_vec: Vec<char> = line.chars().collect();

        while i < char_vec.len() {

            // Find the left bound of the number.
            while i < char_vec.len() && !char_vec[i].is_digit(10) {
                i += 1;
            }

            if i == char_vec.len() {
                break;
            }

            // Find the right bound of the number.
            let mut j = i;
            while j < char_vec.len() && char_vec[j].is_digit(10) {
                j += 1;
            }
            j -= 1;

            if is_part_number(i, j, current_line, &line_vector) {
                sum += get_number(i, j, current_line, &line_vector);
            }

            i = j + 1;
        }

        current_line += 1;
    } 

    println!("[3] The sum is {}.", sum);
}

fn is_part_number(left: usize, right: usize, line_number: usize, line_vector: &Vec<String>) -> bool {
    // Returns true if the number is adjacent to a symbol.
    let current_line: Vec<char> = line_vector[line_number].chars().collect();
    let mut i = left;
    let mut j = right;

    if left > 0 {
        i -= 1;
    }

    if right < current_line.len() - 1 {
        j += 1;
    }

    if is_symbol(current_line[i]) {
        return true;
    }
    if is_symbol(current_line[j]) {
        return true;
    }
    
    if line_number != 0 {
        let previous_line: Vec<char> = line_vector[line_number - 1].chars().collect();
        for k in i..j+1 {
            if is_symbol(previous_line[k]) {
                return true;
            }
        }
    }

    if line_number != line_vector.len() - 1 {
        let next_line: Vec<char> = line_vector[line_number + 1].chars().collect();
        for k in i..j+1 {
            if is_symbol(next_line[k]) {
                return true;
            }
        } 
    } 

    return false;
}

fn is_symbol(c: char) -> bool {
    return !c.is_digit(10) && c != '.';
}

fn get_number(left: usize, right: usize, line_number: usize, line_vector: &Vec<String>) -> u32 {
    // Return the number defined by the left and right bounds.
    let current_line: Vec<char> = line_vector[line_number].chars().collect();

    let mut num: u32 = 0;

    for i in left..right+1 {
        num *= 10;
        num += current_line[i].to_digit(10).unwrap();
    }

    return num;
}