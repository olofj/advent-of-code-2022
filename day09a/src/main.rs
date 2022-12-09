use std::collections::HashSet;

fn main() {
    let mut head: (isize, isize) = (0, 0);
    let mut tail: (isize, isize) = (0, 0);
    let mut been: HashSet<(isize, isize)> = HashSet::new();
    been.insert(tail);

    let input = include_str!("input.txt")
        .lines()
        .map(|l| {
            let cmd = l.split_once(" ").unwrap();
            let dist = cmd.1.parse::<isize>().unwrap();
            (cmd.0, dist)
        })
        .collect::<Vec<(&str, isize)>>();

    let onedist = (|(a1, a2), (b1, b2)| {
        let a: isize = a1 - b1;
        let b: isize = a2 - b2;
        a.abs() <= 1 && b.abs() <= 1
    });

    input.iter().for_each(|(cmd, dist)| {
        for _ in 0..*dist {
            match *cmd {
                "U" => {
                    head.0 += 1;
                    if !onedist(head, tail) {
                        tail.0 = head.0 - 1;
                        tail.1 = head.1;
                    }
                }
                "D" => {
                    head.0 -= 1;
                    if !onedist(head, tail) {
                        tail.0 = head.0 + 1;
                        tail.1 = head.1;
                    }
                }
                "L" => {
                    head.1 -= 1;
                    if !onedist(head, tail) {
                        tail.0 = head.0;
                        tail.1 = head.1 + 1;
                    }
                }
                "R" => {
                    head.1 += 1;
                    if !onedist(head, tail) {
                        tail.0 = head.0;
                        tail.1 = head.1 - 1;
                    }
                }
                _ => {}
            };
            been.insert(tail);
            println!("H {:?} T {:?}", head, tail);
        }
    });

    println!("input was {:?}", input);
    println!("Locations: {:?}", been);
    println!("# Locations: {:?}", been.len());
}
