use crate::util::read_lines;
use std::{cmp::min, cmp::max, collections::{hash_map::Entry::{Occupied, Vacant}, HashMap}};
use rand::Rng;


fn get_or_set_index_of_node(
    node: &String,
    index_by_name: &mut HashMap<String, usize>,
    next_index: &mut usize
) -> (usize, bool) {
    let (is_new, index) = match index_by_name.entry(node.clone()) {
        Occupied(entry) => (false, *entry.get()),
        Vacant(entry) => {
            let index = *next_index;
            *next_index += 1;
            entry.insert(index);
            (true, index)
        }
    };

    (index, is_new)
}

fn init_nodes_and_get_initial_edges(lines: &mut dyn Iterator<Item = std::io::Result<String>>) -> (Vec<i32>, Vec<(usize, usize)>) {
    let mut indexes_by_name = HashMap::<String, usize>::new();
    let mut next_index = 0;

    let mut node_merge_counts = Vec::new();
    let mut initial_edges = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let (from, tos) = line.split_once(": ").unwrap();
        let (from_i, is_new) = get_or_set_index_of_node(&from.to_string(), &mut indexes_by_name, &mut next_index);
        if is_new {
            node_merge_counts.push(1);
        }
        for to in tos.split_ascii_whitespace() {
            let (to_i, is_new) = get_or_set_index_of_node(&to.to_string(), &mut indexes_by_name, &mut next_index);
            if is_new {
                node_merge_counts.push(1);
            }
            initial_edges.push((min(from_i, to_i), max(from_i, to_i)));
        }
    }
    (node_merge_counts, initial_edges)
}

fn clean_up_edges(edges: &mut Vec<(usize, usize)>, node_a: usize, node_b: usize) {
    let mut i = 0;


    while i < edges.len() {
        let (node_c, node_d) = edges[i];

        if (node_c, node_d) == (node_a, node_b) {
            edges.swap_remove(i);
        } else if node_c == node_b {
            edges[i] = (min(node_a, node_d), max(node_a, node_d));
            i += 1;
        } else if node_d == node_b {
            edges[i] = (min(node_a, node_c), max(node_a, node_c));
            i += 1;
        } else {
            i += 1;
        }
    }
}

pub fn solve() {
    let mut lines = read_lines("src/day25/wiring_diagram.txt").unwrap();
    let (
        mut node_merge_counts,
        initial_edges
    ) = init_nodes_and_get_initial_edges(&mut lines);

    let mut rng = rand::thread_rng();
    let mut edges = Vec::new();

    loop {
        for count in &mut node_merge_counts { *count = 1; }

        edges.extend(initial_edges.iter().copied());

        let mut num_node_merge_counts = node_merge_counts.len();

        while num_node_merge_counts > 2 {
            let index = rng.gen_range(0..edges.len());
            let (a, b) = edges.swap_remove(index);
            node_merge_counts[a] += node_merge_counts[b];

            clean_up_edges(&mut edges, a, b);
            num_node_merge_counts -= 1;
        }

        if edges.len() == 3 {
            let (a, b) = edges[0];

            println!("Edges: {:?}", edges);
            println!("Sum: {}", node_merge_counts[a] as u32 * node_merge_counts[b] as u32);
            break;
        }
        edges.clear();
    }
}
