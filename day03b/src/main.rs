use itertools::Itertools;

fn main() {
    let sum:usize = include_str!("input.txt")
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(a,b,c)| a.chars().find(|&ch| b.contains(ch) && c.contains(ch)))
        .map(|c| c.unwrap() as u8)
        .map(|c|
            match c {
                b'A'..=b'Z' => c - b'A' + 27,
                b'a'..=b'z' => c - b'a' + 1,
                _ => 0,
            } as usize
        )
        .sum();

    println!("sum {:?}", sum);
}
