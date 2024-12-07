use std::collections::{HashMap, HashSet, VecDeque};

pub fn run_topological_sort(graph: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut indegrees: HashMap<i32, i32> = HashMap::new();
    let mut sorted: Vec<i32> = Vec::new();

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
        let tos = graph.get(&from);
        if tos.is_none() {
            continue;
        }
        for to in &graph[&from] {
            *indegrees.get_mut(&to).expect("Value should exist") -= 1;
            if *indegrees.get(&to).unwrap() == 0 {
                queue.push_back(*to);
            }
        }
    }

    sorted
}