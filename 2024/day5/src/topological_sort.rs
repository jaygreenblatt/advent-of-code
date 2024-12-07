use std::collections::{HashMap, VecDeque};


pub struct TopologicalSortResult {
    pub sorted: Vec<i32>,
    pub value_to_index: HashMap<i32, usize>
}

pub fn run_topological_sort(graph: &HashMap<i32, Vec<i32>>) -> TopologicalSortResult {
    let mut indegrees: HashMap<i32, i32> = HashMap::new();
    let mut sorted: Vec<i32> = Vec::new();
    let mut value_to_index: HashMap<i32, usize> = HashMap::new();
    let mut queue: VecDeque<i32> = VecDeque::new();

    // Initialize the indegrees to 0
    for from in graph.keys() {
        indegrees.insert(*from, 0);
    }

    // Calculate the indegrees
    for (_, tos) in graph.iter() {
        for to in tos {
            *indegrees.entry(*to).or_insert(0) += 1;
        }
    }

    // Push all vertices with indegree 0 to the queue
    for (from, indegree) in indegrees.iter() {
        if *indegree == 0 {
            queue.push_back(*from);
        }
    }

    while !queue.is_empty() {
        let from = queue.pop_front().unwrap();
        sorted.push(from);
        for to in &graph[&from] {
            *indegrees.get_mut(&from).expect("Value should exist") -= 1;
            if *indegrees.get(&from).unwrap() == 0 {
                queue.push_back(*to);
            }
        }
    }

    // Calculate the value to index
    for (i, val) in sorted.iter().enumerate() {
        value_to_index.insert(*val, i);
    }

    TopologicalSortResult {
        sorted,
        value_to_index
    }
}