mod parse;
mod track;

fn main() {
    puzzle_one();
    puzzle_two();
}

fn puzzle_one() {
    let map = parse::parse_file();
    let num_visited = track::get_number_of_visited_positions(&map);

    println!("Puzzle One Solution:");
    println!("Number of visited positions: {}", num_visited);
}

fn puzzle_two() {
    let mut map = parse::parse_file();
    let num_obstacles = track::get_number_of_obstacles_that_result_in_loop(&mut map);

    println!("Puzzle Two Solution:");
    println!("Number of obstacles: {}", num_obstacles);
}