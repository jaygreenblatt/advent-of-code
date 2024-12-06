use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const FILENAME: &str = "src/input.txt";
const FIRST_CHAR: char = 'X';
const FINAL_CHAR: char = 'S';
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0)
];

fn main() {
    puzzle_one();
    puzzle_two();
}

fn puzzle_one() {
    let board = load_input().unwrap();
    let total = count_instances_of_xmas(&board);

    println!("Puzzle One Solution:");
    println!("Total Instances: {}", total);
}

fn puzzle_two() {
    let board = load_input().unwrap();
    let total = count_crosses(&board);

    println!("Puzzle Two Solution:");
    println!("Total Instances: {}", total);
}


fn load_input() -> io::Result<Vec<Vec<char>>> {
    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);
    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut chars: Vec<char> = Vec::new();
        for c in line.chars() {
            chars.push(c);
        }
        result.push(chars);
    }

    Ok(result)
}


fn count_crosses(board: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;
    let opposite_letters: HashMap<char, char> = HashMap::from([('M', 'S'), ('S', 'M')]);
    let m = board.len() as i32;
    let n = board[0].len() as i32;

    for i in 0..m {
        for j in 0..n {
            if board[i as usize][j as usize] != 'A' {
                continue
            }

            if i - 1 < 0 || i + 1 >= m || j - 1 < 0 || j + 1 >= n {
                continue;
            }

            let top_left = board[(i - 1) as usize][(j + 1) as usize];
            let bottom_right = board[(i + 1) as usize][(j - 1) as usize];
            let bottom_left = board[(i - 1) as usize][(j - 1) as usize];
            let top_right = board[(i + 1) as usize][(j + 1) as usize];

            if opposite_letters.contains_key(&top_left) && opposite_letters[&top_left] == bottom_right 
                && opposite_letters.contains_key(&bottom_left) && opposite_letters[&bottom_left] == top_right {
                    total += 1;
                }
        }
    }

    total
}

fn count_instances_of_xmas(board: &Vec<Vec<char>>) -> i32 {
    let letters: HashMap<char, char> = HashMap::from([('X', 'M'), ('M', 'A'), ('A', 'S')]);
    let mut total = 0;
    let m = board.len() as i32;
    let n = board[0].len() as i32;

    for i in 0..m {
        for j in 0..n {
            if board[i as usize][j as usize] != FIRST_CHAR {
                continue;
            }
      
            for d in DIRECTIONS {
                let curr_x = i;
                let curr_y = j;
                let dx = d.0;
                let dy = d.1;
                let mut curr_letter = FIRST_CHAR;
                
                for k in 1..4 {
                    if curr_x + (k * dx) < 0 || curr_x + (k * dx) >= m {
                        break;
                    }

                    if curr_y + (k * dy) < 0 || curr_y + (k * dy) >= n {
                        break;
                    }

                    if board[(curr_x + (k * dx)) as usize][(curr_y + (k * dy)) as usize] == letters[&curr_letter] {
                        if letters[&curr_letter] == FINAL_CHAR {
                            total += 1;
                            break;
                        }
                        curr_letter = letters[&curr_letter];
                    }
                }
                
            }

        }
    }

    total
}