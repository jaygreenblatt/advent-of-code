mod reader;
#[path = "days/one.rs"] mod one;
#[path = "days/two.rs"] mod two;

fn main() {
    one::solution();
    two::part_one();
    two::part_two();
}
