use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Blueprint {
    costs: Vec<Vec<usize>>,
    ore_per_good: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    time: usize,
    robots: Vec<usize>, // ore clay obsidian geode
    goods: Vec<usize>,
}

const MAXTIME: usize = 32;
const HASHFROM: usize = MAXTIME-14;

fn weight(goods: &Vec<usize>, b: &Blueprint) -> usize {
    // ore clay obsidian geode
    //  Each ore robot costs 4 ore.
    //  Each clay robot costs 2 ore.
    //  Each obsidian robot costs 3 ore and 14 clay.
    //  Each geode robot costs 2 ore and 7 obsidian.
    let mut ore = goods[0];
    let mut clay = goods[1];
    let mut obs = goods[2];
    let geode = goods[3];
    ore += geode * b.costs[3][0];
    ore += obs * b.costs[2][0];
    obs += geode * b.costs[3][2];
    clay += obs * b.costs[2][1];
    ore += clay * b.costs[1][0];
    ore
}

// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
// 0         1   2    3   4    5     6 7    8    9    10    11    12 13  14   15       16    17    18 19 20  21 22    23   24    25    26    27 28 29  30 31
fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|l| {
            let l = l.split_whitespace().collect::<Vec<&str>>();
            Blueprint {
                costs: vec![
                    vec![l[6].parse::<usize>().unwrap(), 0, 0],
                    vec![l[12].parse::<usize>().unwrap(), 0, 0],
                    vec![
                        l[18].parse::<usize>().unwrap(),
                        l[21].parse::<usize>().unwrap(),
                        0,
                    ],
                    vec![
                        l[27].parse::<usize>().unwrap(),
                        0,
                        l[30].parse::<usize>().unwrap(),
                    ],
                ],
                ore_per_good: vec![0; 4],
            }
        })
        .map(|b| Blueprint {
            costs: b.costs.clone(),
            ore_per_good: vec![
                weight(&vec![1, 0, 0, 0], &b),
                weight(&vec![0, 1, 0, 0], &b),
                weight(&vec![0, 0, 1, 0], &b),
                weight(&vec![0, 0, 0, 1], &b),
            ],
        })
        .collect::<Vec<Blueprint>>();

    //    println!("input: {:#?}", input);

    fn fwd_best(level: usize, b: &Blueprint, s: &State, cache: &mut HashMap<(usize,usize,usize), usize>, hits: &mut usize) -> usize {
        let mut retvec: Vec<usize> = Vec::new();

        //        println!("fwd lvl {} {:?} max: {:?}", s.time, s, max);

        let mut ns = s.clone();
        if ns.time < MAXTIME {
            let goods = ns.goods.clone();
            ns.goods[0] += s.robots[0];
            ns.goods[1] += s.robots[1];
            ns.goods[2] += s.robots[2];
            ns.goods[3] += s.robots[3];
            ns.time += 1;
            for r in [3, 2, 1, 0] {
                if (goods[0] >= b.costs[r][0]
                    && goods[1] >= b.costs[r][1]
                    && goods[2] >= b.costs[r][2]) {
                    let mut ns = ns.clone();
                    ns.goods[0] -= b.costs[r][0];
                    ns.goods[1] -= b.costs[r][1];
                    ns.goods[2] -= b.costs[r][2];
                    ns.robots[r] += 1;
                    if ns.time >= HASHFROM {
                        let key = (ns.time,weight(&ns.goods,b),weight(&ns.robots,b));
//                        let key = (ns.time,(ns.goods[0],ns.goods[1],ns.goods[2],ns.goods[3]),(ns.robots[0],ns.robots[1],ns.robots[2],ns.robots[3]));
                        if let Some(m) = cache.get(&key) {
                            *hits += 1;
                            retvec.push(*m);
                        } else {
                            let m = fwd_best(level + 1, b, &ns, cache, hits);
                            cache.insert(key, m);
                            retvec.push(m);
                        }
                    } else {
                            retvec.push(fwd_best(level + 1, b, &ns, cache, hits));
                    }
                }
            }
            if ns.time >= HASHFROM {
                let key = (ns.time,weight(&ns.goods,b),weight(&ns.robots,b));
//                let key = (ns.time,(ns.goods[0],ns.goods[1],ns.goods[2],ns.goods[3]),(ns.robots[0],ns.robots[1],ns.robots[2],ns.robots[3]));
                if let Some(m) = cache.get(&key) {
                    retvec.push(*m);
                } else {
                    let m = fwd_best(level + 1, b, &ns, cache, hits);
                    cache.insert(key, m);
                    retvec.push(m);
                }
            } else {
                retvec.push(fwd_best(level + 1, b, &ns, cache, hits));
            }
        } else {
            return s.goods[3];
        }

        if retvec.len() == 1 {
            retvec.pop().unwrap()
        } else {
            retvec.into_iter().max().unwrap_or(0)
        }
    }

    let mut total: usize = 1;
    for (i, b) in input.iter().take(3).enumerate() {
        let mut cache: HashMap<(usize,usize,usize), usize> = HashMap::new();
        let mut hits: usize = 0;

        let mut init: State = State {
            time: 0,
            robots: vec![1, 0, 0, 0],
            goods: vec![0, 0, 0, 0],
        };
        println!("bp {}", i + 1);
        let b = fwd_best(0, &b, &mut init, &mut cache, &mut hits);
        println!("b: {}", b);
        println!("Cache size: {} hits {}", cache.len(), hits);
        total *= b;
        //        println!("res: {:#?}", res);
    }
    println!("total {}", total);
}
