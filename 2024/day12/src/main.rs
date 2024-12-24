use constants::FILENAME;

mod parse;
mod constants;
mod regions;

fn main() {
    puzzle_one();
}


fn puzzle_one() {
    let grid = parse::parse_grid(FILENAME);
    let total = regions::calculate_total_cost(&grid);

    println!("Puzzle One Solution:");
    println!("The total is: {}", total);
}