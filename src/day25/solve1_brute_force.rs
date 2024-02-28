use crate::util::read_lines;
use std::collections::{HashMap, HashSet};

fn dfs(node: &String, neighbor_map: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>) {
    visited.insert(node.clone());

    if let Some(neighbors) = neighbor_map.get(node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                dfs(neighbor, neighbor_map, visited);
            }
        }
    }
}

fn get_disconnected_group_sizes(neighbor_map: &HashMap<String, Vec<String>>) -> (usize, usize) {
    let mut visited = HashSet::new();
    let start_node = neighbor_map.keys().next().unwrap();

    dfs(start_node, neighbor_map, &mut visited);

    (visited.len(), neighbor_map.len() - visited.len())
}

pub fn solve() {
    let mut edges: HashSet<(String, String)> = HashSet::new();
    let neighbor_map: HashMap<String, Vec<String>> = read_lines("src/day25/wiring_diagram.txt").unwrap()
        .map(|line| line.expect("Nuuuu! 💣"))
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            let from = parts[0].trim().to_string();
            let to = parts[1].trim().split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
            (from, to)
        })
        .fold(HashMap::new(), |mut acc, (from, tos)| {
            let entry = acc.entry(from.clone()).or_insert(Vec::new());
            entry.extend(tos.clone());

            for to in tos {
                let entry = acc.entry(to.clone()).or_insert(Vec::new());
                entry.push(from.clone());

                if !edges.contains(&(from.clone(), to.clone())) {
                    edges.insert((to.clone(), from.clone()));
                }
            }

            acc
        });

    'outer: for edge_a in &edges {
        for edge_b in &edges {
            for edge_c in &edges {
                let mut current_neighbor_map = neighbor_map.clone();

                current_neighbor_map.get_mut(&edge_a.0).unwrap().retain(|value| value != &edge_a.1);
                current_neighbor_map.get_mut(&edge_a.1).unwrap().retain(|value| value != &edge_a.0);
                current_neighbor_map.get_mut(&edge_b.0).unwrap().retain(|value| value != &edge_b.1);
                current_neighbor_map.get_mut(&edge_b.1).unwrap().retain(|value| value != &edge_b.0);
                current_neighbor_map.get_mut(&edge_c.0).unwrap().retain(|value| value != &edge_c.1);
                current_neighbor_map.get_mut(&edge_c.1).unwrap().retain(|value| value != &edge_c.0);

                let (group_a, group_b) = get_disconnected_group_sizes(&current_neighbor_map);
                if group_b != 0 {
                    println!("Removed edges: {:?}", vec![edge_a, edge_b, edge_c]);
                    println!("Answer: {}", group_a * group_b);
                    break 'outer;
                }
            }
        }
    }
}
