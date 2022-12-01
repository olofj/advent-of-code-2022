fn main() {
    let input: Vec<usize> = include_str!("input.txt")
        //    let input: Vec<&str> = include_str!("sample.txt")
        .split("\n\n")
        .map(|b| b.lines().map(|i| i.parse::<usize>().unwrap()).sum())
        .collect();

    let max = input.iter().max().unwrap();
    println!("max {}", max);
}
