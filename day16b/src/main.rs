use std::collections::{HashMap, HashSet, VecDeque};

//Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
#[derive(Debug, Clone)]
struct Visit {
    cur: usize,
    release: usize,
    next: Vec<Visit>,
    bestrem: usize,
}

const MAXTIME: usize = 26;

fn main() {
    let mut valvemap: HashMap<String, usize> = HashMap::new();
    let input = include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let l = l.strip_prefix("Valve ").unwrap();
            let (valve, l) = l.split_once(" has flow rate=").unwrap();
            valvemap.insert(valve.to_string(), i);
            let (rate, l) = l
                .split_once("; tunnel leads to valve ")
                .or_else(|| l.split_once("; tunnels lead to valves "))
                .unwrap();
            let rate = rate.parse::<usize>().unwrap();
            let conn = l.split(", ").collect::<Vec<&str>>();
            (i, (rate, conn))
        })
        .collect::<Vec<(usize, (usize, Vec<&str>))>>()
        .into_iter()
        .map(|(a, (b, c))| {
            (
                a,
                (
                    b,
                    c.into_iter()
                        .map(|c| *valvemap.get(c).unwrap())
                        .collect::<Vec<usize>>(),
                ),
            )
        })
        .collect::<HashMap<usize, (usize, Vec<usize>)>>();

    for (valve, (rate, next)) in input.iter() {
        println!(" valve {} rate {} next {:?}", valve, rate, next);
    }

    let nodes = valvemap.len();
    let connections = (0..nodes)
        .map(|n| input[&n].1.clone())
        .collect::<Vec<Vec<usize>>>();

    let mut distances: Vec<Vec<usize>> = vec![vec![]; nodes];
    for node in 0..nodes {
        let mut q: VecDeque<usize> = VecDeque::new();
        let mut dist: Vec<isize> = vec![-1; nodes];
        dist[node] = 0;
        q.push_back(node);
        while let Some(n) = q.pop_front() {
            let d = dist[n];
            for conn in connections[n].iter() {
                if dist[*conn] == -1 || dist[*conn] > d + 1 {
                    dist[*conn] = d + 1;
                    q.push_back(*conn);
                }
            }
        }
        distances[node] = dist.iter().map(|i| *i as usize).collect::<Vec<usize>>();
    }
    for i in 0..distances.len() {
        println!("{} : {:?}", i, distances[i]);
    }

    fn fwd(
        level: usize,
        cur: usize,
        timeleft: usize,
        mut rem: HashSet<usize>,
        mut allrem: Option<HashSet<usize>>,
        input: &HashMap<usize, (usize, Vec<usize>)>,
        distances: &Vec<Vec<usize>>,
        start: usize,
    ) -> Visit {
        rem.remove(&cur);
        if allrem != None {
            allrem.as_mut().unwrap().remove(&cur);
        }

        let ret = rem
            .iter()
            .filter(|r| distances[cur][**r] + 1 < timeleft)
            .map(|r| {
                let time: usize = distances[cur][*r] + 1;
                let nextallrem = if allrem != None { Some(allrem.as_ref().unwrap().clone()) } else { None };
                fwd(
                    level + 1,
                    *r,
                    timeleft - time,
                    rem.clone(),
                    nextallrem,
                    input,
                    distances,
                    start,
                )
            })
            .collect::<Vec<Visit>>();
            /*
        println!(
            "{}fwd cur {:?} timeleft {:?} ret {:#?}",
            String::from_iter(vec![" | "; level]),
            cur,
            timeleft,
            ret
        );
        */
        let bestrem = if allrem != None {
            let remopts = fwd(0, start, MAXTIME, allrem.unwrap().clone(), None, &input, &distances, start);
            best(0, &remopts)
        } else {
            0
        };
        Visit {
            cur: cur,
            release: timeleft * input[&cur].0,
            next: ret,
            bestrem: bestrem,
        }
    }

    fn best(level: usize, v: &Visit) -> usize {
        let max = v.next.iter().map(|v| best(level + 1, v)).max().unwrap_or(0);
        /*
        println!(
            "{}best {:?}: {:?} {:?}",
            String::from_iter(vec![" | "; level]),
            v.cur,
            v.release,
            v.bestrem
        );
        */
        if v.bestrem < max {
            v.release + max
        } else {
            v.release + v.bestrem
        }
    }

    let rem: HashSet<usize> = (0..nodes)
        .filter(|n| input[n].0 > 0)
        .collect::<HashSet<usize>>();
    let allrem = rem.clone();
    let start: usize = *valvemap.get("AA").unwrap();

    let new: Visit = fwd(0, start, MAXTIME, rem, Some(allrem), &input, &distances, start);

    let b = best(0, &new);
    println!("best: {}", b);
    //    println!("{} by visiting: {} then {:#?}", new.release, new.cur, new.next);
    //    println!("best: {}", best(0, &new));
}
