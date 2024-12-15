use std::collections::HashSet;




const EMPTY: char = '.';

pub fn count_antinodes(grid: &Vec<Vec<char>>) -> usize {
    let positions = get_antenna_positions(grid);
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    let mut antinode_locations: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..positions.len() {
        let (x1, y1) = positions[i];
        for j in 0..positions.len() {
            if i == j {
                continue;
            }
            let (x2, y2) = positions[j];
            if grid[x1][y1] == grid[x2][y2] {
                let (antinode_x, antinode_y) = get_antinode_position(x1 as i32, y1 as i32, x2 as i32, y2 as i32);
                if antinode_x < 0 || antinode_x >= m || antinode_y < 0 || antinode_y >= n {
                    continue;
                }
                antinode_locations.insert((antinode_x, antinode_y));
            }
            
        }
    }

    antinode_locations.len()
}


pub fn count_antinodes_with_resonating_frequencies(grid: &Vec<Vec<char>>) -> usize {
    let positions = get_antenna_positions(grid);
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;
    let mut antinode_locations: HashSet<(i32, i32)> = HashSet::new(); 

    for i in 0..positions.len() {
        for j in 0..positions.len() {
            if i == j {
                continue;
            }
            let coord1 = positions[i];
            let x1 = coord1.0 as i32;
            let y1 = coord1.1 as i32;
            let coord2 = positions[j];
            let x2 = coord2.0 as i32;
            let y2 = coord2.1 as i32;

            if grid[x1 as usize][y1 as usize] == grid[x2 as usize][y2 as usize] {
                antinode_locations.insert((x1, y1));
                antinode_locations.insert((x2, y2));
                let dx = x2 - x1;
                let dy = y2 - y1;
                let mut curr_x = x1;
                let mut curr_y = y1;
                while curr_x >= 0 && curr_x < m && curr_y >= 0 && curr_y < n {

                    curr_x -= dx;
                    curr_y -= dy;
                    if curr_x < 0 || curr_x >= m || curr_y < 0 || curr_y >= n {
                        break;
                    }
                    antinode_locations.insert((curr_x, curr_y));
                }
            }
            
        }
    }

    antinode_locations.len()
}


fn get_antenna_positions(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != EMPTY {
                positions.push((i, j));
            }
        }
    }

    positions

}

fn get_antinode_position(x1: i32, y1: i32, x2: i32, y2: i32) -> (i32, i32) {
    let ydiff = y2 - y1;
    let xdiff = x2 - x1;

    ((x1 - xdiff) as i32, (y1 - ydiff) as i32)
}