fn main() {
    let input = include_str!("input.txt")
        .lines();
    
    let mut count:usize = 0;

    for l in input {
        let (p1, p2) = l.split_once(",").unwrap();
        let (s1,e1) = p1.split_once("-").unwrap();
        let (s2,e2) = p2.split_once("-").unwrap();
        let s1 = s1.parse::<usize>().unwrap();
        let e1 = e1.parse::<usize>().unwrap();
        let s2 = s2.parse::<usize>().unwrap();
        let e2 = e2.parse::<usize>().unwrap();
        if (s1 <= s2 && e1 >= s2) ||
           (s2 <= s1 && e2 >= s1) {
            count += 1;
        }
    }
    println!("count {}", count);
}
