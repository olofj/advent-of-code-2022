fn main() {
//    let input: Vec<usize> = include_str!("input.txt")
    let input: Vec<&str> = include_str!("sample.txt")
        .lines()
        .collect();

    println!("{:?}", input);
}

