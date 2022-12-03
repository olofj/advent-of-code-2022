
fn main() {
    let sum:usize = include_str!("input.txt")
        .lines()
        .map(|l| {
            let sz = l.chars().count();
            let mut found = 'a';
            for c in l.chars() {
                if l.rfind(c).unwrap() >= sz/2 {
                    found = c;
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
