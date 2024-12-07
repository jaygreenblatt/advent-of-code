mod parse;
mod topological_sort;

fn main() {
    let parse_result = parse::parse_file();
    let topological_sort_result = topological_sort::run_topological_sort(&parse_result.graph);
    println!("{:?}", topological_sort_result.sorted);
}
