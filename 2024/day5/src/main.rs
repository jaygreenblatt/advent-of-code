use parse::ParseResult;

mod parse;
mod updates;
mod topological_sort;

fn main() {
    let parse_result = parse::parse_file();
    puzzle_one(&parse_result);
    puzzle_two(&parse_result);
}


fn puzzle_one(parse_result: &ParseResult) {
    let total_from_updates =
        updates::get_total_for_updates(&parse_result.graph, &parse_result.updates);

    println!("Puzzle One Solution:");
    println!("The total is {}", total_from_updates);
}


fn puzzle_two(parse_result: &ParseResult) {
    let total = updates::get_total_for_reordered_updates(&parse_result.graph, &parse_result.updates);

    print!("Puzzle Two Solution:");
    println!("The total is {}", total);
}