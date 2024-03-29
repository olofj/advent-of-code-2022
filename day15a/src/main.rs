use std::collections::HashSet;

fn distance((sx, sy): (isize,isize), (bx, by): (isize,isize)) -> isize {
    (sx.abs_diff(bx) + sy.abs_diff(by)) as isize
}

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|l| l.split_once(":").unwrap())
        .map(|(s, b)| {
            (
                s.strip_prefix("Sensor at x=").unwrap(),
                b.strip_prefix(" closest beacon is at x=").unwrap(),
            )
        })
        .map(|(s, b)| (s.split_once(", y=").unwrap(), b.split_once(", y=").unwrap()))
        .map(|((sx, sy), (bx, by))| {
            (
                (sx.parse::<isize>().unwrap(), sy.parse::<isize>().unwrap()),
                (bx.parse::<isize>().unwrap(), by.parse::<isize>().unwrap()),
            )
        })
        .map(|(s, b) | (s,distance(s,b)))
        .collect::<Vec<((isize, isize), isize)>>();

    // println!("input: {:?}", input);
    let y = 2000000;
    let allranges = input.iter().map(|((sx,sy),d)| {
        let dist = sy.abs_diff(y) as isize;
        let mut hs:Vec<isize> = vec![];
        if dist <= *d {
            // println!("range: {:?}..{:?} (d {} dist {})", sx-(d-dist), sx+(d-dist), d, dist);
            hs = (sx-(d-dist)..sx+(d-dist)).collect::<Vec<isize>>();
        }
        hs
    })
    .flatten()
    .collect::<HashSet<isize>>();

    println!("count: {}", allranges.len());

//    println!("input: {:?}", input);
}
