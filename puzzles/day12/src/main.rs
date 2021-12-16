use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");

    println!("First solution: {}", first_solution(&input));
    println!("Second solution: {}", second_solution(&input));
}

fn second_solution(input: &str) -> u32 {
    let mut graph = Graph::from(input);
    let mut visited = HashMap::new();
    pathfinder(&graph, "start", &mut visited, true)
}

fn first_solution(input: &str) -> u32 {
    let mut graph = Graph::from(input);
    let mut visited = HashMap::new();
    pathfinder(&graph, "start", &mut visited, false)
}

fn pathfinder(graph: &Graph, start_node: &str, visited: &mut HashMap<String, u32>, can_use_double: bool) -> u32 {
    let nodes = graph.map.get(start_node).unwrap();
    let mut result = 0;

    if start_node.chars().next().unwrap().is_lowercase() {
        visited.entry(start_node.to_string()).and_modify(|v| *v += 1).or_insert(1);
    }

    for node in nodes.iter() {
        let visit = visited.get(node);
        if *node == "end" {
            result += 1;
        } else if  visit.is_some() && *visit.unwrap() > 0 && !can_use_double {
            continue;
        } else if visit.is_some() && *visit.unwrap() > 0 && can_use_double {
            result += pathfinder(graph, node, visited, false);
        } else {
            result += pathfinder(graph, node, visited, can_use_double);
        }
    }

    visited.entry(start_node.to_string()).and_modify(|v| *v -= 1);
    result
}


struct Graph {
    map: HashMap<String, Vec<String>>,
}

impl Graph {
    fn from(str: &str) -> Self {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        str.lines()
            .map(|l| {
                let mut split = l.split('-');
                (split.next().unwrap(), split.next().unwrap())
            })
            .for_each(|(a, b)| {
                if b != "start" && a != "end" {
                    map.entry(a.to_string())
                        .and_modify(|e| e.push(b.to_string()))
                        .or_insert(vec![b.to_string()]);
                }
                if a != "start" && b != "end"{
                    map.entry(b.to_string())
                        .and_modify(|e| e.push(a.to_string()))
                        .or_insert(vec![a.to_string()]);
                }
            });

        Self { map }
    }

    fn print(&self) {
        for (key, val) in &self.map {
            println!("{} -> {:?}", key, val);
        }
    }
}


mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 1 + 1);
    }
}