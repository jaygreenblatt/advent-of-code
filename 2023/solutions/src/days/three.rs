/**
 * Advent of Code 2023 Day 3 solution.
 */

use crate::reader;

const filename: &str = "./inputs/three.txt";

// Solution to part one.
pub fn part_one() {
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

    println!("[3a] The sum is {}.", sum);
}

// Solution to part 2.
pub fn part_two() {
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

    for line in &line_vector {
        let mut i = 0;
        let char_vec: Vec<char> = line.chars().collect();

        while i < char_vec.len() {
            if is_gear(char_vec[i]) {
                let adjacents = get_adjacent_numbers(i, current_line, &line_vector);
                if adjacents.len() == 2 {
                    sum += adjacents[0] * adjacents[1];
                }
            }
            i += 1;
        }
        
        current_line += 1;
    } 

    println!("[3b] The sum is {}.", sum);
}


// Returns true if the number is adjacent to a symbol.
fn is_part_number(left: usize, right: usize, line_number: usize, line_vector: &Vec<String>) -> bool {
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

// Returns true if the character is a symbol
fn is_symbol(c: char) -> bool {
    return !c.is_digit(10) && c != '.';
}


// Returns true if the character is a gear.
fn is_gear(c: char) -> bool {
    return c == '*';
}

// Returns the number of adjacent numbers to a given character.
fn get_adjacent_numbers(spot: usize, line_number: usize, line_vector: &Vec<String>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    let current_line: Vec<char> = line_vector[line_number].chars().collect();

    let mut i = spot;
    let mut j = spot;

    if spot > 0 {
        i -= 1;
    }

    if spot < current_line.len() - 1 {
        j += 1;
    }
    
    // Check if the left spot is a digit.
    if spot > 0 && current_line[i].is_digit(10) {
        result.push(get_left_number(spot, line_number, line_vector));
    }

    // Check if the right spot is a digit.
    if spot < current_line.len() - 1 && current_line[j].is_digit(10) {
        result.push(get_right_number(spot, line_number, line_vector));
    }

    // If there's a next line, check it.
    if line_number < current_line.len() - 1 {
        let bottom_numbers = get_numbers_between_bounds(i, j, line_number + 1, line_vector);
        for num in bottom_numbers {
            result.push(num);
        }
    }

    // If there's a previous line, check it.
    if line_number > 0 {
        let top_numbers = get_numbers_between_bounds(i, j, line_number - 1, line_vector);
        for num in top_numbers {
            result.push(num);
        }
    }

    return result;
}

// Gets any numbers beginning or ending between the specified bounds.
fn get_numbers_between_bounds(i: usize, j: usize, line_number: usize, line_vector: &Vec<String>) -> Vec<u32> {
    let current_line: Vec<char> = line_vector[line_number].chars().collect();
    let mut result: Vec<u32> = Vec::new();
    
    let mut k = i;
    if current_line[k].is_digit(10) {
        let mut l = k;
        while k > 0 && current_line[k].is_digit(10) {
            k -= 1;
        }
        while l < current_line.len() && current_line[l].is_digit(10) {
            l += 1;
        }

        if !current_line[k].is_digit(10) {
            k += 1;
        }

        if l < current_line.len() && !current_line[l].is_digit(10) {
            l -= 1;
        } else if l == current_line.len() {
            l -= 1;
        }
        result.push(get_number(k, l, line_number, line_vector));
        k = l + 1;
    }

    while k <= j {

        while k < current_line.len() && !current_line[k].is_digit(10) {
            k += 1;
        }

        if (k > j) {
            break
        }

        let mut l = k;
        while l < current_line.len() && current_line[l].is_digit(10) {
            l += 1;
        }
        if l < current_line.len() && !current_line[l].is_digit(10) {
            l -= 1;
        } else if l == current_line.len() {
            l -= 1;
        }

        result.push(get_number(k, l, line_number, line_vector));

        k = l + 1;
    }

    return result;
}

// Parse the left spot and return.
fn get_left_number(spot: usize, line_number: usize, line_vector: &Vec<String>) -> u32 {
    let current_line: Vec<char> = line_vector[line_number].chars().collect();
    let j = spot - 1;
    let mut i = j;
    while i > 0 && current_line[i].is_digit(10) {
        i -= 1;
    }
    if !current_line[i].is_digit(10) {
        i += 1;
    }
    return get_number(i, j, line_number, line_vector);
}

// Parse the right spot and return.
fn get_right_number(spot: usize, line_number: usize, line_vector: &Vec<String>) -> u32 {
    let current_line: Vec<char> = line_vector[line_number].chars().collect();
    let i = spot + 1;
    let mut j = i;
    while j < current_line.len() && current_line[j].is_digit(10) {
            j += 1;
    }
    if j < current_line.len() && !current_line[j].is_digit(10) {
        j -= 1;
    } else if j == current_line.len() {
        j -= 1;
    }
    return get_number(i, j, line_number, line_vector); 
}


// Return the number defined by the left and right bounds.
fn get_number(left: usize, right: usize, line_number: usize, line_vector: &Vec<String>) -> u32 {
    let current_line: Vec<char> = line_vector[line_number].chars().collect();

    let mut num: u32 = 0;

    for i in left..right+1 {
        num *= 10;
        num += current_line[i].to_digit(10).unwrap();
    }

    return num;
}