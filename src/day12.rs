use std::{collections::HashMap, fs};

struct Visited {
    twice_node: Option<String>,
    count: HashMap<String, u32>,
}

impl Visited {
    fn new() -> Visited {
        Visited {
            twice_node: None,
            count: HashMap::new(),
        }
    }

    fn insert(&mut self, node: &str) {
        let val = self.count.get(node).unwrap_or(&0) + 1;
        if val >= 2 && self.twice_node == None {
            self.twice_node = Some(node.to_string());
        }
        self.count.insert(node.to_string(), val);
    }

    fn contains(&self, node: &str) -> bool {
        let val = self.count.get(node).unwrap_or(&0);
        match &self.twice_node {
            Some(x) => val == &1 || x == node && val >= &2,
            None => val >= &2,
        }
    }

    fn remove(&mut self, conn: &str) {
        let val = *self.count.get(conn).unwrap_or(&0);
        if val != 0 {
            self.count.insert(conn.to_string(), val - 1);
        }
    }
}

pub fn get_result() {
    let inp = fs::read_to_string("src/day12.input").expect("Error in file opening");
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut visited: Visited = Visited::new();
    let mut to_visit: Vec<&str> = Vec::new();

    inp.split("\n").filter(|l| l.len() > 0).for_each(|l| {
        let (left, right) = l.split_once("-").unwrap();
        if !nodes.contains_key(left) {
            nodes.insert(left, Vec::new());
        }
        nodes.get_mut(left).unwrap().push(right);
        if !nodes.contains_key(right) {
            nodes.insert(right, Vec::new());
        }
        nodes.get_mut(right).unwrap().push(left);
    });
    to_visit.push("start");

    println!("{}", get_count("start", &mut visited, &nodes));
}

fn get_count<'a>(node: &'a str, visited: &mut Visited, nodes: &'a HashMap<&str, Vec<&str>>) -> u32 {
    let mut count = 0;
    if node.chars().next().unwrap().is_lowercase() {
        visited.insert(node);
    }
    for conn in nodes.get(node).unwrap() {
        let conn_twice_node = visited.twice_node.clone();
        if conn == &"end" {
            count += 1;
            continue;
        }
        if visited.contains(conn) || conn == &"start" {
            continue;
        }

        count += get_count(conn, visited, nodes);
        visited.twice_node = conn_twice_node;
        visited.remove(conn);
    }
    count
}
