use itertools::Itertools;

fn main() {
    let sum:usize = include_str!("input.txt")
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(a,b,c)| {
             let mut found:char = '_';
             for ch in a.chars() {
                 if b.contains(ch) && c.contains(ch) {
                     found = ch;
                     break;
                 }
             }
             found
        })
        .map(|c|
            if c <= 'Z' {
                c as u8 - b'A' + 27
            } else {
                c as u8 - b'a' + 1
            } as usize
        )
        .sum();

    println!("sum {:?}", sum);
}
