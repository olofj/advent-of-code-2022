use std::collections::HashSet;

fn main() {
    let mut head: (isize, isize) = (0, 0);
    let mut tail: (isize, isize) = (0, 0);
    let mut been: HashSet<(isize, isize)> = HashSet::new();

    include_str!("input.txt")
        .lines()
        .map(|l| {
            let cmd = l.split_once(" ").unwrap();
            (cmd.0, cmd.1.parse::<isize>().unwrap())
        })
        .for_each(|(cmd, dist)| {
            for _ in 0..dist {
                match cmd {
                    "U" => head.0 += 1,
                    "D" => head.0 -= 1,
                    "L" => head.1 -= 1,
                    "R" => head.1 += 1,
                    _ => panic!("unknown command"),
                }
                let delta = (head.0-tail.0, head.1-tail.1);
                if delta.0.abs() > 1 || delta.1.abs() > 1 {
                    tail.0 += delta.0.signum();
                    tail.1 += delta.1.signum();
                }
                been.insert(tail);
            }
        });

    println!("been {:?}", been.len());
}
