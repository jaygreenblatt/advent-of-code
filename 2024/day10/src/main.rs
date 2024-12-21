use constants::FILENAME;

mod parse;
mod constants;
mod search;

fn main() {
    puzzle_one();
    puzzle_two();
}


fn puzzle_one() {
    let grid = parse::parse_file_into_grid(FILENAME);
    let total = search::get_total_score(&grid, false);

    println!("Puzzle One Solution");
    println!("Total is {}", total);
}

fn puzzle_two() {
    let grid = parse::parse_file_into_grid(FILENAME);
    let total = search::get_total_score(&grid, true);

    println!("Puzzle One Solution");
    println!("Total is {}", total);
}


#[cfg(test)]
mod tests {
    use constants::TEST_INPUT_FILENAME;

    use super::*;

    #[test]
    fn test_puzzle_one() {
        let grid = parse::parse_file_into_grid(TEST_INPUT_FILENAME);
        let total = search::get_total_score(&grid, false);
        assert_eq!(total, 36);
    }

    #[test]
    fn test_puzzle_two() {
        let grid = parse::parse_file_into_grid(TEST_INPUT_FILENAME);
        let total = search::get_total_score(&grid, true);   
        assert_eq!(total, 81);
    }
}