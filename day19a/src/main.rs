#[derive(Debug, Clone)]
struct Blueprint {
    costs: Vec<Vec<usize>>,
}

#[derive(Debug, Clone)]
struct State {
    time: usize,
    prev_robots: Vec<usize>,
    robots: Vec<usize>, // ore clay obsidian geode
    goods: Vec<usize>,
    next: Vec<State>,
}

// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
// 0         1   2    3   4    5     6 7    8    9    10    11    12 13  14   15       16    17    18 19 20  21 22    23   24    25    26    27 28 29  30 31
fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|l| {
            let l = l.split_whitespace().collect::<Vec<&str>>();
            Blueprint {
                costs: vec![vec![l[6].parse::<usize>().unwrap(), 0, 0],
                           vec![l[12].parse::<usize>().unwrap(), 0, 0],
                           vec![l[18].parse::<usize>().unwrap(), l[21].parse::<usize>().unwrap(), 0],
                           vec![l[27].parse::<usize>().unwrap(), 0, l[30].parse::<usize>().unwrap()]],
            }
        })
        .collect::<Vec<Blueprint>>();

    println!("input: {:#?}", input);

    fn fwd(level: usize, b: &Blueprint, s: &State) -> State {
        let mut retvec: Vec<State> = Vec::new();

//        println!("fwd lvl {} {:?}", s.time, s);

        let mut ns = s.clone();
        if ns.time < 24 {
            let goods = ns.goods.clone();
            ns.goods[0] += s.robots[0];
            ns.goods[1] += s.robots[1];
            ns.goods[2] += s.robots[2];
            ns.goods[3] += s.robots[3];
            ns.time += 1;
            ns.prev_robots = ns.robots.clone();
            retvec.push(fwd(level+1, b, &ns));

//            for r in 0..2 {
            for r in 0..4 {
                if (goods[0] >= b.costs[r][0] && goods[1] >= b.costs[r][1] && goods[2] >= b.costs[r][2]) &&
                   !(goods[0] - s.prev_robots[0] >= b.costs[r][0] && goods[1] - s.prev_robots[1] >= b.costs[r][1] && goods[2] - s.prev_robots[2] >= b.costs[r][2]) {
                    let mut ns = ns.clone();
                    ns.goods[0] -= b.costs[r][0];
                    ns.goods[1] -= b.costs[r][1];
                    ns.goods[2] -= b.costs[r][2];
                    ns.prev_robots = ns.robots.clone();
                    ns.robots[r] += 1;
                    retvec.push(fwd(level + 1, b, &ns));
                }
            }
        }

        if retvec.len() == 1 {
            retvec.pop().unwrap()
        } else {
            ns.next = retvec;
            ns
        }
    }

    fn best(level: usize, s: &State) -> usize {
        if s.next.len() == 0 {
            s.goods[3]
        } else {
            s.next.iter().map(|v| best(level + 1, v)).max().unwrap_or(0)
        }
    }

    for b in input.iter() {
        // ore clay obsidian geode
        //  Each ore robot costs 4 ore.
        //  Each clay robot costs 2 ore.
        //  Each obsidian robot costs 3 ore and 14 clay.
        //  Each geode robot costs 2 ore and 7 obsidian.
        let need_obs = b.costs[3][2];
        let need_clay = b.costs[2][1] + need_obs * b.costs[2][1];
        let need_ore = b.costs[3][0] + need_obs * b.costs[2][0] + need_clay * b.costs[1][0];
        println!("ore {} clay {} obs {}", need_ore, need_clay, need_obs);
    }
    let mut total: usize = 0;
    for (i,b) in input.iter().enumerate() {
        let init: State = State {
            time: 0,
            prev_robots: vec![1, 0, 0, 0],
            robots: vec![1, 0, 0, 0],
            goods: vec![0, 0, 0, 0],
            next: Vec::new()
        };
        println!("bp");
        let res = fwd(0, &b, &init);
        let b = best(0, &res);
        total += b * (i+1);
        println!("b: {}", b);
//        println!("res: {:#?}", res);
    }
    println!("total {}", total);

}
