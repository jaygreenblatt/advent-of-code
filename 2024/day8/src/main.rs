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

fn puzzle_two() {
   let grid = parse::parse_file(); 
   let total = antinode::count_antinodes_with_resonating_frequencies(&grid);

   println!("Puzzle Two Solution");
   println!("Total: {}", total);
}