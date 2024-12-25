use std::collections::HashMap;

pub const FILENAME: &str = "input.txt";
pub const TEST_INPUT_FILENAME: &str = "input-test.txt";
pub const DIRECTIONS: [(i32, i32); 4] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0)
];
pub const ORTHOGANAL_DIRECTION_PAIRS: [[(i32, i32); 2]; 4] = [
    [(-1, 0), (0, 1)],
    [(0, 1), (1, 0)],
    [(1, 0), (0, -1)],
    [(0, -1), (-1, 0)]
];