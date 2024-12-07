use std::collections::{HashMap, HashSet};

pub fn get_total_for_updates(
    graph: &HashMap<i32, HashSet<i32>>,
    updates: &Vec<Vec<i32>>
) -> i32 {
    let mut total = 0;

    for update in updates.iter() {
        if !is_update_valid(update, graph) {
            continue;
        }

        total += get_middle_number(update)
    }

    total
}


pub fn is_update_valid(update: &Vec<i32>, graph: &HashMap<i32, HashSet<i32>>) -> bool {

    for i in 1..update.len() {
        for j in 0..i-1 {
            if graph.get(&update[i]).unwrap().contains(&update[j]) {
                return false;
            }
        }
    }

    true
}


pub fn get_middle_number(update: &Vec<i32>) -> i32 {
    update[update.len() / 2]
}