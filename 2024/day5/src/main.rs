mod parse;
mod updates;

fn main() {
    let parse_result = parse::parse_file();
    let total_from_updates =
        updates::get_total_for_updates(&parse_result.graph, &parse_result.updates);

    println!("Puzzle One Solution:");
    println!("The total is {}", total_from_updates);
}
