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
}

fn puzzle_one() {
    let board = load_input().unwrap();
    let total = count_instances(&board);

    println!("Puzzle One Solution:");
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


fn count_instances(board: &Vec<Vec<char>>) -> i32 {
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