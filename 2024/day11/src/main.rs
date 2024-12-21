use constants::FILENAME;

mod constants;
mod parse;
mod stones;

fn main() {
    puzzle_one();
}


fn puzzle_one() {
    let stones = parse::parse_stones(FILENAME);
    let total = stones::count_stones(stones);

    println!("Puzzle One Solution");
    println!("Total: {}", total);
}


#[cfg(test)]
mod tests {
    use crate::{constants::TEST_INPUT_FILENAME, parse};
    use super::*;

    #[test]
    fn test_puzzle_one() {
        let stones = parse::parse_stones(TEST_INPUT_FILENAME);
        let total = stones::count_stones(stones);

        assert_eq!(total, 55312);
    }
}