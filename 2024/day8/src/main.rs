mod parse;
mod antinode;

fn main() {
    puzzle_one();
}


fn puzzle_one() {
    let grid = parse::parse_file();
    let total = antinode::count_antinodes(&grid);

    println!("Puzzle One Solution");
    println!("Total: {}", total);
}