fn main() {
    let input: Vec<(u8, u8)> = include_str!("input.txt")
        .lines()
        .map(|l| { 
            let b = l.as_bytes(); 
            (b[0] - b'A', b[2] - b'X')
        } )
        .collect();

    let base:usize = input.iter()
        .map(|(_,b)| *b as usize + 1 )
        .sum();

    let outcomes: usize = input.iter()
        .map(|(a,b)| { 
            match (3+b-a)%3 {
                1 => 6,
                0 => 3,
                _ => 0,
            }
        })
        .sum();

    println!("base {:?}", base);
    println!("sum {:?}", base+outcomes);
}
