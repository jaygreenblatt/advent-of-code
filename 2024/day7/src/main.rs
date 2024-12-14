mod parse;
mod calculate;

fn main() {
    puzzle_one();
    puzzle_two();
}

fn puzzle_one() {
    let calibrations = parse::parse_file();
    let mut total = 0;
    for calibration in calibrations.iter() {
        total += calculate::calibrate(calibration.0, calibration.1[0], 1, &calibration.1);
    }

    println!("Puzzle One Solution:");
    println!("The total is {}", total);
}


fn puzzle_two() {
    let calibrations = parse::parse_file();
    let mut total = 0;
    for calibration in calibrations.iter() {
        total += calculate::calibrate_v2(calibration.0, calibration.1[0], 1, &calibration.1);
    }

    println!("Puzzle Two Solution:");
    println!("The total is {}", total);
}