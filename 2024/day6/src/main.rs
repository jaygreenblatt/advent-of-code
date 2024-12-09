mod parse;
mod track;

fn main() {
    puzzle_one();
}

fn puzzle_one() {
    let map = parse::parse_file();
    let num_visited = track::get_number_of_visited_positions(&map);

    println!("Puzzle One Solution:");
    println!("Number of visited positions: {}", num_visited);
}