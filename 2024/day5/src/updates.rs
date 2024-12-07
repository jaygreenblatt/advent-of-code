use std::collections::{HashMap, HashSet};

use crate::topological_sort::run_topological_sort;

pub fn get_total_for_updates(graph: &HashMap<i32, HashSet<i32>>, updates: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;

    for update in updates.iter() {
        if !is_update_valid(update, graph) {
            continue;
        }

        total += get_middle_number(update)
    }

    total
}


pub fn get_total_for_reordered_updates(graph: &HashMap<i32, HashSet<i32>>, updates: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;

    for update in updates.iter() {
        if !is_update_valid(update, graph) {
            let mini_graph = get_mini_graph(graph, update);
            let sorted = run_topological_sort(&mini_graph);
            total += get_middle_number(&sorted);
        }
    }

    total
}

pub fn get_mini_graph(big_graph: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> HashMap<i32, HashSet<i32>> {
    let mut mini_graph: HashMap<i32, HashSet<i32>> = HashMap::new();

    for from in update {
        let tos = big_graph.get(from).unwrap();
        for to in tos.iter() {
            if update.contains(to) {
                mini_graph.entry(*from).or_insert_with(HashSet::new).insert(*to);
            }
        }
    }

    mini_graph

}

pub fn is_update_valid(update: &Vec<i32>, graph: &HashMap<i32, HashSet<i32>>) -> bool {
    for i in 1..update.len() {
        for j in 0..i - 1 {
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
