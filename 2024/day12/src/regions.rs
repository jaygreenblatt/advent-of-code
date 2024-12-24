use std::collections::{HashSet, VecDeque};

use crate::constants::DIRECTIONS;


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

struct RegionCalculation {
    perimeter: i128,
    area: i128
}

impl RegionCalculation {
    pub fn new(perimeter: i128, area: i128) -> RegionCalculation {
        RegionCalculation {
            perimeter,
            area
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

    RegionCalculation::new(perimeter, area)
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
}