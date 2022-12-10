use std::collections::HashSet;

fn main() {
    let mut rope: Vec<(isize, isize)> = vec![(0, 0); 10];
    let mut been: Vec<HashSet<(isize, isize)>> = (0..10)
        .map(|_| vec![(0, 0)].into_iter().collect())
        .collect::<Vec<HashSet<(isize, isize)>>>();

    include_str!("input.txt")
        .lines()
        .map(|l| {
            let cmd = l.split_once(" ").unwrap();
            (cmd.0, cmd.1.parse::<isize>().unwrap())
        })
        .for_each(|(cmd, dist)| {
            for _ in 0..dist {
                match cmd {
                    "U" => rope[0].0 += 1,
                    "D" => rope[0].0 -= 1,
                    "L" => rope[0].1 -= 1,
                    "R" => rope[0].1 += 1,
                    _ => panic!("unknown command"),
                }
                been[0].insert(rope[0]);
                for i in 1..=9 {
                    let p = rope[i-1];
                    let t = rope[i];
                    let delta = (p.0-t.0, p.1-t.1);
                    if delta.0.abs() > 1 || delta.1.abs() > 1 {
                        rope[i].0 = t.0 + delta.0.signum();
                        rope[i].1 = t.1 + delta.1.signum();
                    }
                    been[i].insert(rope[i]);
                }
            }
        });

    println!("been: {}", been[9].len());
}
