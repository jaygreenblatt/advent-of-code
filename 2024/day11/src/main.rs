use constants::{FILENAME, NUM_ITERATIONS, NUM_ITERATIONS_LONG};

mod constants;
mod parse;
mod stones;

fn main() {
    puzzle_one();
    puzzle_two();
}

fn puzzle_one() {
    let stones = parse::parse_stones(FILENAME);
    let total = stones::count_stones(stones, NUM_ITERATIONS);

    println!("Puzzle One Solution");
    println!("Total: {}", total);
}

fn puzzle_two() {
    let stones = parse::parse_stones(FILENAME);
    let total = stones::count_stones(stones, NUM_ITERATIONS_LONG);

    println!("Puzzle Two Solution");
    println!("Total: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{constants::TEST_INPUT_FILENAME, parse};

    #[test]
    fn test_puzzle_one() {
        let stones = parse::parse_stones(TEST_INPUT_FILENAME);
        let total = stones::count_stones(stones, NUM_ITERATIONS);

        assert_eq!(total, 55312);
    }
}
