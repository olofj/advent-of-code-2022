use itertools::Itertools;

fn main() {
    let input: Vec<(usize, usize, usize, usize)> = include_str!("input.txt")
        .lines()
        .map(|l| {
            l.split(&['-', ','][..])
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap()
        })
        .filter(|(s1, e1, s2, e2)| (s1 <= s2 && e1 >= s2) || (s2 <= s1 && e2 >= s1))
        .collect();

    println!("len {}", input.len());
}
