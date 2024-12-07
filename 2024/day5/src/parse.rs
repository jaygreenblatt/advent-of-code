use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input.txt";

pub struct ParseResult {
    pub graph: HashMap<i32, HashSet<i32>>,
    pub updates: Vec<Vec<i32>>,
}

pub fn parse_file() -> ParseResult {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);
    let mut is_line_on_updates = false;
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if is_line_on_updates {
            updates.push(parse_update(&line));
            continue;
        }
        if line.is_empty() {
            is_line_on_updates = true;
            continue;
        }

        let (to, from) = parse_edge(&line);
        graph.entry(to).or_insert_with(HashSet::new).insert(from); // Avoids explicit check if the key exists
    }

    ParseResult { graph, updates }
}

fn parse_edge(line: &str) -> (i32, i32) {
    let split: (&str, &str) = line.split_once("|").unwrap();
    let to = split.0.parse::<i32>().unwrap();
    let from = split.1.parse::<i32>().unwrap();

    (to, from)
}

fn parse_update(line: &str) -> Vec<i32> {
    let mut update: Vec<i32> = Vec::new();

    let split: Vec<&str> = line.split(",").collect();

    for s in split {
        update.push(s.parse::<i32>().unwrap())
    }

    update
}
