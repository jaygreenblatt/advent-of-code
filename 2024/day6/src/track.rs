use std::collections::HashSet;

const GUARD_UP: char = '^';
const OBSTACLE: char = '#';

#[derive(Copy, Clone, Debug)]
enum Direction {
    LEFT,
    UP,
    RIGHT,
    DOWN
}


pub fn get_number_of_visited_positions(map: &Vec<Vec<char>>) -> usize {
    let mut coord = get_starting_position(map);
    let mut current_direction = Direction::UP;
    let m = map.len() as i32;
    let n = map[0].len() as i32;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    loop {
        let (x, y) = coord;
        let (dx, dy) = get_coord_changes(current_direction);
        visited.insert((x, y));

        if x + dx < 0 || x + dx >= m || y + dy < 0 || y + dy >= n {
            break;
        }
        if map[(x + dx) as usize][(y + dy) as usize] == OBSTACLE {
            current_direction = get_next_direction(current_direction.clone());
        } else {
            coord = (x + dx, y + dy)
        }
    }

    visited.len()
}

fn get_coord_changes(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::LEFT => (0, -1),
        Direction::UP => (-1, 0),
        Direction::RIGHT => (0, 1),
        Direction::DOWN => (1, 0)
    }
}

fn get_next_direction(direction: Direction) -> Direction {
    match direction {
        Direction::LEFT => Direction::UP,
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT
    }
}

// Get the starting position of the guard
fn get_starting_position(map: &Vec<Vec<char>>) -> (i32, i32) {
    let mut coord: (i32, i32) = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == GUARD_UP {
                coord = (i as i32, j as i32)
            }
        }
    }
    coord
}