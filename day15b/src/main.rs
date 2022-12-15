fn distance((sx, sy): (isize, isize), (bx, by): (isize, isize)) -> isize {
    (sx.abs_diff(bx) + sy.abs_diff(by)) as isize
}

const SZ: isize = 4000000;

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
        .map(|(s, b)| (s, distance(s, b)))
        .collect::<Vec<((isize, isize), isize)>>();

    let mut ranges: Vec<(isize, Vec<(isize, isize)>)> = Vec::new();

    for y in 0..SZ {
        let mut newrange = input
            .iter()
            .filter_map(|((sx, sy), d)| {
                let dist = sy.abs_diff(y) as isize;
                if dist <= *d {
                    /*
                    println!(
                        "range: {:?}..{:?} (d {} dist {})",
                        sx - (d - dist),
                        sx + (d - dist),
                        d,
                        dist
                    );
                    */
                    let start: isize = 0.max(sx - (d - dist));
                    let end: isize = SZ.min(sx + (d - dist));
                    Some((start, end))
                } else {
                    None
                }
            })
            .collect::<Vec<(isize, isize)>>();

        newrange.sort_by(|a, b| a.0.cmp(&b.0));

        // println!("range: {:?}", newrange);

        let pruned = newrange
            .iter()
            .fold(Vec::<(isize, isize)>::new(), |mut acc, (s, e)| {
                if let Some(last) = acc.last_mut() {
                    if last.1 >= *s - 1 {
                        last.1 = last.1.max(*e)
                    } else {
                        acc.push((*s, *e))
                    }
                } else {
                    acc.push((*s, *e))
                }
                acc
            });

        // println!("pruned: {:?}", pruned);
        let mut inverse: Vec<(isize, isize)> = Vec::new();
        if pruned.len() > 1 {
            let mut last_end = -1;
            for p in pruned {
                if last_end + 1 < p.0 {
                    inverse.push((last_end + 1, p.0 - 1));
                }
                last_end = p.1;
            }
            // println!("inverse: {:?}", inverse);
            ranges.push((y, inverse));
        }
    }

    if ranges.len() != 1 {
        panic!(
            "I should only get a single location for a possible beacong. Got: {:?}",
            ranges
        );
    }
    let (x, y) = (ranges[0].1[0].0, ranges[0].0);

    println!("Location: {} {}", x, y);

    let tunefreq = x * 4000000 + y;

    println!("tunefreq: {}", tunefreq);
}
