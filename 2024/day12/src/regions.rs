use std::collections::{HashMap, HashSet, VecDeque};

use crate::constants::{DIRECTIONS, ORTHOGANAL_DIRECTION_PAIRS};


pub fn calculate_total_cost(grid: &Vec<Vec<char>>) -> i128 {
    let mut result = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let m = grid.len();
    let n = grid[0].len();

    for i in 0..m {
        for j in 0..n {
            let i = i as i32;
            let j = j as i32;
            if !visited.contains(&(i, j)) {
                visited.insert((i, j));
                let region_calculation = get_perimeter_and_area_for_region(grid, &mut visited, (i, j));
                result += region_calculation.perimeter * region_calculation.area;
            }
        }
    }

    result
}

pub fn calculate_total_cost_v2(grid: &Vec<Vec<char>>) -> i128 {
   let mut result = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let m = grid.len();
    let n = grid[0].len();

    for i in 0..m {
        for j in 0..n {
            let i = i as i32;
            let j = j as i32;
            if !visited.contains(&(i, j)) {
                visited.insert((i, j));
                let region_calculation = get_sides_and_area_for_region(grid, &mut visited, (i, j));
                println!("sides: {}, area: {}", region_calculation.sides, region_calculation.area);
                result += region_calculation.sides * region_calculation.area;
            }
        }
    }

    result 
}

struct RegionCalculation {
    perimeter: i128,
    area: i128,
    sides: i128,
}

impl RegionCalculation {
    pub fn new(perimeter: i128, area: i128, sides: i128) -> RegionCalculation {
        RegionCalculation {
            perimeter,
            area,
            sides
        }
    }
}

fn get_perimeter_and_area_for_region(
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(i32, i32)>,
    start: (i32, i32)
) -> RegionCalculation {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;
    let mut perimeter = 0;
    let mut area = 0;
    let mut queue: VecDeque<(i32, i32)> = VecDeque::from([start]);
    let current_char = grid[start.0 as usize][start.1 as usize];
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        area += 1;
        for (dx, dy) in DIRECTIONS {
            let new_x = x + dx;
            let new_y = y + dy;

            if new_x < 0 || new_x >= m || new_y < 0 || new_y >= n {
                perimeter += 1;
                continue;
            }

            if grid[new_x as usize][new_y as usize] != current_char {
                perimeter += 1;
                continue;
            }

            if !visited.contains(&(new_x, new_y)) {
                queue.push_back((new_x, new_y));
                visited.insert((new_x, new_y));
            }
        }
    }

    RegionCalculation::new(perimeter, area, 0)
}


fn get_sides_and_area_for_region(
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(i32, i32)>,
    start: (i32, i32)
) -> RegionCalculation {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;
    let mut area = 0;
    let mut sides = 0;
    let mut queue: VecDeque<(i32, i32)> = VecDeque::from([start]);
    let current_char = grid[start.0 as usize][start.1 as usize];
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        area += 1;

        sides += count_external_corners(grid, (x, y), m, n);
        sides += count_internal_corners(grid, (x, y), m, n);

        for (dx, dy) in DIRECTIONS {
            let new_x = x + dx;
            let new_y = y + dy;

            if is_in_bounds((new_x, new_y), m, n) && grid[new_x as usize][new_y as usize] == current_char && !visited.contains(&(new_x, new_y)) {
                queue.push_back((new_x, new_y));
                visited.insert((new_x, new_y));
            }
        }
    }

    RegionCalculation::new(0, area, sides)
}


fn count_external_corners(grid: &Vec<Vec<char>>, pos: (i32, i32), m: i32, n: i32) -> i128 {
    let c = grid[pos.0 as usize][pos.1 as usize];
    let (x, y) = pos;
    let mut num_corners = 0;

    for direction_pair in ORTHOGANAL_DIRECTION_PAIRS {
        let (dx1, dy1) = direction_pair[0];
        let (dx2, dy2) = direction_pair[1];
        let mut num_edges = 0;

        if x + dx1 < 0 || x + dx1 >= m || y + dy1 < 0 || y + dy1 >= n {
            num_edges += 1;
        }

        if x + dx2 < 0 || x + dx2 >= m || y + dy2 < 0 || y + dy2 >= n {
            num_edges += 1;
        }

        if is_in_bounds((x + dx1, y + dy1), m, n) && grid[(x + dx1) as usize][(y + dy1) as usize] != c {
            num_edges += 1;
        }

        if is_in_bounds((x + dx2, y + dy2), m, n) && grid[(x + dx2) as usize][(y + dy2) as usize] != c {
            num_edges += 1;
        }

        if num_edges == 2 {
            num_corners += 1;
        }
    }

    num_corners
}

fn is_in_bounds(pos: (i32, i32), m: i32, n: i32) -> bool {
    let (x, y) = pos;
    return x >= 0 && x < m && y >= 0 && y < n;
}

fn count_internal_corners(grid: &Vec<Vec<char>>, pos: (i32, i32), m: i32, n: i32) -> i128 {
    let c = grid[pos.0 as usize][pos.1 as usize];
    let (x, y) = pos;
    let mut num_corners = 0;
    let diagonal_directions: HashMap<[(i32, i32); 2], (i32, i32)> = HashMap::from([
        ([(-1, 0), (0, 1)], (-1, 1)),
        ([(0, 1), (1, 0)], (1, 1)),
        ([(1, 0), (0, -1)], (1, -1)),
        ([(0, -1), (-1, 0)], (-1, -1)), 
    ]);
    for direction_pair in ORTHOGANAL_DIRECTION_PAIRS {
        let (dx1, dy1) = direction_pair[0];
        let (dx2, dy2) = direction_pair[1];
        let mut num_edges = 0;

        if is_in_bounds((x + dx1, y + dy1), m, n) && grid[(x + dx1) as usize][(y + dy1) as usize] == c {
            num_edges += 1;
        }

        if is_in_bounds((x + dx2, y + dy2), m, n) && grid[(x + dx2) as usize][(y + dy2) as usize] == c {
            num_edges += 1;
        }

        if num_edges == 2 {
            let (dx3, dy3) = diagonal_directions[&direction_pair];
            if is_in_bounds((x + dx3, y + dy3), m, n) && grid[(x + dx3) as usize][(y + dy3) as usize] != c {
                num_corners += 1;
            }
        }
    }
    num_corners
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_cost() {
        let input = vec![
            vec!['R','R','R','R','I','I','C','C','F','F'],
            vec!['R','R','R','R','I','I','C','C','C','F'],
            vec!['V','V','R','R','R','C','C','F','F','F'],
            vec!['V','V','R','C','C','C','J','F','F','F'],
            vec!['V','V','V','V','C','J','J','C','F','E'],
            vec!['V','V','I','V','C','C','J','J','E','E'],
            vec!['V','V','I','I','I','C','J','J','E','E'],
            vec!['M','I','I','I','I','I','J','J','E','E'],
            vec!['M','I','I','I','S','I','J','E','E','E'],
            vec!['M','M','M','I','S','S','J','E','E','E'],
        ];

        assert_eq!(calculate_total_cost(&input), 1930);
    }

    #[test]
    fn test_calculate_total_cost_v2_simple() {
        let input = vec![
            vec!['A','A','A','A','A','A'],
            vec!['A','A','A','B','B','A'],
            vec!['A','A','A','B','B','A'],
            vec!['A','B','B','A','A','A'],
            vec!['A','B','B','A','A','A'],
            vec!['A','A','A','A','A','A'],
        ];

        assert_eq!(calculate_total_cost_v2(&input), 368);
    }

    #[test]
    fn test_calculate_total_cost_v2() {
        let input = vec![
            vec!['R','R','R','R','I','I','C','C','F','F'],
            vec!['R','R','R','R','I','I','C','C','C','F'],
            vec!['V','V','R','R','R','C','C','F','F','F'],
            vec!['V','V','R','C','C','C','J','F','F','F'],
            vec!['V','V','V','V','C','J','J','C','F','E'],
            vec!['V','V','I','V','C','C','J','J','E','E'],
            vec!['V','V','I','I','I','C','J','J','E','E'],
            vec!['M','I','I','I','I','I','J','J','E','E'],
            vec!['M','I','I','I','S','I','J','E','E','E'],
            vec!['M','M','M','I','S','S','J','E','E','E'],
        ];

        assert_eq!(calculate_total_cost_v2(&input), 1206);
    }

}