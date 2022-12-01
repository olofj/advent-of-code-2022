fn main() {
    let mut input: Vec<usize> = include_str!("input.txt")
        .split("\n\n")
        .map(|b| b.lines().map(|i| i.parse::<usize>().unwrap()).sum())
        .collect();

    input.sort();
    input.reverse();
    input.truncate(3);
    let sum: usize = input.iter().sum();
    println!("sum {}", sum);
}
