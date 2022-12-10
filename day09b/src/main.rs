use std::collections::HashSet;

fn main() {
    let mut head: (isize, isize) = (0, 0);
    let mut tail: Vec<(isize, isize)> = vec![(0, 0); 10];
    let mut been: Vec<HashSet<(isize, isize)>> = (0..10)
        .map(|_| HashSet::new())
        .collect::<Vec<HashSet<(isize, isize)>>>();

    let input = include_str!("input.txt")
        .lines()
        .map(|l| {
            let cmd = l.split_once(" ").unwrap();
            let dist = cmd.1.parse::<isize>().unwrap();
            (cmd.0, dist)
        })
        .collect::<Vec<(&str, isize)>>();

    let onedist = |(a1, a2), (b1, b2)| {
        let a: isize = a1 - b1;
        let b: isize = a2 - b2;
        a.abs() <= 1 && b.abs() <= 1
    };

    head = (0, 0);
    been[0].insert((0, 0));
    for (cmd, dist) in input {
        println!("Move: {} {}", cmd, dist);
        for _ in 0..dist {
            for c in cmd.chars() {
                match c {
                    'U' => head.0 += 1,
                    'D' => head.0 -= 1,
                    'L' => head.1 -= 1,
                    'R' => head.1 += 1,
                    _ => panic!("unknown command"),
                }
            }
            let mut p = head;
            for i in 0..10 {
                let t = tail[i];
                if !onedist(p, t) {
                    let delta = if t.0 == p.0 {
                        if t.1 < p.1 {
                            (0, 1)
                        } else {
                            (0, -1)
                        }
                    } else if t.1 == p.1 {
                        if t.0 < p.0 {
                            (1, 0)
                        } else {
                            (-1, 0)
                        }
                    } else {
                        match (p.0 < t.0, p.1 < t.1) {
                            (true, true) => (-1, -1),
                            (true, false) => (-1, 1),
                            (false, true) => (1, -1),
                            (false, false) => (1, 1),
                        }
                    };
                    tail[i].0 = t.0 + delta.0;
                    tail[i].1 = t.1 + delta.1;
                }
                p = tail[i];
                been[i].insert(tail[i]);
            }
        }
    }

    for r in -20..20 {
        let mut line: String = "".to_owned();
        for c in -20..20 {
            let mut fill = '.';
            for i in 0..10 {
                if been[i].contains(&(r, c)) {
                    fill = ('0'..='9').collect::<Vec<char>>()[i];
                    break;
                }
            }
            line.push(fill);
        }
        println!("{}", line);
    }

    for i in 0..10 {
        println!("been[{}]: {}", i, been[i].len());
    }

    //    println!("input was {:?}", input);
    //    println!("newmoves {:?}", newmoves);
    //    println!("Locations: {:?}", been);
    //    println!("# Locations: {:?}", been.len());
}
