use std::collections::HashSet;

const HLEN: usize = 4;  // For part 1
//const HLEN: usize = 14; // For part 2

fn main() {
    let input:Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|l| l.chars()
            .collect::<Vec<char>>()
            .windows(HLEN)
            .enumerate()
            .filter_map(|(i, t)| {
                let mut hs = HashSet::new();
                t.iter().all(|c| hs.insert(c)).then(|| i + HLEN)
            })
            .nth(0)
            .unwrap()
        )
        .collect();

    println!("found at {:?}", input);
}
