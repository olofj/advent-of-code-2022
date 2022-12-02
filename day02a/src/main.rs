fn main() {
    let sum: usize = include_str!("input.txt")
        .lines()
        .map(|l| {
            let b = l.as_bytes();
            (b[0] - b'A', b[2] - b'X')
        } )
        .map(|(a,b)|
            match (3+b-a)%3 {
                1 => 6,
                0 => 3,
                _ => 0,
            } + b as usize + 1
        )
        .sum();

    println!("sum {:?}", sum);
}
