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

    println!("Number of nodes: {}", trie.nb_nodes());
    println!("Depth: {}", trie.depth());
    println!("Fastest to access: {}", trie.fastest_access().0);
    println!("First children to get his gift: {}", trie.first());
}
