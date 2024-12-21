use std::collections::HashSet;

use crate::constants::{DIRECTIONS, END, START};


pub fn get_total_score(grid: &Vec<Vec<i32>>, ignore_visited: bool) -> i32 {
    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == START {
                let result = bfs(grid, (i as i32, j as i32), ignore_visited);
                total += result;
            }
        }
    }

    total
} 


// Run BFS on the starting position and return the number
// of trailheads starting from this position.
pub fn bfs(grid: &Vec<Vec<i32>>, start: (i32, i32), ignore_visited: bool) -> i32 {
    let mut total = 0;
    let mut queue: Vec<(i32, i32)> = vec![start];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    while !queue.is_empty() {
        let (x, y) = queue.pop().unwrap();
        visited.insert((x, y));

        if grid[x as usize][y as usize] == END {
            total += 1;
            continue;
        }

        for (dx, dy) in DIRECTIONS {
            if x + dx < 0 || x + dx >= m || y + dy < 0 || y + dy >= n {
                continue;
            }

            if grid[(x + dx) as usize][(y + dy) as usize] == grid[x as usize][y as usize] + 1 
            && (ignore_visited || !visited.contains(&(x + dx, y + dy)))
            {
                queue.push((x + dx, y + dy));
            }
        }
    }

    total
}
