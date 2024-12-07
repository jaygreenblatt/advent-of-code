mod parse;

fn main() {
    let result = parse::parse_file();
    println!("Graph {:?}", result.graph);
    println!("Updates {:?}", result.updates);
}
