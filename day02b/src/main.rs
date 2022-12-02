fn main() {
    let sum: usize = include_str!("input.txt")
        .lines()
        .map(|l| {
            let b = l.as_bytes();
            (b[0] - b'A', b[2] - b'X')
        })
        .map(|(a,b)|
            match b {
                0 => (3+a-1)%3,
                1 => a + 3,
                2 => (a+1)%3  + 6,
                _ => panic!("invalid input")
            } as usize + 1
        )
        .sum();

    println!("sum {:?}", sum);
}
