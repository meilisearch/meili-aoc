use std::collections::HashSet;

use meili_aoc::*;

fn main() {
    let input = std::env::args().nth(1).expect("Give me the input.");
    let input = std::fs::read_to_string(&input).expect(&format!("Could not open {input}"));

    let mut trie = Trie::default();

    for input in input.lines() {
        let input = input.split("-").map(|s| s.trim()).collect::<Vec<_>>();
        let (name, directions) = (input[0], input[1]);

        let directions = directions
            .chars()
            .map(|c| c.to_string().parse::<Dir>().unwrap())
            .collect::<Vec<Dir>>();

        trie.insert(&directions, name.to_string());
    }
    trie.finish();
    // trie.to_graph();

    println!("Part 1:");
    println!("Number of nodes: {}", trie.nb_nodes());
    println!("Depth: {}", trie.depth());

    let (fast, distance) = trie.fastest_access(&HashSet::new()).unwrap();

    println!("Fastest to access: {}", fast.terminate[0]);
    println!("In {} instructions.", distance);
    println!("Leftest children in the trie: {}", trie.first());

    // -------------- Part 2
    println!("Part 2:");
    let mut current = &trie;
    let mut ignored = HashSet::new();
    let mut total_distance = 0;
    loop {
        ignored.insert(current as *const Trie);
        if let Some((trie, dist)) = current.fastest_access(&ignored) {
            current = trie;
            total_distance += dist;
            // println!(
            //     "Gift to {} in {} steps, total: {}",
            //     current.terminate[0], dist, total_distance
            // );
        } else {
            println!("{total_distance} nodes has been covered.");
            break;
        }
    }
}
