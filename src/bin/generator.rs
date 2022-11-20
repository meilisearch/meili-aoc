use rand::prelude::*;

fn main() {
    let names = std::env::args().nth(1).expect("Give a list of name");
    let names = std::fs::read_to_string(&names).expect(&format!("Could not open {names}"));

    let mut rng = thread_rng();

    for children in names.lines() {
        let directions: usize = rng.gen_range(10..30);
        let directions = (0..directions)
            .map(|_| if rng.gen() { 'L' } else { 'R' })
            .collect::<String>();

        println!("{children} - {directions}");
    }
}
